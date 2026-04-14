
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
#ifndef __windows2Eui_h__
#define __windows2Eui_h__
#ifndef __windows2Eui_p_h__
#define __windows2Eui_p_h__


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

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CIColorHelper_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIColorHelper_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            interface IColorHelper;
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CIColorHelper ABI::Windows::UI::IColorHelper

#endif // ____x_ABI_CWindows_CUI_CIColorHelper_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIColorHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIColorHelperStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            interface IColorHelperStatics;
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CIColorHelperStatics ABI::Windows::UI::IColorHelperStatics

#endif // ____x_ABI_CWindows_CUI_CIColorHelperStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIColorHelperStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIColorHelperStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            interface IColorHelperStatics2;
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CIColorHelperStatics2 ABI::Windows::UI::IColorHelperStatics2

#endif // ____x_ABI_CWindows_CUI_CIColorHelperStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIColors_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIColors_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            interface IColors;
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CIColors ABI::Windows::UI::IColors

#endif // ____x_ABI_CWindows_CUI_CIColors_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIColorsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIColorsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            interface IColorsStatics;
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CIColorsStatics ABI::Windows::UI::IColorsStatics

#endif // ____x_ABI_CWindows_CUI_CIColorsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIUIContentRoot_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIUIContentRoot_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            interface IUIContentRoot;
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CIUIContentRoot ABI::Windows::UI::IUIContentRoot

#endif // ____x_ABI_CWindows_CUI_CIUIContentRoot_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            interface IUIContext;
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CIUIContext ABI::Windows::UI::IUIContext

#endif // ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
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
            class UIContext;
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Color
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            struct Color
            {
                BYTE A;
                BYTE R;
                BYTE G;
                BYTE B;
            };
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.WindowId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
namespace ABI {
    namespace Windows {
        namespace UI {
            struct WindowId
            {
                UINT64 Value;
            };
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.UI.IColorHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ColorHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CIColorHelper_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CIColorHelper_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_IColorHelper[] = L"Windows.UI.IColorHelper";
namespace ABI {
    namespace Windows {
        namespace UI {
            MIDL_INTERFACE("193cfbe7-65c7-4540-ad08-6283ba76879a")
            IColorHelper : public IInspectable
            {
            public:
            };

