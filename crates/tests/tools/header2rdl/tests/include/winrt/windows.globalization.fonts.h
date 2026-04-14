
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
#ifndef __windows2Eglobalization2Efonts_h__
#define __windows2Eglobalization2Efonts_h__
#ifndef __windows2Eglobalization2Efonts_p_h__
#define __windows2Eglobalization2Efonts_p_h__


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
#include "Windows.UI.Text.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace Fonts {
                interface ILanguageFont;
            } /* Fonts */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont ABI::Windows::Globalization::Fonts::ILanguageFont

#endif // ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace Fonts {
                interface ILanguageFontGroup;
            } /* Fonts */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup ABI::Windows::Globalization::Fonts::ILanguageFontGroup

#endif // ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace Fonts {
                interface ILanguageFontGroupFactory;
            } /* Fonts */
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory ABI::Windows::Globalization::Fonts::ILanguageFontGroupFactory

#endif // ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                typedef enum FontStretch : int FontStretch;
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                typedef enum FontStyle : int FontStyle;
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                typedef struct FontWeight FontWeight;
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace Fonts {
                class LanguageFont;
            } /* Fonts */
        } /* Globalization */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace Fonts {
                class LanguageFontGroup;
            } /* Fonts */
        } /* Globalization */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.Globalization.Fonts.ILanguageFont
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Fonts.LanguageFont
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_Fonts_ILanguageFont[] = L"Windows.Globalization.Fonts.ILanguageFont";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace Fonts {
                MIDL_INTERFACE("b12e5c3a-b76d-459b-beeb-901151cd77d1")
                ILanguageFont : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FontFamily(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FontWeight(
                        ABI::Windows::UI::Text::FontWeight* weight
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FontStretch(
                        ABI::Windows::UI::Text::FontStretch* stretch
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FontStyle(
                        ABI::Windows::UI::Text::FontStyle* style
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ScaleFactor(
                        DOUBLE* scale
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILanguageFont = __uuidof(ILanguageFont);
            } /* Fonts */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.Fonts.ILanguageFontGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Fonts.LanguageFontGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_Fonts_ILanguageFontGroup[] = L"Windows.Globalization.Fonts.ILanguageFontGroup";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace Fonts {
                MIDL_INTERFACE("f33a7fc3-3a5c-4aea-b9ff-b39fb242f7f6")
                ILanguageFontGroup : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UITextFont(
                        ABI::Windows::Globalization::Fonts::ILanguageFont** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UIHeadingFont(
                        ABI::Windows::Globalization::Fonts::ILanguageFont** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UITitleFont(
                        ABI::Windows::Globalization::Fonts::ILanguageFont** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UICaptionFont(
                        ABI::Windows::Globalization::Fonts::ILanguageFont** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UINotificationHeadingFont(
                        ABI::Windows::Globalization::Fonts::ILanguageFont** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TraditionalDocumentFont(
                        ABI::Windows::Globalization::Fonts::ILanguageFont** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ModernDocumentFont(
                        ABI::Windows::Globalization::Fonts::ILanguageFont** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DocumentHeadingFont(
                        ABI::Windows::Globalization::Fonts::ILanguageFont** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FixedWidthTextFont(
                        ABI::Windows::Globalization::Fonts::ILanguageFont** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DocumentAlternate1Font(
                        ABI::Windows::Globalization::Fonts::ILanguageFont** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DocumentAlternate2Font(
                        ABI::Windows::Globalization::Fonts::ILanguageFont** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILanguageFontGroup = __uuidof(ILanguageFontGroup);
            } /* Fonts */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.Fonts.ILanguageFontGroupFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Fonts.LanguageFontGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_Fonts_ILanguageFontGroupFactory[] = L"Windows.Globalization.Fonts.ILanguageFontGroupFactory";
namespace ABI {
    namespace Windows {
        namespace Globalization {
            namespace Fonts {
                MIDL_INTERFACE("fcaeac67-4e77-49c7-b856-dde934fc735b")
                ILanguageFontGroupFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateLanguageFontGroup(
                        HSTRING languageTag,
                        ABI::Windows::Globalization::Fonts::ILanguageFontGroup** recommendedFonts
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILanguageFontGroupFactory = __uuidof(ILanguageFontGroupFactory);
            } /* Fonts */
        } /* Globalization */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.Fonts.LanguageFont
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.Fonts.ILanguageFont ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_Fonts_LanguageFont_DEFINED
#define RUNTIMECLASS_Windows_Globalization_Fonts_LanguageFont_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_Fonts_LanguageFont[] = L"Windows.Globalization.Fonts.LanguageFont";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.Fonts.LanguageFontGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.Fonts.ILanguageFontGroupFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.Fonts.ILanguageFontGroup ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_Fonts_LanguageFontGroup_DEFINED
#define RUNTIMECLASS_Windows_Globalization_Fonts_LanguageFontGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_Fonts_LanguageFontGroup[] = L"Windows.Globalization.Fonts.LanguageFontGroup";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont;

#endif // ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup;

#endif // ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory;

#endif // ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef enum __x_ABI_CWindows_CUI_CText_CFontStretch __x_ABI_CWindows_CUI_CText_CFontStretch;

typedef enum __x_ABI_CWindows_CUI_CText_CFontStyle __x_ABI_CWindows_CUI_CText_CFontStyle;

typedef struct __x_ABI_CWindows_CUI_CText_CFontWeight __x_ABI_CWindows_CUI_CText_CFontWeight;

/*
 *
 * Interface Windows.Globalization.Fonts.ILanguageFont
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Fonts.LanguageFont
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_Fonts_ILanguageFont[] = L"Windows.Globalization.Fonts.ILanguageFont";
typedef struct __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FontFamily)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FontWeight)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont* This,
        struct __x_ABI_CWindows_CUI_CText_CFontWeight* weight);
    HRESULT (STDMETHODCALLTYPE* get_FontStretch)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont* This,
        enum __x_ABI_CWindows_CUI_CText_CFontStretch* stretch);
    HRESULT (STDMETHODCALLTYPE* get_FontStyle)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont* This,
        enum __x_ABI_CWindows_CUI_CText_CFontStyle* style);
    HRESULT (STDMETHODCALLTYPE* get_ScaleFactor)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont* This,
        DOUBLE* scale);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontVtbl;

interface __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_get_FontFamily(This, value) \
    ((This)->lpVtbl->get_FontFamily(This, value))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_get_FontWeight(This, weight) \
    ((This)->lpVtbl->get_FontWeight(This, weight))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_get_FontStretch(This, stretch) \
    ((This)->lpVtbl->get_FontStretch(This, stretch))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_get_FontStyle(This, style) \
    ((This)->lpVtbl->get_FontStyle(This, style))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_get_ScaleFactor(This, scale) \
    ((This)->lpVtbl->get_ScaleFactor(This, scale))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.Fonts.ILanguageFontGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Fonts.LanguageFontGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_Fonts_ILanguageFontGroup[] = L"Windows.Globalization.Fonts.ILanguageFontGroup";
typedef struct __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UITextFont)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This,
        __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont** value);
    HRESULT (STDMETHODCALLTYPE* get_UIHeadingFont)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This,
        __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont** value);
    HRESULT (STDMETHODCALLTYPE* get_UITitleFont)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This,
        __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont** value);
    HRESULT (STDMETHODCALLTYPE* get_UICaptionFont)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This,
        __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont** value);
    HRESULT (STDMETHODCALLTYPE* get_UINotificationHeadingFont)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This,
        __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont** value);
    HRESULT (STDMETHODCALLTYPE* get_TraditionalDocumentFont)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This,
        __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont** value);
    HRESULT (STDMETHODCALLTYPE* get_ModernDocumentFont)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This,
        __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentHeadingFont)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This,
        __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont** value);
    HRESULT (STDMETHODCALLTYPE* get_FixedWidthTextFont)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This,
        __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentAlternate1Font)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This,
        __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont** value);
    HRESULT (STDMETHODCALLTYPE* get_DocumentAlternate2Font)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup* This,
        __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFont** value);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupVtbl;

interface __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_get_UITextFont(This, value) \
    ((This)->lpVtbl->get_UITextFont(This, value))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_get_UIHeadingFont(This, value) \
    ((This)->lpVtbl->get_UIHeadingFont(This, value))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_get_UITitleFont(This, value) \
    ((This)->lpVtbl->get_UITitleFont(This, value))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_get_UICaptionFont(This, value) \
    ((This)->lpVtbl->get_UICaptionFont(This, value))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_get_UINotificationHeadingFont(This, value) \
    ((This)->lpVtbl->get_UINotificationHeadingFont(This, value))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_get_TraditionalDocumentFont(This, value) \
    ((This)->lpVtbl->get_TraditionalDocumentFont(This, value))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_get_ModernDocumentFont(This, value) \
    ((This)->lpVtbl->get_ModernDocumentFont(This, value))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_get_DocumentHeadingFont(This, value) \
    ((This)->lpVtbl->get_DocumentHeadingFont(This, value))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_get_FixedWidthTextFont(This, value) \
    ((This)->lpVtbl->get_FixedWidthTextFont(This, value))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_get_DocumentAlternate1Font(This, value) \
    ((This)->lpVtbl->get_DocumentAlternate1Font(This, value))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_get_DocumentAlternate2Font(This, value) \
    ((This)->lpVtbl->get_DocumentAlternate2Font(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Globalization.Fonts.ILanguageFontGroupFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Globalization.Fonts.LanguageFontGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Globalization_Fonts_ILanguageFontGroupFactory[] = L"Windows.Globalization.Fonts.ILanguageFontGroupFactory";
typedef struct __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateLanguageFontGroup)(__x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory* This,
        HSTRING languageTag,
        __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroup** recommendedFonts);

    END_INTERFACE
} __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactoryVtbl;

interface __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_CreateLanguageFontGroup(This, languageTag, recommendedFonts) \
    ((This)->lpVtbl->CreateLanguageFontGroup(This, languageTag, recommendedFonts))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory;
#endif /* !defined(____x_ABI_CWindows_CGlobalization_CFonts_CILanguageFontGroupFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.Fonts.LanguageFont
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.Fonts.ILanguageFont ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_Fonts_LanguageFont_DEFINED
#define RUNTIMECLASS_Windows_Globalization_Fonts_LanguageFont_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_Fonts_LanguageFont[] = L"Windows.Globalization.Fonts.LanguageFont";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Globalization.Fonts.LanguageFontGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Globalization.Fonts.ILanguageFontGroupFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Globalization.Fonts.ILanguageFontGroup ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Globalization_Fonts_LanguageFontGroup_DEFINED
#define RUNTIMECLASS_Windows_Globalization_Fonts_LanguageFontGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Globalization_Fonts_LanguageFontGroup[] = L"Windows.Globalization.Fonts.LanguageFontGroup";
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
#endif // __windows2Eglobalization2Efonts_p_h__

#endif // __windows2Eglobalization2Efonts_h__