            MIDL_CONST_ID IID& IID_IColorHelper = __uuidof(IColorHelper);
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CIColorHelper;
#endif /* !defined(____x_ABI_CWindows_CUI_CIColorHelper_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.IColorHelperStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ColorHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CIColorHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CIColorHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_IColorHelperStatics[] = L"Windows.UI.IColorHelperStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            MIDL_INTERFACE("8504dbea-fb6a-4144-a6c2-33499c9284f5")
            IColorHelperStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE FromArgb(
                    BYTE a,
                    BYTE r,
                    BYTE g,
                    BYTE b,
                    ABI::Windows::UI::Color* returnValue
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IColorHelperStatics = __uuidof(IColorHelperStatics);
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CIColorHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CIColorHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.IColorHelperStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.ColorHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CIColorHelperStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CIColorHelperStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_IColorHelperStatics2[] = L"Windows.UI.IColorHelperStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            MIDL_INTERFACE("24d9af02-6eb0-4b94-855c-fcf0818d9a16")
            IColorHelperStatics2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE ToDisplayName(
                    ABI::Windows::UI::Color color,
                    HSTRING* returnValue
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IColorHelperStatics2 = __uuidof(IColorHelperStatics2);
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CIColorHelperStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CIColorHelperStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.IColors
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Colors
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CIColors_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CIColors_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_IColors[] = L"Windows.UI.IColors";
namespace ABI {
    namespace Windows {
        namespace UI {
            MIDL_INTERFACE("9b8c9326-4ca6-4ce5-8994-9eff65cabdcc")
            IColors : public IInspectable
            {
            public:
            };

            MIDL_CONST_ID IID& IID_IColors = __uuidof(IColors);
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CIColors;
#endif /* !defined(____x_ABI_CWindows_CUI_CIColors_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.IColorsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Colors
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CIColorsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CIColorsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_IColorsStatics[] = L"Windows.UI.IColorsStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            MIDL_INTERFACE("cff52e04-cca6-4614-a17e-754910c84a99")
            IColorsStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_AliceBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_AntiqueWhite(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Aqua(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Aquamarine(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Azure(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Beige(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Bisque(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Black(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BlanchedAlmond(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Blue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BlueViolet(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Brown(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_BurlyWood(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CadetBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Chartreuse(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Chocolate(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Coral(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_CornflowerBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Cornsilk(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Crimson(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Cyan(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkCyan(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkGoldenrod(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkGray(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkGreen(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkKhaki(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkMagenta(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkOliveGreen(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkOrange(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkOrchid(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkRed(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkSalmon(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkSeaGreen(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkSlateBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkSlateGray(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkTurquoise(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DarkViolet(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DeepPink(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DeepSkyBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DimGray(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DodgerBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Firebrick(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FloralWhite(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ForestGreen(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Fuchsia(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Gainsboro(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_GhostWhite(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Gold(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Goldenrod(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Gray(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Green(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_GreenYellow(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Honeydew(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_HotPink(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IndianRed(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Indigo(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Ivory(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Khaki(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Lavender(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LavenderBlush(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LawnGreen(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LemonChiffon(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LightBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LightCoral(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LightCyan(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LightGoldenrodYellow(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LightGreen(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LightGray(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LightPink(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LightSalmon(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LightSeaGreen(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LightSkyBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LightSlateGray(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LightSteelBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LightYellow(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Lime(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LimeGreen(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Linen(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Magenta(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Maroon(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MediumAquamarine(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MediumBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MediumOrchid(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MediumPurple(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MediumSeaGreen(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MediumSlateBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MediumSpringGreen(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MediumTurquoise(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MediumVioletRed(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MidnightBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MintCream(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MistyRose(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Moccasin(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NavajoWhite(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Navy(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_OldLace(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Olive(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_OliveDrab(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Orange(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_OrangeRed(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Orchid(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PaleGoldenrod(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PaleGreen(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PaleTurquoise(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PaleVioletRed(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PapayaWhip(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PeachPuff(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Peru(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Pink(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Plum(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PowderBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Purple(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Red(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RosyBrown(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RoyalBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SaddleBrown(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Salmon(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SandyBrown(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SeaGreen(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SeaShell(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Sienna(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Silver(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SkyBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SlateBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SlateGray(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Snow(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SpringGreen(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SteelBlue(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Tan(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Teal(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Thistle(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Tomato(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Transparent(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Turquoise(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Violet(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Wheat(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_White(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_WhiteSmoke(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Yellow(
                    ABI::Windows::UI::Color* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_YellowGreen(
                    ABI::Windows::UI::Color* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IColorsStatics = __uuidof(IColorsStatics);
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CIColorsStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CIColorsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.IUIContentRoot
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIContentRoot
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CIUIContentRoot_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CIUIContentRoot_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_IUIContentRoot[] = L"Windows.UI.IUIContentRoot";
namespace ABI {
    namespace Windows {
        namespace UI {
            MIDL_INTERFACE("1dfcbac6-b36b-5cb9-9bc5-2b7a0eddc378")
            IUIContentRoot : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_UIContext(
                    ABI::Windows::UI::IUIContext** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IUIContentRoot = __uuidof(IUIContentRoot);
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CIUIContentRoot;
#endif /* !defined(____x_ABI_CWindows_CUI_CIUIContentRoot_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.IUIContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CIUIContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CIUIContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_IUIContext[] = L"Windows.UI.IUIContext";
namespace ABI {
    namespace Windows {
        namespace UI {
            MIDL_INTERFACE("bb5cfacd-5bd8-59d0-a59e-1c17a4d6d243")
            IUIContext : public IInspectable
            {
            public:
            };

            MIDL_CONST_ID IID& IID_IUIContext = __uuidof(IUIContext);
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CIUIContext;
#endif /* !defined(____x_ABI_CWindows_CUI_CIUIContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.ColorHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.IColorHelperStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.IColorHelperStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.IColorHelper ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ColorHelper_DEFINED
#define RUNTIMECLASS_Windows_UI_ColorHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ColorHelper[] = L"Windows.UI.ColorHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Colors
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.IColorsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.IColors ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Colors_DEFINED
#define RUNTIMECLASS_Windows_UI_Colors_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Colors[] = L"Windows.UI.Colors";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.UIContentRoot
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.IUIContentRoot ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_UIContentRoot_DEFINED
#define RUNTIMECLASS_Windows_UI_UIContentRoot_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIContentRoot[] = L"Windows.UI.UIContentRoot";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.UIContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.IUIContext ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_UIContext_DEFINED
#define RUNTIMECLASS_Windows_UI_UIContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIContext[] = L"Windows.UI.UIContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CIColorHelper_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIColorHelper_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CIColorHelper __x_ABI_CWindows_CUI_CIColorHelper;

#endif // ____x_ABI_CWindows_CUI_CIColorHelper_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIColorHelperStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIColorHelperStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CIColorHelperStatics __x_ABI_CWindows_CUI_CIColorHelperStatics;

#endif // ____x_ABI_CWindows_CUI_CIColorHelperStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIColorHelperStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIColorHelperStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CIColorHelperStatics2 __x_ABI_CWindows_CUI_CIColorHelperStatics2;

#endif // ____x_ABI_CWindows_CUI_CIColorHelperStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIColors_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIColors_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CIColors __x_ABI_CWindows_CUI_CIColors;

#endif // ____x_ABI_CWindows_CUI_CIColors_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIColorsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIColorsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CIColorsStatics __x_ABI_CWindows_CUI_CIColorsStatics;

#endif // ____x_ABI_CWindows_CUI_CIColorsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIUIContentRoot_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIUIContentRoot_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CIUIContentRoot __x_ABI_CWindows_CUI_CIUIContentRoot;

#endif // ____x_ABI_CWindows_CUI_CIUIContentRoot_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CIUIContext __x_ABI_CWindows_CUI_CIUIContext;

#endif // ____x_ABI_CWindows_CUI_CIUIContext_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

/*
 *
 * Struct Windows.UI.Color
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CUI_CColor
{
    BYTE A;
    BYTE R;
    BYTE G;
    BYTE B;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.WindowId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
struct __x_ABI_CWindows_CUI_CWindowId
{
    UINT64 Value;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.UI.IColorHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ColorHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CIColorHelper_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CIColorHelper_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_IColorHelper[] = L"Windows.UI.IColorHelper";
typedef struct __x_ABI_CWindows_CUI_CIColorHelperVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CIColorHelper* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CIColorHelper* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CIColorHelper* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CIColorHelper* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CIColorHelper* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CIColorHelper* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CIColorHelperVtbl;

interface __x_ABI_CWindows_CUI_CIColorHelper
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CIColorHelperVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CIColorHelper_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CIColorHelper_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CIColorHelper_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CIColorHelper_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CIColorHelper_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CIColorHelper_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CIColorHelper;
#endif /* !defined(____x_ABI_CWindows_CUI_CIColorHelper_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.IColorHelperStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.ColorHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CIColorHelperStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CIColorHelperStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_IColorHelperStatics[] = L"Windows.UI.IColorHelperStatics";
typedef struct __x_ABI_CWindows_CUI_CIColorHelperStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CIColorHelperStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CIColorHelperStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CIColorHelperStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CIColorHelperStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CIColorHelperStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CIColorHelperStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromArgb)(__x_ABI_CWindows_CUI_CIColorHelperStatics* This,
        BYTE a,
        BYTE r,
        BYTE g,
        BYTE b,
        struct __x_ABI_CWindows_CUI_CColor* returnValue);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CIColorHelperStaticsVtbl;

interface __x_ABI_CWindows_CUI_CIColorHelperStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CIColorHelperStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CIColorHelperStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CIColorHelperStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CIColorHelperStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CIColorHelperStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CIColorHelperStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CIColorHelperStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CIColorHelperStatics_FromArgb(This, a, r, g, b, returnValue) \
    ((This)->lpVtbl->FromArgb(This, a, r, g, b, returnValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CIColorHelperStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CIColorHelperStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.IColorHelperStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.ColorHelper
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CIColorHelperStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CIColorHelperStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_IColorHelperStatics2[] = L"Windows.UI.IColorHelperStatics2";
typedef struct __x_ABI_CWindows_CUI_CIColorHelperStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CIColorHelperStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CIColorHelperStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CIColorHelperStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CIColorHelperStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CIColorHelperStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CIColorHelperStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ToDisplayName)(__x_ABI_CWindows_CUI_CIColorHelperStatics2* This,
        struct __x_ABI_CWindows_CUI_CColor color,
        HSTRING* returnValue);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CIColorHelperStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CIColorHelperStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CIColorHelperStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CIColorHelperStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CIColorHelperStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CIColorHelperStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CIColorHelperStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CIColorHelperStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CIColorHelperStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CIColorHelperStatics2_ToDisplayName(This, color, returnValue) \
    ((This)->lpVtbl->ToDisplayName(This, color, returnValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CIColorHelperStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CIColorHelperStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.IColors
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Colors
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CIColors_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CIColors_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_IColors[] = L"Windows.UI.IColors";
typedef struct __x_ABI_CWindows_CUI_CIColorsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CIColors* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CIColors* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CIColors* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CIColors* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CIColors* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CIColors* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CIColorsVtbl;

interface __x_ABI_CWindows_CUI_CIColors
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CIColorsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CIColors_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CIColors_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CIColors_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CIColors_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CIColors_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CIColors_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CIColors;
#endif /* !defined(____x_ABI_CWindows_CUI_CIColors_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.IColorsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Colors
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CIColorsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CIColorsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_IColorsStatics[] = L"Windows.UI.IColorsStatics";
typedef struct __x_ABI_CWindows_CUI_CIColorsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CIColorsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CIColorsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AliceBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_AntiqueWhite)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Aqua)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Aquamarine)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Azure)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Beige)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Bisque)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Black)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_BlanchedAlmond)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Blue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_BlueViolet)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Brown)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_BurlyWood)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_CadetBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Chartreuse)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Chocolate)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Coral)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_CornflowerBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Cornsilk)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Crimson)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Cyan)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkCyan)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkGoldenrod)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkGray)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkGreen)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkKhaki)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkMagenta)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkOliveGreen)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkOrange)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkOrchid)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkRed)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkSalmon)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkSeaGreen)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkSlateBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkSlateGray)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkTurquoise)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DarkViolet)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DeepPink)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DeepSkyBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DimGray)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DodgerBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Firebrick)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_FloralWhite)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_ForestGreen)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Fuchsia)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Gainsboro)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_GhostWhite)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Gold)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Goldenrod)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Gray)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Green)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_GreenYellow)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Honeydew)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_HotPink)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_IndianRed)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Indigo)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Ivory)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Khaki)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Lavender)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LavenderBlush)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LawnGreen)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LemonChiffon)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LightBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LightCoral)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LightCyan)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LightGoldenrodYellow)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LightGreen)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LightGray)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LightPink)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LightSalmon)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LightSeaGreen)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LightSkyBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LightSlateGray)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LightSteelBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LightYellow)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Lime)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_LimeGreen)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Linen)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Magenta)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Maroon)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_MediumAquamarine)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_MediumBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_MediumOrchid)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_MediumPurple)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_MediumSeaGreen)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_MediumSlateBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_MediumSpringGreen)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_MediumTurquoise)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_MediumVioletRed)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_MidnightBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_MintCream)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_MistyRose)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Moccasin)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_NavajoWhite)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Navy)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_OldLace)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Olive)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_OliveDrab)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Orange)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_OrangeRed)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Orchid)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_PaleGoldenrod)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_PaleGreen)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_PaleTurquoise)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_PaleVioletRed)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_PapayaWhip)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_PeachPuff)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Peru)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Pink)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Plum)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_PowderBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Purple)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Red)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_RosyBrown)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_RoyalBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_SaddleBrown)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Salmon)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_SandyBrown)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_SeaGreen)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_SeaShell)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Sienna)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Silver)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_SkyBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_SlateBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_SlateGray)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Snow)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_SpringGreen)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_SteelBlue)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Tan)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Teal)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Thistle)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Tomato)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Transparent)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Turquoise)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Violet)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Wheat)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_White)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_WhiteSmoke)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_Yellow)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_YellowGreen)(__x_ABI_CWindows_CUI_CIColorsStatics* This,
        struct __x_ABI_CWindows_CUI_CColor* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CIColorsStaticsVtbl;

interface __x_ABI_CWindows_CUI_CIColorsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CIColorsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CIColorsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CIColorsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CIColorsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CIColorsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CIColorsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CIColorsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_AliceBlue(This, value) \
    ((This)->lpVtbl->get_AliceBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_AntiqueWhite(This, value) \
    ((This)->lpVtbl->get_AntiqueWhite(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Aqua(This, value) \
    ((This)->lpVtbl->get_Aqua(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Aquamarine(This, value) \
    ((This)->lpVtbl->get_Aquamarine(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Azure(This, value) \
    ((This)->lpVtbl->get_Azure(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Beige(This, value) \
    ((This)->lpVtbl->get_Beige(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Bisque(This, value) \
    ((This)->lpVtbl->get_Bisque(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Black(This, value) \
    ((This)->lpVtbl->get_Black(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_BlanchedAlmond(This, value) \
    ((This)->lpVtbl->get_BlanchedAlmond(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Blue(This, value) \
    ((This)->lpVtbl->get_Blue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_BlueViolet(This, value) \
    ((This)->lpVtbl->get_BlueViolet(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Brown(This, value) \
    ((This)->lpVtbl->get_Brown(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_BurlyWood(This, value) \
    ((This)->lpVtbl->get_BurlyWood(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_CadetBlue(This, value) \
    ((This)->lpVtbl->get_CadetBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Chartreuse(This, value) \
    ((This)->lpVtbl->get_Chartreuse(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Chocolate(This, value) \
    ((This)->lpVtbl->get_Chocolate(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Coral(This, value) \
    ((This)->lpVtbl->get_Coral(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_CornflowerBlue(This, value) \
    ((This)->lpVtbl->get_CornflowerBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Cornsilk(This, value) \
    ((This)->lpVtbl->get_Cornsilk(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Crimson(This, value) \
    ((This)->lpVtbl->get_Crimson(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Cyan(This, value) \
    ((This)->lpVtbl->get_Cyan(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkBlue(This, value) \
    ((This)->lpVtbl->get_DarkBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkCyan(This, value) \
    ((This)->lpVtbl->get_DarkCyan(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkGoldenrod(This, value) \
    ((This)->lpVtbl->get_DarkGoldenrod(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkGray(This, value) \
    ((This)->lpVtbl->get_DarkGray(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkGreen(This, value) \
    ((This)->lpVtbl->get_DarkGreen(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkKhaki(This, value) \
    ((This)->lpVtbl->get_DarkKhaki(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkMagenta(This, value) \
    ((This)->lpVtbl->get_DarkMagenta(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkOliveGreen(This, value) \
    ((This)->lpVtbl->get_DarkOliveGreen(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkOrange(This, value) \
    ((This)->lpVtbl->get_DarkOrange(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkOrchid(This, value) \
    ((This)->lpVtbl->get_DarkOrchid(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkRed(This, value) \
    ((This)->lpVtbl->get_DarkRed(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkSalmon(This, value) \
    ((This)->lpVtbl->get_DarkSalmon(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkSeaGreen(This, value) \
    ((This)->lpVtbl->get_DarkSeaGreen(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkSlateBlue(This, value) \
    ((This)->lpVtbl->get_DarkSlateBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkSlateGray(This, value) \
    ((This)->lpVtbl->get_DarkSlateGray(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkTurquoise(This, value) \
    ((This)->lpVtbl->get_DarkTurquoise(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DarkViolet(This, value) \
    ((This)->lpVtbl->get_DarkViolet(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DeepPink(This, value) \
    ((This)->lpVtbl->get_DeepPink(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DeepSkyBlue(This, value) \
    ((This)->lpVtbl->get_DeepSkyBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DimGray(This, value) \
    ((This)->lpVtbl->get_DimGray(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_DodgerBlue(This, value) \
    ((This)->lpVtbl->get_DodgerBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Firebrick(This, value) \
    ((This)->lpVtbl->get_Firebrick(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_FloralWhite(This, value) \
    ((This)->lpVtbl->get_FloralWhite(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_ForestGreen(This, value) \
    ((This)->lpVtbl->get_ForestGreen(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Fuchsia(This, value) \
    ((This)->lpVtbl->get_Fuchsia(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Gainsboro(This, value) \
    ((This)->lpVtbl->get_Gainsboro(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_GhostWhite(This, value) \
    ((This)->lpVtbl->get_GhostWhite(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Gold(This, value) \
    ((This)->lpVtbl->get_Gold(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Goldenrod(This, value) \
    ((This)->lpVtbl->get_Goldenrod(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Gray(This, value) \
    ((This)->lpVtbl->get_Gray(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Green(This, value) \
    ((This)->lpVtbl->get_Green(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_GreenYellow(This, value) \
    ((This)->lpVtbl->get_GreenYellow(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Honeydew(This, value) \
    ((This)->lpVtbl->get_Honeydew(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_HotPink(This, value) \
    ((This)->lpVtbl->get_HotPink(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_IndianRed(This, value) \
    ((This)->lpVtbl->get_IndianRed(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Indigo(This, value) \
    ((This)->lpVtbl->get_Indigo(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Ivory(This, value) \
    ((This)->lpVtbl->get_Ivory(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Khaki(This, value) \
    ((This)->lpVtbl->get_Khaki(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Lavender(This, value) \
    ((This)->lpVtbl->get_Lavender(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LavenderBlush(This, value) \
    ((This)->lpVtbl->get_LavenderBlush(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LawnGreen(This, value) \
    ((This)->lpVtbl->get_LawnGreen(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LemonChiffon(This, value) \
    ((This)->lpVtbl->get_LemonChiffon(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LightBlue(This, value) \
    ((This)->lpVtbl->get_LightBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LightCoral(This, value) \
    ((This)->lpVtbl->get_LightCoral(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LightCyan(This, value) \
    ((This)->lpVtbl->get_LightCyan(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LightGoldenrodYellow(This, value) \
    ((This)->lpVtbl->get_LightGoldenrodYellow(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LightGreen(This, value) \
    ((This)->lpVtbl->get_LightGreen(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LightGray(This, value) \
    ((This)->lpVtbl->get_LightGray(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LightPink(This, value) \
    ((This)->lpVtbl->get_LightPink(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LightSalmon(This, value) \
    ((This)->lpVtbl->get_LightSalmon(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LightSeaGreen(This, value) \
    ((This)->lpVtbl->get_LightSeaGreen(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LightSkyBlue(This, value) \
    ((This)->lpVtbl->get_LightSkyBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LightSlateGray(This, value) \
    ((This)->lpVtbl->get_LightSlateGray(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LightSteelBlue(This, value) \
    ((This)->lpVtbl->get_LightSteelBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LightYellow(This, value) \
    ((This)->lpVtbl->get_LightYellow(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Lime(This, value) \
    ((This)->lpVtbl->get_Lime(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_LimeGreen(This, value) \
    ((This)->lpVtbl->get_LimeGreen(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Linen(This, value) \
    ((This)->lpVtbl->get_Linen(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Magenta(This, value) \
    ((This)->lpVtbl->get_Magenta(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Maroon(This, value) \
    ((This)->lpVtbl->get_Maroon(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_MediumAquamarine(This, value) \
    ((This)->lpVtbl->get_MediumAquamarine(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_MediumBlue(This, value) \
    ((This)->lpVtbl->get_MediumBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_MediumOrchid(This, value) \
    ((This)->lpVtbl->get_MediumOrchid(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_MediumPurple(This, value) \
    ((This)->lpVtbl->get_MediumPurple(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_MediumSeaGreen(This, value) \
    ((This)->lpVtbl->get_MediumSeaGreen(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_MediumSlateBlue(This, value) \
    ((This)->lpVtbl->get_MediumSlateBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_MediumSpringGreen(This, value) \
    ((This)->lpVtbl->get_MediumSpringGreen(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_MediumTurquoise(This, value) \
    ((This)->lpVtbl->get_MediumTurquoise(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_MediumVioletRed(This, value) \
    ((This)->lpVtbl->get_MediumVioletRed(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_MidnightBlue(This, value) \
    ((This)->lpVtbl->get_MidnightBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_MintCream(This, value) \
    ((This)->lpVtbl->get_MintCream(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_MistyRose(This, value) \
    ((This)->lpVtbl->get_MistyRose(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Moccasin(This, value) \
    ((This)->lpVtbl->get_Moccasin(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_NavajoWhite(This, value) \
    ((This)->lpVtbl->get_NavajoWhite(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Navy(This, value) \
    ((This)->lpVtbl->get_Navy(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_OldLace(This, value) \
    ((This)->lpVtbl->get_OldLace(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Olive(This, value) \
    ((This)->lpVtbl->get_Olive(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_OliveDrab(This, value) \
    ((This)->lpVtbl->get_OliveDrab(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Orange(This, value) \
    ((This)->lpVtbl->get_Orange(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_OrangeRed(This, value) \
    ((This)->lpVtbl->get_OrangeRed(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Orchid(This, value) \
    ((This)->lpVtbl->get_Orchid(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_PaleGoldenrod(This, value) \
    ((This)->lpVtbl->get_PaleGoldenrod(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_PaleGreen(This, value) \
    ((This)->lpVtbl->get_PaleGreen(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_PaleTurquoise(This, value) \
    ((This)->lpVtbl->get_PaleTurquoise(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_PaleVioletRed(This, value) \
    ((This)->lpVtbl->get_PaleVioletRed(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_PapayaWhip(This, value) \
    ((This)->lpVtbl->get_PapayaWhip(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_PeachPuff(This, value) \
    ((This)->lpVtbl->get_PeachPuff(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Peru(This, value) \
    ((This)->lpVtbl->get_Peru(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Pink(This, value) \
    ((This)->lpVtbl->get_Pink(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Plum(This, value) \
    ((This)->lpVtbl->get_Plum(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_PowderBlue(This, value) \
    ((This)->lpVtbl->get_PowderBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Purple(This, value) \
    ((This)->lpVtbl->get_Purple(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Red(This, value) \
    ((This)->lpVtbl->get_Red(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_RosyBrown(This, value) \
    ((This)->lpVtbl->get_RosyBrown(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_RoyalBlue(This, value) \
    ((This)->lpVtbl->get_RoyalBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_SaddleBrown(This, value) \
    ((This)->lpVtbl->get_SaddleBrown(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Salmon(This, value) \
    ((This)->lpVtbl->get_Salmon(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_SandyBrown(This, value) \
    ((This)->lpVtbl->get_SandyBrown(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_SeaGreen(This, value) \
    ((This)->lpVtbl->get_SeaGreen(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_SeaShell(This, value) \
    ((This)->lpVtbl->get_SeaShell(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Sienna(This, value) \
    ((This)->lpVtbl->get_Sienna(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Silver(This, value) \
    ((This)->lpVtbl->get_Silver(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_SkyBlue(This, value) \
    ((This)->lpVtbl->get_SkyBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_SlateBlue(This, value) \
    ((This)->lpVtbl->get_SlateBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_SlateGray(This, value) \
    ((This)->lpVtbl->get_SlateGray(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Snow(This, value) \
    ((This)->lpVtbl->get_Snow(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_SpringGreen(This, value) \
    ((This)->lpVtbl->get_SpringGreen(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_SteelBlue(This, value) \
    ((This)->lpVtbl->get_SteelBlue(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Tan(This, value) \
    ((This)->lpVtbl->get_Tan(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Teal(This, value) \
    ((This)->lpVtbl->get_Teal(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Thistle(This, value) \
    ((This)->lpVtbl->get_Thistle(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Tomato(This, value) \
    ((This)->lpVtbl->get_Tomato(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Transparent(This, value) \
    ((This)->lpVtbl->get_Transparent(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Turquoise(This, value) \
    ((This)->lpVtbl->get_Turquoise(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Violet(This, value) \
    ((This)->lpVtbl->get_Violet(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Wheat(This, value) \
    ((This)->lpVtbl->get_Wheat(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_White(This, value) \
    ((This)->lpVtbl->get_White(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_WhiteSmoke(This, value) \
    ((This)->lpVtbl->get_WhiteSmoke(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_Yellow(This, value) \
    ((This)->lpVtbl->get_Yellow(This, value))

#define __x_ABI_CWindows_CUI_CIColorsStatics_get_YellowGreen(This, value) \
    ((This)->lpVtbl->get_YellowGreen(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CIColorsStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CIColorsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.IUIContentRoot
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIContentRoot
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CIUIContentRoot_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CIUIContentRoot_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_IUIContentRoot[] = L"Windows.UI.IUIContentRoot";
typedef struct __x_ABI_CWindows_CUI_CIUIContentRootVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CIUIContentRoot* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CIUIContentRoot* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CIUIContentRoot* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CIUIContentRoot* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CIUIContentRoot* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CIUIContentRoot* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UIContext)(__x_ABI_CWindows_CUI_CIUIContentRoot* This,
        __x_ABI_CWindows_CUI_CIUIContext** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CIUIContentRootVtbl;

interface __x_ABI_CWindows_CUI_CIUIContentRoot
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CIUIContentRootVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CIUIContentRoot_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CIUIContentRoot_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CIUIContentRoot_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CIUIContentRoot_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CIUIContentRoot_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CIUIContentRoot_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CIUIContentRoot_get_UIContext(This, value) \
    ((This)->lpVtbl->get_UIContext(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CIUIContentRoot;
#endif /* !defined(____x_ABI_CWindows_CUI_CIUIContentRoot_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.IUIContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.UIContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CIUIContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CIUIContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_IUIContext[] = L"Windows.UI.IUIContext";
typedef struct __x_ABI_CWindows_CUI_CIUIContextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CIUIContext* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CIUIContext* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CIUIContext* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CIUIContext* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CIUIContext* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CIUIContext* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CIUIContextVtbl;

interface __x_ABI_CWindows_CUI_CIUIContext
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CIUIContextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CIUIContext_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CIUIContext_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CIUIContext_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CIUIContext_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CIUIContext_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CIUIContext_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CIUIContext;
#endif /* !defined(____x_ABI_CWindows_CUI_CIUIContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.ColorHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.IColorHelperStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.IColorHelperStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.IColorHelper ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_ColorHelper_DEFINED
#define RUNTIMECLASS_Windows_UI_ColorHelper_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_ColorHelper[] = L"Windows.UI.ColorHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Colors
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.IColorsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.IColors ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Colors_DEFINED
#define RUNTIMECLASS_Windows_UI_Colors_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Colors[] = L"Windows.UI.Colors";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.UIContentRoot
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.IUIContentRoot ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_UIContentRoot_DEFINED
#define RUNTIMECLASS_Windows_UI_UIContentRoot_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIContentRoot[] = L"Windows.UI.UIContentRoot";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Class Windows.UI.UIContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.IUIContext ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_UI_UIContext_DEFINED
#define RUNTIMECLASS_Windows_UI_UIContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_UIContext[] = L"Windows.UI.UIContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui_p_h__

#endif // __windows2Eui_h__
