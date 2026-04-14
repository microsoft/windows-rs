
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
#ifndef __windows2Eui2Examl2Edocuments_h__
#define __windows2Eui2Examl2Edocuments_h__
#ifndef __windows2Eui2Examl2Edocuments_p_h__
#define __windows2Eui2Examl2Edocuments_p_h__


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

#if !defined(WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION)
#define WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_CORE_COREWINDOWDIALOGSCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.UI.Core.h"
#include "Windows.UI.Text.h"
#include "Windows.UI.Xaml.h"
#include "Windows.UI.Xaml.Input.h"
#include "Windows.UI.Xaml.Media.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IBlock;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock ABI::Windows::UI::Xaml::Documents::IBlock

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IBlock2;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2 ABI::Windows::UI::Xaml::Documents::IBlock2

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IBlockFactory;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory ABI::Windows::UI::Xaml::Documents::IBlockFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IBlockStatics;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics ABI::Windows::UI::Xaml::Documents::IBlockStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IBlockStatics2;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2 ABI::Windows::UI::Xaml::Documents::IBlockStatics2

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IBold;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold ABI::Windows::UI::Xaml::Documents::IBold

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IContactContentLinkProvider;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider ABI::Windows::UI::Xaml::Documents::IContactContentLinkProvider

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IContentLink;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink ABI::Windows::UI::Xaml::Documents::IContentLink

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IContentLinkInvokedEventArgs;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs ABI::Windows::UI::Xaml::Documents::IContentLinkInvokedEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IContentLinkProvider;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider ABI::Windows::UI::Xaml::Documents::IContentLinkProvider

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IContentLinkProviderCollection;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection ABI::Windows::UI::Xaml::Documents::IContentLinkProviderCollection

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IContentLinkProviderFactory;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory ABI::Windows::UI::Xaml::Documents::IContentLinkProviderFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IContentLinkStatics;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics ABI::Windows::UI::Xaml::Documents::IContentLinkStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IGlyphs;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs ABI::Windows::UI::Xaml::Documents::IGlyphs

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IGlyphs2;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2 ABI::Windows::UI::Xaml::Documents::IGlyphs2

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IGlyphsStatics;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics ABI::Windows::UI::Xaml::Documents::IGlyphsStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IGlyphsStatics2;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2 ABI::Windows::UI::Xaml::Documents::IGlyphsStatics2

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IHyperlink;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink ABI::Windows::UI::Xaml::Documents::IHyperlink

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IHyperlink2;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2 ABI::Windows::UI::Xaml::Documents::IHyperlink2

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IHyperlink3;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3 ABI::Windows::UI::Xaml::Documents::IHyperlink3

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IHyperlink4;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4 ABI::Windows::UI::Xaml::Documents::IHyperlink4

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IHyperlink5;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5 ABI::Windows::UI::Xaml::Documents::IHyperlink5

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IHyperlinkClickEventArgs;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs ABI::Windows::UI::Xaml::Documents::IHyperlinkClickEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IHyperlinkStatics;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics ABI::Windows::UI::Xaml::Documents::IHyperlinkStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IHyperlinkStatics2;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2 ABI::Windows::UI::Xaml::Documents::IHyperlinkStatics2

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IHyperlinkStatics3;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3 ABI::Windows::UI::Xaml::Documents::IHyperlinkStatics3

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IHyperlinkStatics4;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4 ABI::Windows::UI::Xaml::Documents::IHyperlinkStatics4

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IHyperlinkStatics5;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5 ABI::Windows::UI::Xaml::Documents::IHyperlinkStatics5

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IInline;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline ABI::Windows::UI::Xaml::Documents::IInline

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IInlineFactory;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory ABI::Windows::UI::Xaml::Documents::IInlineFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IInlineUIContainer;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer ABI::Windows::UI::Xaml::Documents::IInlineUIContainer

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IItalic;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic ABI::Windows::UI::Xaml::Documents::IItalic

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ILineBreak;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak ABI::Windows::UI::Xaml::Documents::ILineBreak

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IParagraph;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph ABI::Windows::UI::Xaml::Documents::IParagraph

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IParagraphStatics;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics ABI::Windows::UI::Xaml::Documents::IParagraphStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IPlaceContentLinkProvider;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider ABI::Windows::UI::Xaml::Documents::IPlaceContentLinkProvider

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IRun;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun ABI::Windows::UI::Xaml::Documents::IRun

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IRunStatics;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics ABI::Windows::UI::Xaml::Documents::IRunStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ISpan;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan ABI::Windows::UI::Xaml::Documents::ISpan

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ISpanFactory;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory ABI::Windows::UI::Xaml::Documents::ISpanFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextElement;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement ABI::Windows::UI::Xaml::Documents::ITextElement

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextElement2;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2 ABI::Windows::UI::Xaml::Documents::ITextElement2

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextElement3;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3 ABI::Windows::UI::Xaml::Documents::ITextElement3

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextElement4;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4 ABI::Windows::UI::Xaml::Documents::ITextElement4

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextElement5;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5 ABI::Windows::UI::Xaml::Documents::ITextElement5

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextElementFactory;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory ABI::Windows::UI::Xaml::Documents::ITextElementFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextElementOverrides;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides ABI::Windows::UI::Xaml::Documents::ITextElementOverrides

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextElementStatics;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics ABI::Windows::UI::Xaml::Documents::ITextElementStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextElementStatics2;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2 ABI::Windows::UI::Xaml::Documents::ITextElementStatics2

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextElementStatics3;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3 ABI::Windows::UI::Xaml::Documents::ITextElementStatics3

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextElementStatics4;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4 ABI::Windows::UI::Xaml::Documents::ITextElementStatics4

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextHighlighter;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter ABI::Windows::UI::Xaml::Documents::ITextHighlighter

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextHighlighterBase;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase ABI::Windows::UI::Xaml::Documents::ITextHighlighterBase

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextHighlighterBaseFactory;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory ABI::Windows::UI::Xaml::Documents::ITextHighlighterBaseFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextHighlighterFactory;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory ABI::Windows::UI::Xaml::Documents::ITextHighlighterFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextHighlighterStatics;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics ABI::Windows::UI::Xaml::Documents::ITextHighlighterStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITextPointer;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer ABI::Windows::UI::Xaml::Documents::ITextPointer

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITypography;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography ABI::Windows::UI::Xaml::Documents::ITypography

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface ITypographyStatics;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics ABI::Windows::UI::Xaml::Documents::ITypographyStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    interface IUnderline;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline ABI::Windows::UI::Xaml::Documents::IUnderline

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    class Block;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_USE
#define DEF___FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("798d518e-a9f8-5fc7-8ccc-2a49069aba05"))
IIterator<ABI::Windows::UI::Xaml::Documents::Block*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::Block*, ABI::Windows::UI::Xaml::Documents::IBlock*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Xaml.Documents.Block>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Xaml::Documents::Block*> __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_t;
#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock_USE
#define DEF___FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f7023b9a-e6d1-5e2d-8f41-b28c33323e04"))
IIterable<ABI::Windows::UI::Xaml::Documents::Block*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::Block*, ABI::Windows::UI::Xaml::Documents::IBlock*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Xaml.Documents.Block>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Xaml::Documents::Block*> __FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock_t;
#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    class ContentLinkProvider;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_USE
#define DEF___FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("63e9b404-2fc1-59b1-ab76-cdb9a4530c0d"))
IIterator<ABI::Windows::UI::Xaml::Documents::ContentLinkProvider*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::ContentLinkProvider*, ABI::Windows::UI::Xaml::Documents::IContentLinkProvider*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Xaml.Documents.ContentLinkProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Xaml::Documents::ContentLinkProvider*> __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_t;
#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_USE
#define DEF___FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("193a81bb-d85c-5cd3-a130-a1d08eaaf4be"))
IIterable<ABI::Windows::UI::Xaml::Documents::ContentLinkProvider*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::ContentLinkProvider*, ABI::Windows::UI::Xaml::Documents::IContentLinkProvider*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Xaml.Documents.ContentLinkProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Xaml::Documents::ContentLinkProvider*> __FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_t;
#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    class Inline;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_USE
#define DEF___FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f80dc964-2542-5c6a-ba65-b04824b5ed75"))
IIterator<ABI::Windows::UI::Xaml::Documents::Inline*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::Inline*, ABI::Windows::UI::Xaml::Documents::IInline*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Xaml.Documents.Inline>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Xaml::Documents::Inline*> __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_t;
#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline_USE
#define DEF___FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e1d2b910-18c2-5906-8f8a-d62a63f93f18"))
IIterable<ABI::Windows::UI::Xaml::Documents::Inline*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::Inline*, ABI::Windows::UI::Xaml::Documents::IInline*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Xaml.Documents.Inline>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Xaml::Documents::Inline*> __FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline_t;
#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    typedef struct TextRange TextRange;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_USE
#define DEF___FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("71f63622-c0fe-5423-914e-d319d25bcc84"))
IIterator<struct ABI::Windows::UI::Xaml::Documents::TextRange> : IIterator_impl<struct ABI::Windows::UI::Xaml::Documents::TextRange>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Xaml.Documents.TextRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<struct ABI::Windows::UI::Xaml::Documents::TextRange> __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_t;
#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange_USE
#define DEF___FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2ad42fdb-56db-500b-8ea8-3d57edfadfc6"))
IIterable<struct ABI::Windows::UI::Xaml::Documents::TextRange> : IIterable_impl<struct ABI::Windows::UI::Xaml::Documents::TextRange>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Xaml.Documents.TextRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<struct ABI::Windows::UI::Xaml::Documents::TextRange> __FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange_t;
#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_USE
#define DEF___FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("995f73c4-7cf8-5b59-a0fb-7e0c6477172e"))
IVectorView<ABI::Windows::UI::Xaml::Documents::Block*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::Block*, ABI::Windows::UI::Xaml::Documents::IBlock*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Xaml.Documents.Block>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Xaml::Documents::Block*> __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_t;
#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_USE
#define DEF___FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6548aa5f-3fd4-5bea-9bd5-138b5bd899fe"))
IVectorView<ABI::Windows::UI::Xaml::Documents::ContentLinkProvider*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::ContentLinkProvider*, ABI::Windows::UI::Xaml::Documents::IContentLinkProvider*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Xaml.Documents.ContentLinkProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Xaml::Documents::ContentLinkProvider*> __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_t;
#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_USE
#define DEF___FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c2dd082f-8cb4-51d6-b91b-7e2377780cee"))
IVectorView<ABI::Windows::UI::Xaml::Documents::Inline*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::Inline*, ABI::Windows::UI::Xaml::Documents::IInline*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Xaml.Documents.Inline>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Xaml::Documents::Inline*> __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_t;
#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_USE
#define DEF___FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c824d1d0-771a-5123-90cc-52281f0f287a"))
IVectorView<struct ABI::Windows::UI::Xaml::Documents::TextRange> : IVectorView_impl<struct ABI::Windows::UI::Xaml::Documents::TextRange>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Xaml.Documents.TextRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<struct ABI::Windows::UI::Xaml::Documents::TextRange> __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_t;
#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_USE
#define DEF___FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3ee78a34-160e-50ff-b5aa-09f263a669f8"))
IVector<ABI::Windows::UI::Xaml::Documents::Block*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::Block*, ABI::Windows::UI::Xaml::Documents::IBlock*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Xaml.Documents.Block>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::Xaml::Documents::Block*> __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_t;
#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_USE
#define DEF___FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e69ef1b6-2eb1-5e9c-bc41-b94d396281e4"))
IVector<ABI::Windows::UI::Xaml::Documents::ContentLinkProvider*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::ContentLinkProvider*, ABI::Windows::UI::Xaml::Documents::IContentLinkProvider*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Xaml.Documents.ContentLinkProvider>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::Xaml::Documents::ContentLinkProvider*> __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_t;
#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_USE
#define DEF___FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("92ec9252-8ee3-55d6-84b4-30b635077778"))
IVector<ABI::Windows::UI::Xaml::Documents::Inline*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::Inline*, ABI::Windows::UI::Xaml::Documents::IInline*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Xaml.Documents.Inline>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::Xaml::Documents::Inline*> __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_t;
#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_USE
#define DEF___FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ee9d4cda-0750-5c1f-93aa-59add8c1421b"))
IVector<struct ABI::Windows::UI::Xaml::Documents::TextRange> : IVector_impl<struct ABI::Windows::UI::Xaml::Documents::TextRange>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Xaml.Documents.TextRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<struct ABI::Windows::UI::Xaml::Documents::TextRange> __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_t;
#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    class ContentLink;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    class ContentLinkInvokedEventArgs;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("51ba1af0-354e-5868-b10b-8748e55a6370"))
ITypedEventHandler<ABI::Windows::UI::Xaml::Documents::ContentLink*, ABI::Windows::UI::Xaml::Documents::ContentLinkInvokedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::ContentLink*, ABI::Windows::UI::Xaml::Documents::IContentLink*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::ContentLinkInvokedEventArgs*, ABI::Windows::UI::Xaml::Documents::IContentLinkInvokedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Xaml.Documents.ContentLink, Windows.UI.Xaml.Documents.ContentLinkInvokedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Xaml::Documents::ContentLink*, ABI::Windows::UI::Xaml::Documents::ContentLinkInvokedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    class Hyperlink;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    class HyperlinkClickEventArgs;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5fead0d2-e657-5aef-a91b-7f52ead17fe3"))
ITypedEventHandler<ABI::Windows::UI::Xaml::Documents::Hyperlink*, ABI::Windows::UI::Xaml::Documents::HyperlinkClickEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::Hyperlink*, ABI::Windows::UI::Xaml::Documents::IHyperlink*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::HyperlinkClickEventArgs*, ABI::Windows::UI::Xaml::Documents::IHyperlinkClickEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Xaml.Documents.Hyperlink, Windows.UI.Xaml.Documents.HyperlinkClickEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Xaml::Documents::Hyperlink*, ABI::Windows::UI::Xaml::Documents::HyperlinkClickEventArgs*> __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    class TextElement;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Input {
                    class AccessKeyDisplayDismissedEventArgs;
                } /* Input */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayDismissedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayDismissedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Input {
                    interface IAccessKeyDisplayDismissedEventArgs;
                } /* Input */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayDismissedEventArgs ABI::Windows::UI::Xaml::Input::IAccessKeyDisplayDismissedEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayDismissedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a8c9544b-7078-5316-beb6-d9ec916ab88a"))
ITypedEventHandler<ABI::Windows::UI::Xaml::Documents::TextElement*, ABI::Windows::UI::Xaml::Input::AccessKeyDisplayDismissedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::TextElement*, ABI::Windows::UI::Xaml::Documents::ITextElement*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Input::AccessKeyDisplayDismissedEventArgs*, ABI::Windows::UI::Xaml::Input::IAccessKeyDisplayDismissedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Xaml.Documents.TextElement, Windows.UI.Xaml.Input.AccessKeyDisplayDismissedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Xaml::Documents::TextElement*, ABI::Windows::UI::Xaml::Input::AccessKeyDisplayDismissedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Input {
                    class AccessKeyDisplayRequestedEventArgs;
                } /* Input */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Input {
                    interface IAccessKeyDisplayRequestedEventArgs;
                } /* Input */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayRequestedEventArgs ABI::Windows::UI::Xaml::Input::IAccessKeyDisplayRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayRequestedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("39ad7708-254e-560a-9e2e-73b1db31f935"))
ITypedEventHandler<ABI::Windows::UI::Xaml::Documents::TextElement*, ABI::Windows::UI::Xaml::Input::AccessKeyDisplayRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::TextElement*, ABI::Windows::UI::Xaml::Documents::ITextElement*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Input::AccessKeyDisplayRequestedEventArgs*, ABI::Windows::UI::Xaml::Input::IAccessKeyDisplayRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Xaml.Documents.TextElement, Windows.UI.Xaml.Input.AccessKeyDisplayRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Xaml::Documents::TextElement*, ABI::Windows::UI::Xaml::Input::AccessKeyDisplayRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Input {
                    class AccessKeyInvokedEventArgs;
                } /* Input */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyInvokedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyInvokedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Input {
                    interface IAccessKeyInvokedEventArgs;
                } /* Input */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyInvokedEventArgs ABI::Windows::UI::Xaml::Input::IAccessKeyInvokedEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyInvokedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a72c6b01-2e3c-57b5-9ec4-948f6c6d930a"))
ITypedEventHandler<ABI::Windows::UI::Xaml::Documents::TextElement*, ABI::Windows::UI::Xaml::Input::AccessKeyInvokedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Documents::TextElement*, ABI::Windows::UI::Xaml::Documents::ITextElement*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Input::AccessKeyInvokedEventArgs*, ABI::Windows::UI::Xaml::Input::IAccessKeyInvokedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Xaml.Documents.TextElement, Windows.UI.Xaml.Input.AccessKeyInvokedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Xaml::Documents::TextElement*, ABI::Windows::UI::Xaml::Input::AccessKeyInvokedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
            namespace Core {
                typedef enum CoreCursorType : int CoreCursorType;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                class ContentLinkInfo;
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CText_CIContentLinkInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CIContentLinkInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                interface IContentLinkInfo;
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CIContentLinkInfo ABI::Windows::UI::Text::IContentLinkInfo

#endif // ____x_ABI_CWindows_CUI_CText_CIContentLinkInfo_FWD_DEFINED__

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
        namespace UI {
            namespace Text {
                typedef enum TextDecorations : unsigned int TextDecorations;
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                class DependencyObject;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIDependencyObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIDependencyObject_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IDependencyObject;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIDependencyObject ABI::Windows::UI::Xaml::IDependencyObject

#endif // ____x_ABI_CWindows_CUI_CXaml_CIDependencyObject_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                class DependencyProperty;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IDependencyProperty;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty ABI::Windows::UI::Xaml::IDependencyProperty

#endif // ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef enum ElementSoundMode : int ElementSoundMode;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef enum FlowDirection : int FlowDirection;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef enum FocusState : int FocusState;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef enum FontCapitals : int FontCapitals;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef enum FontEastAsianLanguage : int FontEastAsianLanguage;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef enum FontEastAsianWidths : int FontEastAsianWidths;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef enum FontFraction : int FontFraction;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef enum FontNumeralAlignment : int FontNumeralAlignment;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef enum FontNumeralStyle : int FontNumeralStyle;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef enum FontVariants : int FontVariants;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                class FrameworkElement;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIFrameworkElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIFrameworkElement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IFrameworkElement;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIFrameworkElement ABI::Windows::UI::Xaml::IFrameworkElement

#endif // ____x_ABI_CWindows_CUI_CXaml_CIFrameworkElement_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Input {
                    typedef enum KeyTipPlacementMode : int KeyTipPlacementMode;
                } /* Input */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Input {
                    typedef enum XYFocusNavigationStrategy : int XYFocusNavigationStrategy;
                } /* Input */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef enum LineStackingStrategy : int LineStackingStrategy;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    class Brush;
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    interface IBrush;
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush ABI::Windows::UI::Xaml::Media::IBrush

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    class FontFamily;
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CIFontFamily_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CIFontFamily_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    interface IFontFamily;
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CMedia_CIFontFamily ABI::Windows::UI::Xaml::Media::IFontFamily

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CIFontFamily_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Media {
                    typedef enum StyleSimulations : int StyleSimulations;
                } /* Media */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IRoutedEventHandler;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler ABI::Windows::UI::Xaml::IRoutedEventHandler

#endif // ____x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef enum TextAlignment : int TextAlignment;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                typedef struct Thickness Thickness;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                class UIElement;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IUIElement;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIUIElement ABI::Windows::UI::Xaml::IUIElement

#endif // ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                class XamlRoot;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIXamlRoot_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIXamlRoot_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                interface IXamlRoot;
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CIXamlRoot ABI::Windows::UI::Xaml::IXamlRoot

#endif // ____x_ABI_CWindows_CUI_CXaml_CIXamlRoot_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    typedef enum LogicalDirection : int LogicalDirection;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    typedef enum UnderlineStyle : int UnderlineStyle;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    class InlineCollection;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    class Span;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    class TextHighlighter;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    class TextPointer;
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Xaml.Documents.LogicalDirection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    enum LogicalDirection : int
                    {
                        LogicalDirection_Backward = 0,
                        LogicalDirection_Forward = 1,
                    };
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Documents.UnderlineStyle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    enum UnderlineStyle : int
                    {
                        UnderlineStyle_None = 0,
                        UnderlineStyle_Single = 1,
                    };
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Documents.TextRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    struct TextRange
                    {
                        INT32 StartIndex;
                        INT32 Length;
                    };
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IBlock
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Block
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IBlock[] = L"Windows.UI.Xaml.Documents.IBlock";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("4bce0016-dd47-4350-8cb0-e171600ac896")
                    IBlock : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TextAlignment(
                            ABI::Windows::UI::Xaml::TextAlignment* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_TextAlignment(
                            ABI::Windows::UI::Xaml::TextAlignment value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LineHeight(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_LineHeight(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LineStackingStrategy(
                            ABI::Windows::UI::Xaml::LineStackingStrategy* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_LineStackingStrategy(
                            ABI::Windows::UI::Xaml::LineStackingStrategy value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Margin(
                            ABI::Windows::UI::Xaml::Thickness* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Margin(
                            ABI::Windows::UI::Xaml::Thickness value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBlock = __uuidof(IBlock);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IBlock2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Block
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IBlock2[] = L"Windows.UI.Xaml.Documents.IBlock2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("5ec7bdf3-1333-4a92-8318-6caedc12ef89")
                    IBlock2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_HorizontalTextAlignment(
                            ABI::Windows::UI::Xaml::TextAlignment* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_HorizontalTextAlignment(
                            ABI::Windows::UI::Xaml::TextAlignment value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBlock2 = __uuidof(IBlock2);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IBlockFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Block
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IBlockFactory[] = L"Windows.UI.Xaml.Documents.IBlockFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("07110532-4f59-4f3b-9ce5-25784c430507")
                    IBlockFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            IInspectable* baseInterface,
                            IInspectable** innerInterface,
                            ABI::Windows::UI::Xaml::Documents::IBlock** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBlockFactory = __uuidof(IBlockFactory);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IBlockStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Block
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IBlockStatics[] = L"Windows.UI.Xaml.Documents.IBlockStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("f86a8c34-8d18-4c53-aebd-91e610a5e010")
                    IBlockStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TextAlignmentProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LineHeightProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LineStackingStrategyProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MarginProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBlockStatics = __uuidof(IBlockStatics);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IBlockStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Block
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IBlockStatics2[] = L"Windows.UI.Xaml.Documents.IBlockStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("af01a4d6-03e3-4cee-9b02-2bfc308b27a9")
                    IBlockStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_HorizontalTextAlignmentProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBlockStatics2 = __uuidof(IBlockStatics2);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IBold
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Bold
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IBold[] = L"Windows.UI.Xaml.Documents.IBold";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("ade73784-1b59-4da4-bb23-0f20e885b4bf")
                    IBold : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IBold = __uuidof(IBold);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IContactContentLinkProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.ContactContentLinkProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IContactContentLinkProvider[] = L"Windows.UI.Xaml.Documents.IContactContentLinkProvider";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("f92fd29b-589b-4abd-9d37-35a1468f021e")
                    IContactContentLinkProvider : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IContactContentLinkProvider = __uuidof(IContactContentLinkProvider);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IContentLink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.ContentLink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IContentLink[] = L"Windows.UI.Xaml.Documents.IContentLink";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("6c60c3e1-528c-42f8-92be-34b8c68be304")
                    IContentLink : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Info(
                            ABI::Windows::UI::Text::IContentLinkInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Info(
                            ABI::Windows::UI::Text::IContentLinkInfo* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Background(
                            ABI::Windows::UI::Xaml::Media::IBrush** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Background(
                            ABI::Windows::UI::Xaml::Media::IBrush* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Cursor(
                            ABI::Windows::UI::Core::CoreCursorType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Cursor(
                            ABI::Windows::UI::Core::CoreCursorType value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusLeft(
                            ABI::Windows::UI::Xaml::IDependencyObject** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XYFocusLeft(
                            ABI::Windows::UI::Xaml::IDependencyObject* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusRight(
                            ABI::Windows::UI::Xaml::IDependencyObject** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XYFocusRight(
                            ABI::Windows::UI::Xaml::IDependencyObject* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusUp(
                            ABI::Windows::UI::Xaml::IDependencyObject** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XYFocusUp(
                            ABI::Windows::UI::Xaml::IDependencyObject* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusDown(
                            ABI::Windows::UI::Xaml::IDependencyObject** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XYFocusDown(
                            ABI::Windows::UI::Xaml::IDependencyObject* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ElementSoundMode(
                            ABI::Windows::UI::Xaml::ElementSoundMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ElementSoundMode(
                            ABI::Windows::UI::Xaml::ElementSoundMode value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FocusState(
                            ABI::Windows::UI::Xaml::FocusState* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusUpNavigationStrategy(
                            ABI::Windows::UI::Xaml::Input::XYFocusNavigationStrategy* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XYFocusUpNavigationStrategy(
                            ABI::Windows::UI::Xaml::Input::XYFocusNavigationStrategy value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusDownNavigationStrategy(
                            ABI::Windows::UI::Xaml::Input::XYFocusNavigationStrategy* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XYFocusDownNavigationStrategy(
                            ABI::Windows::UI::Xaml::Input::XYFocusNavigationStrategy value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusLeftNavigationStrategy(
                            ABI::Windows::UI::Xaml::Input::XYFocusNavigationStrategy* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XYFocusLeftNavigationStrategy(
                            ABI::Windows::UI::Xaml::Input::XYFocusNavigationStrategy value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusRightNavigationStrategy(
                            ABI::Windows::UI::Xaml::Input::XYFocusNavigationStrategy* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XYFocusRightNavigationStrategy(
                            ABI::Windows::UI::Xaml::Input::XYFocusNavigationStrategy value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsTabStop(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsTabStop(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TabIndex(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_TabIndex(
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Invoked(
                            __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Invoked(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_GotFocus(
                            ABI::Windows::UI::Xaml::IRoutedEventHandler* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_GotFocus(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_LostFocus(
                            ABI::Windows::UI::Xaml::IRoutedEventHandler* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_LostFocus(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Focus(
                            ABI::Windows::UI::Xaml::FocusState value,
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContentLink = __uuidof(IContentLink);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IContentLinkInvokedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.ContentLinkInvokedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IContentLinkInvokedEventArgs[] = L"Windows.UI.Xaml.Documents.IContentLinkInvokedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("546717c1-e8df-4593-9639-97595fdf8310")
                    IContentLinkInvokedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ContentLinkInfo(
                            ABI::Windows::UI::Text::IContentLinkInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Handled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Handled(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContentLinkInvokedEventArgs = __uuidof(IContentLinkInvokedEventArgs);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IContentLinkProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.ContentLinkProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IContentLinkProvider[] = L"Windows.UI.Xaml.Documents.IContentLinkProvider";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("730587fd-bfdc-4cb3-904d-b65ab339bbf5")
                    IContentLinkProvider : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IContentLinkProvider = __uuidof(IContentLinkProvider);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IContentLinkProviderCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.ContentLinkProviderCollection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IContentLinkProviderCollection[] = L"Windows.UI.Xaml.Documents.IContentLinkProviderCollection";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("f5b84d0c-a9f4-4d1a-a13c-10def1843734")
                    IContentLinkProviderCollection : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IContentLinkProviderCollection = __uuidof(IContentLinkProviderCollection);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IContentLinkProviderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.ContentLinkProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IContentLinkProviderFactory[] = L"Windows.UI.Xaml.Documents.IContentLinkProviderFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("57d60d3b-ef1a-4e8e-839b-d36ef3a503e0")
                    IContentLinkProviderFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            IInspectable* baseInterface,
                            IInspectable** innerInterface,
                            ABI::Windows::UI::Xaml::Documents::IContentLinkProvider** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContentLinkProviderFactory = __uuidof(IContentLinkProviderFactory);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IContentLinkStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.ContentLink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IContentLinkStatics[] = L"Windows.UI.Xaml.Documents.IContentLinkStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("a34e3063-eb16-484e-a3df-522b9a832e6e")
                    IContentLinkStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_BackgroundProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CursorProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusLeftProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusRightProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusUpProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusDownProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ElementSoundModeProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FocusStateProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusUpNavigationStrategyProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusDownNavigationStrategyProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusLeftNavigationStrategyProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusRightNavigationStrategyProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsTabStopProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TabIndexProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContentLinkStatics = __uuidof(IContentLinkStatics);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IGlyphs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Glyphs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IGlyphs[] = L"Windows.UI.Xaml.Documents.IGlyphs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("d079498b-f2b1-4281-99a2-e4d05932b2b5")
                    IGlyphs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_UnicodeString(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_UnicodeString(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Indices(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Indices(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FontUri(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_FontUri(
                            ABI::Windows::Foundation::IUriRuntimeClass* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StyleSimulations(
                            ABI::Windows::UI::Xaml::Media::StyleSimulations* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StyleSimulations(
                            ABI::Windows::UI::Xaml::Media::StyleSimulations value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FontRenderingEmSize(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_FontRenderingEmSize(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OriginX(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_OriginX(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OriginY(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_OriginY(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Fill(
                            ABI::Windows::UI::Xaml::Media::IBrush** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Fill(
                            ABI::Windows::UI::Xaml::Media::IBrush* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGlyphs = __uuidof(IGlyphs);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IGlyphs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Glyphs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IGlyphs2[] = L"Windows.UI.Xaml.Documents.IGlyphs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("aa8bfe5c-3754-4bee-bbe1-4403ee9b86f0")
                    IGlyphs2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsColorFontEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsColorFontEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ColorFontPaletteIndex(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ColorFontPaletteIndex(
                            INT32 value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGlyphs2 = __uuidof(IGlyphs2);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IGlyphsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Glyphs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IGlyphsStatics[] = L"Windows.UI.Xaml.Documents.IGlyphsStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("225cf4c5-fdf1-43ed-958f-414e86f103f2")
                    IGlyphsStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_UnicodeStringProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IndicesProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FontUriProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StyleSimulationsProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FontRenderingEmSizeProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OriginXProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OriginYProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FillProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGlyphsStatics = __uuidof(IGlyphsStatics);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IGlyphsStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Glyphs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IGlyphsStatics2[] = L"Windows.UI.Xaml.Documents.IGlyphsStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("10489aa7-1615-4a33-aa02-d7ef2aefc739")
                    IGlyphsStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsColorFontEnabledProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ColorFontPaletteIndexProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGlyphsStatics2 = __uuidof(IGlyphsStatics2);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlink[] = L"Windows.UI.Xaml.Documents.IHyperlink";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("0fe2363b-14e9-4152-9e58-5aea5b21f08d")
                    IHyperlink : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_NavigateUri(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_NavigateUri(
                            ABI::Windows::Foundation::IUriRuntimeClass* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Click(
                            __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Click(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHyperlink = __uuidof(IHyperlink);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlink2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlink2[] = L"Windows.UI.Xaml.Documents.IHyperlink2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("4ce9da5f-7cff-4291-b78f-dfec72490576")
                    IHyperlink2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_UnderlineStyle(
                            ABI::Windows::UI::Xaml::Documents::UnderlineStyle* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_UnderlineStyle(
                            ABI::Windows::UI::Xaml::Documents::UnderlineStyle value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHyperlink2 = __uuidof(IHyperlink2);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlink3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlink3[] = L"Windows.UI.Xaml.Documents.IHyperlink3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("c3f157d9-e5d3-4fb7-8702-4f6d85dd9e0a")
                    IHyperlink3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusLeft(
                            ABI::Windows::UI::Xaml::IDependencyObject** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XYFocusLeft(
                            ABI::Windows::UI::Xaml::IDependencyObject* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusRight(
                            ABI::Windows::UI::Xaml::IDependencyObject** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XYFocusRight(
                            ABI::Windows::UI::Xaml::IDependencyObject* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusUp(
                            ABI::Windows::UI::Xaml::IDependencyObject** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XYFocusUp(
                            ABI::Windows::UI::Xaml::IDependencyObject* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusDown(
                            ABI::Windows::UI::Xaml::IDependencyObject** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XYFocusDown(
                            ABI::Windows::UI::Xaml::IDependencyObject* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ElementSoundMode(
                            ABI::Windows::UI::Xaml::ElementSoundMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ElementSoundMode(
                            ABI::Windows::UI::Xaml::ElementSoundMode value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHyperlink3 = __uuidof(IHyperlink3);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlink4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlink4[] = L"Windows.UI.Xaml.Documents.IHyperlink4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("f7d02959-82fb-400a-a407-5a4ee677988a")
                    IHyperlink4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FocusState(
                            ABI::Windows::UI::Xaml::FocusState* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusUpNavigationStrategy(
                            ABI::Windows::UI::Xaml::Input::XYFocusNavigationStrategy* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XYFocusUpNavigationStrategy(
                            ABI::Windows::UI::Xaml::Input::XYFocusNavigationStrategy value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusDownNavigationStrategy(
                            ABI::Windows::UI::Xaml::Input::XYFocusNavigationStrategy* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XYFocusDownNavigationStrategy(
                            ABI::Windows::UI::Xaml::Input::XYFocusNavigationStrategy value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusLeftNavigationStrategy(
                            ABI::Windows::UI::Xaml::Input::XYFocusNavigationStrategy* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XYFocusLeftNavigationStrategy(
                            ABI::Windows::UI::Xaml::Input::XYFocusNavigationStrategy value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusRightNavigationStrategy(
                            ABI::Windows::UI::Xaml::Input::XYFocusNavigationStrategy* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XYFocusRightNavigationStrategy(
                            ABI::Windows::UI::Xaml::Input::XYFocusNavigationStrategy value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_GotFocus(
                            ABI::Windows::UI::Xaml::IRoutedEventHandler* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_GotFocus(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_LostFocus(
                            ABI::Windows::UI::Xaml::IRoutedEventHandler* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_LostFocus(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Focus(
                            ABI::Windows::UI::Xaml::FocusState value,
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHyperlink4 = __uuidof(IHyperlink4);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlink5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlink5[] = L"Windows.UI.Xaml.Documents.IHyperlink5";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("607dd7d2-0945-4328-91ee-94ccec2ea6c3")
                    IHyperlink5 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsTabStop(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsTabStop(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TabIndex(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_TabIndex(
                            INT32 value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHyperlink5 = __uuidof(IHyperlink5);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlinkClickEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.HyperlinkClickEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlinkClickEventArgs[] = L"Windows.UI.Xaml.Documents.IHyperlinkClickEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("c755916b-7bdc-4be7-b373-9240a503d870")
                    IHyperlinkClickEventArgs : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IHyperlinkClickEventArgs = __uuidof(IHyperlinkClickEventArgs);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlinkStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlinkStatics[] = L"Windows.UI.Xaml.Documents.IHyperlinkStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("3a44d3d4-fd41-41db-8c72-3b790acd9fd3")
                    IHyperlinkStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_NavigateUriProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHyperlinkStatics = __uuidof(IHyperlinkStatics);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlinkStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlinkStatics2[] = L"Windows.UI.Xaml.Documents.IHyperlinkStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("5028d8b7-7adf-43ee-a4ae-9c925f755716")
                    IHyperlinkStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_UnderlineStyleProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHyperlinkStatics2 = __uuidof(IHyperlinkStatics2);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlinkStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlinkStatics3[] = L"Windows.UI.Xaml.Documents.IHyperlinkStatics3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("3e15dea0-205e-4947-99a5-74e757e8e1b4")
                    IHyperlinkStatics3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusLeftProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusRightProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusUpProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusDownProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ElementSoundModeProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHyperlinkStatics3 = __uuidof(IHyperlinkStatics3);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlinkStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlinkStatics4[] = L"Windows.UI.Xaml.Documents.IHyperlinkStatics4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("0476b378-8faa-4e24-b3b6-e9de4d3c708c")
                    IHyperlinkStatics4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FocusStateProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusUpNavigationStrategyProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusDownNavigationStrategyProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusLeftNavigationStrategyProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_XYFocusRightNavigationStrategyProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHyperlinkStatics4 = __uuidof(IHyperlinkStatics4);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlinkStatics5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlinkStatics5[] = L"Windows.UI.Xaml.Documents.IHyperlinkStatics5";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("59308cea-1e49-4921-bd88-a2878d07e30e")
                    IHyperlinkStatics5 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsTabStopProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TabIndexProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IHyperlinkStatics5 = __uuidof(IHyperlinkStatics5);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IInline
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Inline
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IInline[] = L"Windows.UI.Xaml.Documents.IInline";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("0c92712d-1bc9-4931-8cb1-1aeadf1cc685")
                    IInline : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IInline = __uuidof(IInline);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IInlineFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Inline
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IInlineFactory[] = L"Windows.UI.Xaml.Documents.IInlineFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("4058acd1-2f90-4b8f-99dd-4218ef5f03de")
                    IInlineFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            IInspectable* baseInterface,
                            IInspectable** innerInterface,
                            ABI::Windows::UI::Xaml::Documents::IInline** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInlineFactory = __uuidof(IInlineFactory);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IInlineUIContainer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.InlineUIContainer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IInlineUIContainer[] = L"Windows.UI.Xaml.Documents.IInlineUIContainer";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("1416ce81-28ee-452e-b121-5fc4f60b86a6")
                    IInlineUIContainer : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Child(
                            ABI::Windows::UI::Xaml::IUIElement** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Child(
                            ABI::Windows::UI::Xaml::IUIElement* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInlineUIContainer = __uuidof(IInlineUIContainer);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IItalic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Italic
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IItalic[] = L"Windows.UI.Xaml.Documents.IItalic";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("91f4619c-fcbb-4157-802c-76f63b5fb657")
                    IItalic : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IItalic = __uuidof(IItalic);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ILineBreak
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.LineBreak
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ILineBreak[] = L"Windows.UI.Xaml.Documents.ILineBreak";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("645589c4-f769-41ed-895b-8a1b2fb31562")
                    ILineBreak : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ILineBreak = __uuidof(ILineBreak);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IParagraph
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Paragraph
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IParagraph[] = L"Windows.UI.Xaml.Documents.IParagraph";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("f83ef59a-fa61-4bef-ae33-0b0ad756a84d")
                    IParagraph : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Inlines(
                            __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TextIndent(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_TextIndent(
                            DOUBLE value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IParagraph = __uuidof(IParagraph);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IParagraphStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Paragraph
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IParagraphStatics[] = L"Windows.UI.Xaml.Documents.IParagraphStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("ef08889a-535b-4e4c-8d84-283b33e98a37")
                    IParagraphStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TextIndentProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IParagraphStatics = __uuidof(IParagraphStatics);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IPlaceContentLinkProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.PlaceContentLinkProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IPlaceContentLinkProvider[] = L"Windows.UI.Xaml.Documents.IPlaceContentLinkProvider";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("10348a4c-2366-41be-90c8-3258b53b5483")
                    IPlaceContentLinkProvider : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IPlaceContentLinkProvider = __uuidof(IPlaceContentLinkProvider);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IRun
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Run
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IRun[] = L"Windows.UI.Xaml.Documents.IRun";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("59553c83-0e14-49bd-b84b-c526f3034349")
                    IRun : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Text(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Text(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FlowDirection(
                            ABI::Windows::UI::Xaml::FlowDirection* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_FlowDirection(
                            ABI::Windows::UI::Xaml::FlowDirection value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRun = __uuidof(IRun);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IRunStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Run
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IRunStatics[] = L"Windows.UI.Xaml.Documents.IRunStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("e9303cef-65a0-4b8d-a7f7-8fdb287b46f3")
                    IRunStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FlowDirectionProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRunStatics = __uuidof(IRunStatics);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ISpan
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Span
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ISpan[] = L"Windows.UI.Xaml.Documents.ISpan";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("9839d4a9-02af-4811-aa15-6bef3acac97a")
                    ISpan : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Inlines(
                            __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Inlines(
                            __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpan = __uuidof(ISpan);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ISpanFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Span
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ISpanFactory[] = L"Windows.UI.Xaml.Documents.ISpanFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("5b916f5c-cd2d-40c0-956a-386448322f79")
                    ISpanFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            IInspectable* baseInterface,
                            IInspectable** innerInterface,
                            ABI::Windows::UI::Xaml::Documents::ISpan** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpanFactory = __uuidof(ISpanFactory);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElement[] = L"Windows.UI.Xaml.Documents.ITextElement";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("e83b0062-d776-4f92-baea-40e77d4791d5")
                    ITextElement : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FontSize(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_FontSize(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FontFamily(
                            ABI::Windows::UI::Xaml::Media::IFontFamily** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_FontFamily(
                            ABI::Windows::UI::Xaml::Media::IFontFamily* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FontWeight(
                            ABI::Windows::UI::Text::FontWeight* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_FontWeight(
                            ABI::Windows::UI::Text::FontWeight value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FontStyle(
                            ABI::Windows::UI::Text::FontStyle* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_FontStyle(
                            ABI::Windows::UI::Text::FontStyle value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FontStretch(
                            ABI::Windows::UI::Text::FontStretch* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_FontStretch(
                            ABI::Windows::UI::Text::FontStretch value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CharacterSpacing(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CharacterSpacing(
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Foreground(
                            ABI::Windows::UI::Xaml::Media::IBrush** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Foreground(
                            ABI::Windows::UI::Xaml::Media::IBrush* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Language(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Language(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContentStart(
                            ABI::Windows::UI::Xaml::Documents::ITextPointer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContentEnd(
                            ABI::Windows::UI::Xaml::Documents::ITextPointer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ElementStart(
                            ABI::Windows::UI::Xaml::Documents::ITextPointer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ElementEnd(
                            ABI::Windows::UI::Xaml::Documents::ITextPointer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FindName(
                            HSTRING name,
                            IInspectable** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITextElement = __uuidof(ITextElement);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElement2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElement2[] = L"Windows.UI.Xaml.Documents.ITextElement2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("a8076aa8-f892-49f6-8cd2-89addaf06d2d")
                    ITextElement2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsTextScaleFactorEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsTextScaleFactorEnabled(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITextElement2 = __uuidof(ITextElement2);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElement3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElement3[] = L"Windows.UI.Xaml.Documents.ITextElement3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("d1db340f-1bc4-4ca8-bcf7-770bff9b27ab")
                    ITextElement3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AllowFocusOnInteraction(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AllowFocusOnInteraction(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AccessKey(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AccessKey(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExitDisplayModeOnAccessKeyInvoked(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ExitDisplayModeOnAccessKeyInvoked(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITextElement3 = __uuidof(ITextElement3);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElement4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElement4[] = L"Windows.UI.Xaml.Documents.ITextElement4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("b196e222-ca0e-48a9-83bc-36ce50566ac7")
                    ITextElement4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TextDecorations(
                            ABI::Windows::UI::Text::TextDecorations* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_TextDecorations(
                            ABI::Windows::UI::Text::TextDecorations value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsAccessKeyScope(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsAccessKeyScope(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AccessKeyScopeOwner(
                            ABI::Windows::UI::Xaml::IDependencyObject** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AccessKeyScopeOwner(
                            ABI::Windows::UI::Xaml::IDependencyObject* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyTipPlacementMode(
                            ABI::Windows::UI::Xaml::Input::KeyTipPlacementMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_KeyTipPlacementMode(
                            ABI::Windows::UI::Xaml::Input::KeyTipPlacementMode value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyTipHorizontalOffset(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_KeyTipHorizontalOffset(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyTipVerticalOffset(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_KeyTipVerticalOffset(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_AccessKeyDisplayRequested(
                            __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_AccessKeyDisplayRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_AccessKeyDisplayDismissed(
                            __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_AccessKeyDisplayDismissed(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_AccessKeyInvoked(
                            __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_AccessKeyInvoked(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITextElement4 = __uuidof(ITextElement4);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElement5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElement5[] = L"Windows.UI.Xaml.Documents.ITextElement5";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("bd9552f3-540d-58bf-b6a8-07556aeda2ea")
                    ITextElement5 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_XamlRoot(
                            ABI::Windows::UI::Xaml::IXamlRoot** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_XamlRoot(
                            ABI::Windows::UI::Xaml::IXamlRoot* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITextElement5 = __uuidof(ITextElement5);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElementFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElementFactory[] = L"Windows.UI.Xaml.Documents.ITextElementFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("35007285-cf47-4bfe-b1bc-39c93af4ae80")
                    ITextElementFactory : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ITextElementFactory = __uuidof(ITextElementFactory);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElementOverrides
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElementOverrides[] = L"Windows.UI.Xaml.Documents.ITextElementOverrides";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("0ce21ee7-4f76-4dd9-bf91-163beccf84bc")
                    ITextElementOverrides : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE OnDisconnectVisualChildren(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITextElementOverrides = __uuidof(ITextElementOverrides);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElementStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElementStatics[] = L"Windows.UI.Xaml.Documents.ITextElementStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("0a2f9b98-6c03-4470-a79b-3298a10482ce")
                    ITextElementStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FontSizeProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FontFamilyProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FontWeightProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FontStyleProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FontStretchProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CharacterSpacingProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ForegroundProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LanguageProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITextElementStatics = __uuidof(ITextElementStatics);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElementStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElementStatics2[] = L"Windows.UI.Xaml.Documents.ITextElementStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("164297b2-982b-49e1-8c03-ca43bc4d5b6d")
                    ITextElementStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsTextScaleFactorEnabledProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITextElementStatics2 = __uuidof(ITextElementStatics2);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElementStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElementStatics3[] = L"Windows.UI.Xaml.Documents.ITextElementStatics3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("cfefcfaf-0fa1-45ec-9a4e-9b33664dc8b1")
                    ITextElementStatics3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AllowFocusOnInteractionProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AccessKeyProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExitDisplayModeOnAccessKeyInvokedProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITextElementStatics3 = __uuidof(ITextElementStatics3);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElementStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElementStatics4[] = L"Windows.UI.Xaml.Documents.ITextElementStatics4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("fd8f641e-6b12-40d5-b6ef-d1bd12ac9066")
                    ITextElementStatics4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TextDecorationsProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsAccessKeyScopeProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AccessKeyScopeOwnerProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyTipPlacementModeProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyTipHorizontalOffsetProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyTipVerticalOffsetProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITextElementStatics4 = __uuidof(ITextElementStatics4);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextHighlighter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextHighlighter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextHighlighter[] = L"Windows.UI.Xaml.Documents.ITextHighlighter";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("ba6cb54b-7d75-4535-b30d-a81a00b637a4")
                    ITextHighlighter : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Ranges(
                            __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Foreground(
                            ABI::Windows::UI::Xaml::Media::IBrush** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Foreground(
                            ABI::Windows::UI::Xaml::Media::IBrush* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Background(
                            ABI::Windows::UI::Xaml::Media::IBrush** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Background(
                            ABI::Windows::UI::Xaml::Media::IBrush* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITextHighlighter = __uuidof(ITextHighlighter);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextHighlighterBase
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextHighlighterBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextHighlighterBase[] = L"Windows.UI.Xaml.Documents.ITextHighlighterBase";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("d957601a-5f0d-4cdf-9758-97e0eb95c8fa")
                    ITextHighlighterBase : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ITextHighlighterBase = __uuidof(ITextHighlighterBase);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextHighlighterBaseFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextHighlighterBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextHighlighterBaseFactory[] = L"Windows.UI.Xaml.Documents.ITextHighlighterBaseFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("9592b2d0-eadc-4c74-92c8-6e896e22506d")
                    ITextHighlighterBaseFactory : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ITextHighlighterBaseFactory = __uuidof(ITextHighlighterBaseFactory);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextHighlighterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextHighlighter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextHighlighterFactory[] = L"Windows.UI.Xaml.Documents.ITextHighlighterFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("70125461-9a8f-4fa0-b235-8ffaa507bef2")
                    ITextHighlighterFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            IInspectable* baseInterface,
                            IInspectable** innerInterface,
                            ABI::Windows::UI::Xaml::Documents::ITextHighlighter** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITextHighlighterFactory = __uuidof(ITextHighlighterFactory);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextHighlighterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextHighlighter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextHighlighterStatics[] = L"Windows.UI.Xaml.Documents.ITextHighlighterStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("b3b009c4-3a7e-49cc-ab84-29c405488765")
                    ITextHighlighterStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ForegroundProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BackgroundProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITextHighlighterStatics = __uuidof(ITextHighlighterStatics);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextPointer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextPointer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextPointer[] = L"Windows.UI.Xaml.Documents.ITextPointer";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("ac687aa1-6a41-43ff-851e-45348aa2cf7b")
                    ITextPointer : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Parent(
                            ABI::Windows::UI::Xaml::IDependencyObject** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VisualParent(
                            ABI::Windows::UI::Xaml::IFrameworkElement** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LogicalDirection(
                            ABI::Windows::UI::Xaml::Documents::LogicalDirection* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Offset(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetCharacterRect(
                            ABI::Windows::UI::Xaml::Documents::LogicalDirection direction,
                            ABI::Windows::Foundation::Rect* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetPositionAtOffset(
                            INT32 offset,
                            ABI::Windows::UI::Xaml::Documents::LogicalDirection direction,
                            ABI::Windows::UI::Xaml::Documents::ITextPointer** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITextPointer = __uuidof(ITextPointer);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITypography
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Typography
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITypography[] = L"Windows.UI.Xaml.Documents.ITypography";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("866f65d5-ea97-42ab-9288-9c01aebc7a97")
                    ITypography : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ITypography = __uuidof(ITypography);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITypographyStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Typography
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITypographyStatics[] = L"Windows.UI.Xaml.Documents.ITypographyStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("67b9ec88-6c57-4ce0-95f1-d4b9ed632fb4")
                    ITypographyStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AnnotationAlternatesProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAnnotationAlternates(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            INT32* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAnnotationAlternates(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EastAsianExpertFormsProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetEastAsianExpertForms(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetEastAsianExpertForms(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EastAsianLanguageProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetEastAsianLanguage(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::FontEastAsianLanguage* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetEastAsianLanguage(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::FontEastAsianLanguage value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EastAsianWidthsProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetEastAsianWidths(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::FontEastAsianWidths* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetEastAsianWidths(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::FontEastAsianWidths value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StandardLigaturesProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStandardLigatures(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStandardLigatures(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContextualLigaturesProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetContextualLigatures(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetContextualLigatures(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DiscretionaryLigaturesProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDiscretionaryLigatures(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetDiscretionaryLigatures(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HistoricalLigaturesProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetHistoricalLigatures(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetHistoricalLigatures(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StandardSwashesProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStandardSwashes(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            INT32* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStandardSwashes(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContextualSwashesProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetContextualSwashes(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            INT32* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetContextualSwashes(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContextualAlternatesProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetContextualAlternates(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetContextualAlternates(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticAlternatesProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticAlternates(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            INT32* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticAlternates(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet1Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet1(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet1(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet2Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet2(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet2(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet3Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet3(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet3(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet4Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet4(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet4(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet5Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet5(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet5(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet6Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet6(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet6(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet7Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet7(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet7(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet8Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet8(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet8(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet9Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet9(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet9(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet10Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet10(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet10(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet11Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet11(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet11(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet12Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet12(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet12(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet13Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet13(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet13(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet14Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet14(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet14(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet15Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet15(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet15(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet16Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet16(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet16(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet17Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet17(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet17(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet18Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet18(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet18(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet19Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet19(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet19(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StylisticSet20Property(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStylisticSet20(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetStylisticSet20(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CapitalsProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetCapitals(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::FontCapitals* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetCapitals(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::FontCapitals value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CapitalSpacingProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetCapitalSpacing(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetCapitalSpacing(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KerningProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetKerning(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetKerning(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CaseSensitiveFormsProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetCaseSensitiveForms(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetCaseSensitiveForms(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HistoricalFormsProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetHistoricalForms(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetHistoricalForms(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FractionProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetFraction(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::FontFraction* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetFraction(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::FontFraction value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NumeralStyleProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetNumeralStyle(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::FontNumeralStyle* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetNumeralStyle(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::FontNumeralStyle value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NumeralAlignmentProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetNumeralAlignment(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::FontNumeralAlignment* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetNumeralAlignment(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::FontNumeralAlignment value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SlashedZeroProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetSlashedZero(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetSlashedZero(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MathematicalGreekProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetMathematicalGreek(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetMathematicalGreek(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VariantsProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetVariants(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::FontVariants* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetVariants(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::FontVariants value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITypographyStatics = __uuidof(ITypographyStatics);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IUnderline
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Underline
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IUnderline[] = L"Windows.UI.Xaml.Documents.IUnderline";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Documents {
                    MIDL_INTERFACE("a5fa8202-61c0-47d7-93ef-bc0b577c5f26")
                    IUnderline : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IUnderline = __uuidof(IUnderline);
                } /* Documents */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Block
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IBlockStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IBlockStatics2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IBlock ** Default Interface **
 *    Windows.UI.Xaml.Documents.IBlock2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Block_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Block_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Block[] = L"Windows.UI.Xaml.Documents.Block";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.BlockCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVector`1<Windows.UI.Xaml.Documents.Block> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.UI.Xaml.Documents.Block>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_BlockCollection_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_BlockCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_BlockCollection[] = L"Windows.UI.Xaml.Documents.BlockCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Bold
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IBold ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Bold_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Bold_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Bold[] = L"Windows.UI.Xaml.Documents.Bold";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.ContactContentLinkProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IContactContentLinkProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_ContactContentLinkProvider_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_ContactContentLinkProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_ContactContentLinkProvider[] = L"Windows.UI.Xaml.Documents.ContactContentLinkProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Xaml.Documents.ContentLink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IContentLinkStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IContentLink ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_ContentLink_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_ContentLink_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_ContentLink[] = L"Windows.UI.Xaml.Documents.ContentLink";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Xaml.Documents.ContentLinkInvokedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IContentLinkInvokedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_ContentLinkInvokedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_ContentLinkInvokedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_ContentLinkInvokedEventArgs[] = L"Windows.UI.Xaml.Documents.ContentLinkInvokedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Xaml.Documents.ContentLinkProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IContentLinkProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_ContentLinkProvider_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_ContentLinkProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_ContentLinkProvider[] = L"Windows.UI.Xaml.Documents.ContentLinkProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Xaml.Documents.ContentLinkProviderCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IContentLinkProviderCollection ** Default Interface **
 *    Windows.Foundation.Collections.IVector`1<Windows.UI.Xaml.Documents.ContentLinkProvider>
 *    Windows.Foundation.Collections.IIterable`1<Windows.UI.Xaml.Documents.ContentLinkProvider>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_ContentLinkProviderCollection_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_ContentLinkProviderCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_ContentLinkProviderCollection[] = L"Windows.UI.Xaml.Documents.ContentLinkProviderCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Xaml.Documents.Glyphs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IGlyphsStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IGlyphsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IGlyphs ** Default Interface **
 *    Windows.UI.Xaml.Documents.IGlyphs2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Glyphs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Glyphs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Glyphs[] = L"Windows.UI.Xaml.Documents.Glyphs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Hyperlink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IHyperlinkStatics4 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IHyperlinkStatics5 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IHyperlinkStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IHyperlinkStatics3 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IHyperlinkStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IHyperlink ** Default Interface **
 *    Windows.UI.Xaml.Documents.IHyperlink2
 *    Windows.UI.Xaml.Documents.IHyperlink3
 *    Windows.UI.Xaml.Documents.IHyperlink4
 *    Windows.UI.Xaml.Documents.IHyperlink5
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Hyperlink_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Hyperlink_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Hyperlink[] = L"Windows.UI.Xaml.Documents.Hyperlink";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.HyperlinkClickEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IHyperlinkClickEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_HyperlinkClickEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_HyperlinkClickEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_HyperlinkClickEventArgs[] = L"Windows.UI.Xaml.Documents.HyperlinkClickEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Inline
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IInline ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Inline_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Inline_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Inline[] = L"Windows.UI.Xaml.Documents.Inline";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.InlineCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVector`1<Windows.UI.Xaml.Documents.Inline> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.UI.Xaml.Documents.Inline>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_InlineCollection_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_InlineCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_InlineCollection[] = L"Windows.UI.Xaml.Documents.InlineCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.InlineUIContainer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IInlineUIContainer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_InlineUIContainer_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_InlineUIContainer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_InlineUIContainer[] = L"Windows.UI.Xaml.Documents.InlineUIContainer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Italic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IItalic ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Italic_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Italic_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Italic[] = L"Windows.UI.Xaml.Documents.Italic";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.LineBreak
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.ILineBreak ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_LineBreak_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_LineBreak_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_LineBreak[] = L"Windows.UI.Xaml.Documents.LineBreak";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Paragraph
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IParagraphStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IParagraph ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Paragraph_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Paragraph_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Paragraph[] = L"Windows.UI.Xaml.Documents.Paragraph";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.PlaceContentLinkProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IPlaceContentLinkProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_PlaceContentLinkProvider_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_PlaceContentLinkProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_PlaceContentLinkProvider[] = L"Windows.UI.Xaml.Documents.PlaceContentLinkProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Xaml.Documents.Run
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IRunStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IRun ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Run_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Run_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Run[] = L"Windows.UI.Xaml.Documents.Run";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Span
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.ISpan ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Span_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Span_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Span[] = L"Windows.UI.Xaml.Documents.Span";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.TextElement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.ITextElementStatics3 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.ITextElementStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.ITextElementStatics4 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.ITextElementStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.ITextElement ** Default Interface **
 *    Windows.UI.Xaml.Documents.ITextElement2
 *    Windows.UI.Xaml.Documents.ITextElement3
 *    Windows.UI.Xaml.Documents.ITextElement4
 *    Windows.UI.Xaml.Documents.ITextElement5
 *    Windows.UI.Xaml.Documents.ITextElementOverrides
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_TextElement_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_TextElement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_TextElement[] = L"Windows.UI.Xaml.Documents.TextElement";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.TextHighlighter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.ITextHighlighterStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.ITextHighlighter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_TextHighlighter_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_TextHighlighter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_TextHighlighter[] = L"Windows.UI.Xaml.Documents.TextHighlighter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Xaml.Documents.TextHighlighterBase
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.ITextHighlighterBase ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_TextHighlighterBase_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_TextHighlighterBase_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_TextHighlighterBase[] = L"Windows.UI.Xaml.Documents.TextHighlighterBase";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Xaml.Documents.TextPointer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.ITextPointer ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_TextPointer_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_TextPointer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_TextPointer[] = L"Windows.UI.Xaml.Documents.TextPointer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Typography
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.ITypographyStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.ITypography ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Typography_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Typography_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Typography[] = L"Windows.UI.Xaml.Documents.Typography";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Underline
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IUnderline ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Underline_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Underline_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Underline[] = L"Windows.UI.Xaml.Documents.Underline";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2 __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2 __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2 __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2 __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2 __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3 __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4 __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5 __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2 __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3 __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4 __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5 __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic __x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak __x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider __x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2 __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3 __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4 __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5 __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2 __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3 __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4 __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline __x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline;

#endif // ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock;

typedef struct __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlockVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlockVtbl;

interface __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlockVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock __FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock;

typedef struct __FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlockVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        __FIIterator_1_Windows__CUI__CXaml__CDocuments__CBlock** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlockVtbl;

interface __FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlockVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CXaml__CDocuments__CBlock_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider;

typedef struct __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProviderVtbl;

interface __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider __FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider;

typedef struct __FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        __FIIterator_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProviderVtbl;

interface __FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline;

typedef struct __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInlineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInlineVtbl;

interface __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInlineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline __FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline;

typedef struct __FIIterable_1_Windows__CUI__CXaml__CDocuments__CInlineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        __FIIterator_1_Windows__CUI__CXaml__CDocuments__CInline** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CXaml__CDocuments__CInlineVtbl;

interface __FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CXaml__CDocuments__CInlineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CXaml__CDocuments__CInline_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CTextRange __x_ABI_CWindows_CUI_CXaml_CDocuments_CTextRange;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange;

typedef struct __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CTextRange* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CTextRange* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRangeVtbl;

interface __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange __FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange;

typedef struct __FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        __FIIterator_1_Windows__CUI__CXaml__CDocuments__CTextRange** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRangeVtbl;

interface __FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CXaml__CDocuments__CTextRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock;

typedef struct __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlockVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlockVtbl;

interface __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlockVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider;

typedef struct __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProviderVtbl;

interface __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline;

typedef struct __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInlineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInlineVtbl;

interface __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInlineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange;

typedef struct __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        UINT32 index,
        struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CTextRange* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CTextRange value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CTextRange* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRangeVtbl;

interface __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock;

typedef struct __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlockVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CBlock** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlockVtbl;

interface __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlockVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CXaml__CDocuments__CBlock_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider;

typedef struct __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProviderVtbl;

interface __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CXaml__CDocuments__CContentLinkProvider_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CXaml__CDocuments__CInline;

typedef struct __FIVector_1_Windows__CUI__CXaml__CDocuments__CInlineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CInline** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CXaml__CDocuments__CInlineVtbl;

interface __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CXaml__CDocuments__CInlineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CXaml__CDocuments__CInline_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange;

typedef struct __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        UINT32 index,
        struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CTextRange* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        __FIVectorView_1_Windows__CUI__CXaml__CDocuments__CTextRange** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CTextRange value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        UINT32 index,
        struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CTextRange value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        UINT32 index,
        struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CTextRange value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CTextRange value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CTextRange* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CTextRange* items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRangeVtbl;

interface __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* sender,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink* sender,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayDismissedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayDismissedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayDismissedEventArgs __x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayDismissedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayDismissedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* sender,
        __x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayDismissedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayRequestedEventArgs __x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayRequestedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* sender,
        __x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyDisplayRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyInvokedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyInvokedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyInvokedEventArgs __x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyInvokedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyInvokedEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* sender,
        __x_ABI_CWindows_CUI_CXaml_CInput_CIAccessKeyInvokedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CCore_CCoreCursorType __x_ABI_CWindows_CUI_CCore_CCoreCursorType;

#ifndef ____x_ABI_CWindows_CUI_CText_CIContentLinkInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CIContentLinkInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CIContentLinkInfo __x_ABI_CWindows_CUI_CText_CIContentLinkInfo;

#endif // ____x_ABI_CWindows_CUI_CText_CIContentLinkInfo_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CText_CFontStretch __x_ABI_CWindows_CUI_CText_CFontStretch;

typedef enum __x_ABI_CWindows_CUI_CText_CFontStyle __x_ABI_CWindows_CUI_CText_CFontStyle;

typedef struct __x_ABI_CWindows_CUI_CText_CFontWeight __x_ABI_CWindows_CUI_CText_CFontWeight;

typedef enum __x_ABI_CWindows_CUI_CText_CTextDecorations __x_ABI_CWindows_CUI_CText_CTextDecorations;

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIDependencyObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIDependencyObject_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIDependencyObject __x_ABI_CWindows_CUI_CXaml_CIDependencyObject;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIDependencyObject_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CXaml_CElementSoundMode __x_ABI_CWindows_CUI_CXaml_CElementSoundMode;

typedef enum __x_ABI_CWindows_CUI_CXaml_CFlowDirection __x_ABI_CWindows_CUI_CXaml_CFlowDirection;

typedef enum __x_ABI_CWindows_CUI_CXaml_CFocusState __x_ABI_CWindows_CUI_CXaml_CFocusState;

typedef enum __x_ABI_CWindows_CUI_CXaml_CFontCapitals __x_ABI_CWindows_CUI_CXaml_CFontCapitals;

typedef enum __x_ABI_CWindows_CUI_CXaml_CFontEastAsianLanguage __x_ABI_CWindows_CUI_CXaml_CFontEastAsianLanguage;

typedef enum __x_ABI_CWindows_CUI_CXaml_CFontEastAsianWidths __x_ABI_CWindows_CUI_CXaml_CFontEastAsianWidths;

typedef enum __x_ABI_CWindows_CUI_CXaml_CFontFraction __x_ABI_CWindows_CUI_CXaml_CFontFraction;

typedef enum __x_ABI_CWindows_CUI_CXaml_CFontNumeralAlignment __x_ABI_CWindows_CUI_CXaml_CFontNumeralAlignment;

typedef enum __x_ABI_CWindows_CUI_CXaml_CFontNumeralStyle __x_ABI_CWindows_CUI_CXaml_CFontNumeralStyle;

typedef enum __x_ABI_CWindows_CUI_CXaml_CFontVariants __x_ABI_CWindows_CUI_CXaml_CFontVariants;

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIFrameworkElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIFrameworkElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIFrameworkElement __x_ABI_CWindows_CUI_CXaml_CIFrameworkElement;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIFrameworkElement_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CXaml_CInput_CKeyTipPlacementMode __x_ABI_CWindows_CUI_CXaml_CInput_CKeyTipPlacementMode;

typedef enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy;

typedef enum __x_ABI_CWindows_CUI_CXaml_CLineStackingStrategy __x_ABI_CWindows_CUI_CXaml_CLineStackingStrategy;

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CMedia_CIFontFamily_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CMedia_CIFontFamily_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CMedia_CIFontFamily __x_ABI_CWindows_CUI_CXaml_CMedia_CIFontFamily;

#endif // ____x_ABI_CWindows_CUI_CXaml_CMedia_CIFontFamily_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CXaml_CMedia_CStyleSimulations __x_ABI_CWindows_CUI_CXaml_CMedia_CStyleSimulations;

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler __x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CXaml_CTextAlignment __x_ABI_CWindows_CUI_CXaml_CTextAlignment;

typedef struct __x_ABI_CWindows_CUI_CXaml_CThickness __x_ABI_CWindows_CUI_CXaml_CThickness;

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIUIElement __x_ABI_CWindows_CUI_CXaml_CIUIElement;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIXamlRoot_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIXamlRoot_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIXamlRoot __x_ABI_CWindows_CUI_CXaml_CIXamlRoot;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIXamlRoot_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CXaml_CDocuments_CLogicalDirection __x_ABI_CWindows_CUI_CXaml_CDocuments_CLogicalDirection;

typedef enum __x_ABI_CWindows_CUI_CXaml_CDocuments_CUnderlineStyle __x_ABI_CWindows_CUI_CXaml_CDocuments_CUnderlineStyle;

/*
 *
 * Struct Windows.UI.Xaml.Documents.LogicalDirection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CDocuments_CLogicalDirection
{
    LogicalDirection_Backward = 0,
    LogicalDirection_Forward = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Documents.UnderlineStyle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CDocuments_CUnderlineStyle
{
    UnderlineStyle_None = 0,
    UnderlineStyle_Single = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Documents.TextRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CTextRange
{
    INT32 StartIndex;
    INT32 Length;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IBlock
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Block
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IBlock[] = L"Windows.UI.Xaml.Documents.IBlock";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TextAlignment)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* This,
        enum __x_ABI_CWindows_CUI_CXaml_CTextAlignment* value);
    HRESULT (STDMETHODCALLTYPE* put_TextAlignment)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* This,
        enum __x_ABI_CWindows_CUI_CXaml_CTextAlignment value);
    HRESULT (STDMETHODCALLTYPE* get_LineHeight)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_LineHeight)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_LineStackingStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* This,
        enum __x_ABI_CWindows_CUI_CXaml_CLineStackingStrategy* value);
    HRESULT (STDMETHODCALLTYPE* put_LineStackingStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* This,
        enum __x_ABI_CWindows_CUI_CXaml_CLineStackingStrategy value);
    HRESULT (STDMETHODCALLTYPE* get_Margin)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* This,
        struct __x_ABI_CWindows_CUI_CXaml_CThickness* value);
    HRESULT (STDMETHODCALLTYPE* put_Margin)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock* This,
        struct __x_ABI_CWindows_CUI_CXaml_CThickness value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_get_TextAlignment(This, value) \
    ((This)->lpVtbl->get_TextAlignment(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_put_TextAlignment(This, value) \
    ((This)->lpVtbl->put_TextAlignment(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_get_LineHeight(This, value) \
    ((This)->lpVtbl->get_LineHeight(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_put_LineHeight(This, value) \
    ((This)->lpVtbl->put_LineHeight(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_get_LineStackingStrategy(This, value) \
    ((This)->lpVtbl->get_LineStackingStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_put_LineStackingStrategy(This, value) \
    ((This)->lpVtbl->put_LineStackingStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_get_Margin(This, value) \
    ((This)->lpVtbl->get_Margin(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_put_Margin(This, value) \
    ((This)->lpVtbl->put_Margin(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IBlock2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Block
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IBlock2[] = L"Windows.UI.Xaml.Documents.IBlock2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HorizontalTextAlignment)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2* This,
        enum __x_ABI_CWindows_CUI_CXaml_CTextAlignment* value);
    HRESULT (STDMETHODCALLTYPE* put_HorizontalTextAlignment)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2* This,
        enum __x_ABI_CWindows_CUI_CXaml_CTextAlignment value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_get_HorizontalTextAlignment(This, value) \
    ((This)->lpVtbl->get_HorizontalTextAlignment(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_put_HorizontalTextAlignment(This, value) \
    ((This)->lpVtbl->put_HorizontalTextAlignment(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IBlockFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Block
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IBlockFactory[] = L"Windows.UI.Xaml.Documents.IBlockFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory* This,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlock** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_CreateInstance(This, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstance(This, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IBlockStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Block
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IBlockStatics[] = L"Windows.UI.Xaml.Documents.IBlockStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TextAlignmentProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_LineHeightProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_LineStackingStrategyProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_MarginProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_get_TextAlignmentProperty(This, value) \
    ((This)->lpVtbl->get_TextAlignmentProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_get_LineHeightProperty(This, value) \
    ((This)->lpVtbl->get_LineHeightProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_get_LineStackingStrategyProperty(This, value) \
    ((This)->lpVtbl->get_LineStackingStrategyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_get_MarginProperty(This, value) \
    ((This)->lpVtbl->get_MarginProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IBlockStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Block
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IBlockStatics2[] = L"Windows.UI.Xaml.Documents.IBlockStatics2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HorizontalTextAlignmentProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_get_HorizontalTextAlignmentProperty(This, value) \
    ((This)->lpVtbl->get_HorizontalTextAlignmentProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBlockStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IBold
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Bold
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IBold[] = L"Windows.UI.Xaml.Documents.IBold";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBoldVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBoldVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBoldVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIBold_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IContactContentLinkProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.ContactContentLinkProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IContactContentLinkProvider[] = L"Windows.UI.Xaml.Documents.IContactContentLinkProvider";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProviderVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContactContentLinkProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IContentLink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.ContentLink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IContentLink[] = L"Windows.UI.Xaml.Documents.IContentLink";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Info)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        __x_ABI_CWindows_CUI_CText_CIContentLinkInfo** value);
    HRESULT (STDMETHODCALLTYPE* put_Info)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        __x_ABI_CWindows_CUI_CText_CIContentLinkInfo* value);
    HRESULT (STDMETHODCALLTYPE* get_Background)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush** value);
    HRESULT (STDMETHODCALLTYPE* put_Background)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush* value);
    HRESULT (STDMETHODCALLTYPE* get_Cursor)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreCursorType* value);
    HRESULT (STDMETHODCALLTYPE* put_Cursor)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreCursorType value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusLeft)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** value);
    HRESULT (STDMETHODCALLTYPE* put_XYFocusLeft)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusRight)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** value);
    HRESULT (STDMETHODCALLTYPE* put_XYFocusRight)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusUp)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** value);
    HRESULT (STDMETHODCALLTYPE* put_XYFocusUp)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusDown)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** value);
    HRESULT (STDMETHODCALLTYPE* put_XYFocusDown)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* value);
    HRESULT (STDMETHODCALLTYPE* get_ElementSoundMode)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        enum __x_ABI_CWindows_CUI_CXaml_CElementSoundMode* value);
    HRESULT (STDMETHODCALLTYPE* put_ElementSoundMode)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        enum __x_ABI_CWindows_CUI_CXaml_CElementSoundMode value);
    HRESULT (STDMETHODCALLTYPE* get_FocusState)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        enum __x_ABI_CWindows_CUI_CXaml_CFocusState* value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusUpNavigationStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy* value);
    HRESULT (STDMETHODCALLTYPE* put_XYFocusUpNavigationStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusDownNavigationStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy* value);
    HRESULT (STDMETHODCALLTYPE* put_XYFocusDownNavigationStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusLeftNavigationStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy* value);
    HRESULT (STDMETHODCALLTYPE* put_XYFocusLeftNavigationStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusRightNavigationStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy* value);
    HRESULT (STDMETHODCALLTYPE* put_XYFocusRightNavigationStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy value);
    HRESULT (STDMETHODCALLTYPE* get_IsTabStop)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsTabStop)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_TabIndex)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TabIndex)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* add_Invoked)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CContentLink_Windows__CUI__CXaml__CDocuments__CContentLinkInvokedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Invoked)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_GotFocus)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        __x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GotFocus)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_LostFocus)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        __x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_LostFocus)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* Focus)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink* This,
        enum __x_ABI_CWindows_CUI_CXaml_CFocusState value,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_get_Info(This, value) \
    ((This)->lpVtbl->get_Info(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_put_Info(This, value) \
    ((This)->lpVtbl->put_Info(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_get_Background(This, value) \
    ((This)->lpVtbl->get_Background(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_put_Background(This, value) \
    ((This)->lpVtbl->put_Background(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_get_Cursor(This, value) \
    ((This)->lpVtbl->get_Cursor(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_put_Cursor(This, value) \
    ((This)->lpVtbl->put_Cursor(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_get_XYFocusLeft(This, value) \
    ((This)->lpVtbl->get_XYFocusLeft(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_put_XYFocusLeft(This, value) \
    ((This)->lpVtbl->put_XYFocusLeft(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_get_XYFocusRight(This, value) \
    ((This)->lpVtbl->get_XYFocusRight(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_put_XYFocusRight(This, value) \
    ((This)->lpVtbl->put_XYFocusRight(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_get_XYFocusUp(This, value) \
    ((This)->lpVtbl->get_XYFocusUp(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_put_XYFocusUp(This, value) \
    ((This)->lpVtbl->put_XYFocusUp(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_get_XYFocusDown(This, value) \
    ((This)->lpVtbl->get_XYFocusDown(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_put_XYFocusDown(This, value) \
    ((This)->lpVtbl->put_XYFocusDown(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_get_ElementSoundMode(This, value) \
    ((This)->lpVtbl->get_ElementSoundMode(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_put_ElementSoundMode(This, value) \
    ((This)->lpVtbl->put_ElementSoundMode(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_get_FocusState(This, value) \
    ((This)->lpVtbl->get_FocusState(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_get_XYFocusUpNavigationStrategy(This, value) \
    ((This)->lpVtbl->get_XYFocusUpNavigationStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_put_XYFocusUpNavigationStrategy(This, value) \
    ((This)->lpVtbl->put_XYFocusUpNavigationStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_get_XYFocusDownNavigationStrategy(This, value) \
    ((This)->lpVtbl->get_XYFocusDownNavigationStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_put_XYFocusDownNavigationStrategy(This, value) \
    ((This)->lpVtbl->put_XYFocusDownNavigationStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_get_XYFocusLeftNavigationStrategy(This, value) \
    ((This)->lpVtbl->get_XYFocusLeftNavigationStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_put_XYFocusLeftNavigationStrategy(This, value) \
    ((This)->lpVtbl->put_XYFocusLeftNavigationStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_get_XYFocusRightNavigationStrategy(This, value) \
    ((This)->lpVtbl->get_XYFocusRightNavigationStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_put_XYFocusRightNavigationStrategy(This, value) \
    ((This)->lpVtbl->put_XYFocusRightNavigationStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_get_IsTabStop(This, value) \
    ((This)->lpVtbl->get_IsTabStop(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_put_IsTabStop(This, value) \
    ((This)->lpVtbl->put_IsTabStop(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_get_TabIndex(This, value) \
    ((This)->lpVtbl->get_TabIndex(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_put_TabIndex(This, value) \
    ((This)->lpVtbl->put_TabIndex(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_add_Invoked(This, handler, token) \
    ((This)->lpVtbl->add_Invoked(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_remove_Invoked(This, token) \
    ((This)->lpVtbl->remove_Invoked(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_add_GotFocus(This, handler, token) \
    ((This)->lpVtbl->add_GotFocus(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_remove_GotFocus(This, token) \
    ((This)->lpVtbl->remove_GotFocus(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_add_LostFocus(This, handler, token) \
    ((This)->lpVtbl->add_LostFocus(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_remove_LostFocus(This, token) \
    ((This)->lpVtbl->remove_LostFocus(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_Focus(This, value, result) \
    ((This)->lpVtbl->Focus(This, value, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLink_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IContentLinkInvokedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.ContentLinkInvokedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IContentLinkInvokedEventArgs[] = L"Windows.UI.Xaml.Documents.IContentLinkInvokedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContentLinkInfo)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs* This,
        __x_ABI_CWindows_CUI_CText_CIContentLinkInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_Handled)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Handled)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_get_ContentLinkInfo(This, value) \
    ((This)->lpVtbl->get_ContentLinkInfo(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_get_Handled(This, value) \
    ((This)->lpVtbl->get_Handled(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_put_Handled(This, value) \
    ((This)->lpVtbl->put_Handled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkInvokedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IContentLinkProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.ContentLinkProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IContentLinkProvider[] = L"Windows.UI.Xaml.Documents.IContentLinkProvider";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IContentLinkProviderCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.ContentLinkProviderCollection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IContentLinkProviderCollection[] = L"Windows.UI.Xaml.Documents.IContentLinkProviderCollection";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollectionVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderCollection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IContentLinkProviderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.ContentLinkProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IContentLinkProviderFactory[] = L"Windows.UI.Xaml.Documents.IContentLinkProviderFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory* This,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProvider** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_CreateInstance(This, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstance(This, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkProviderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IContentLinkStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.ContentLink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IContentLinkStatics[] = L"Windows.UI.Xaml.Documents.IContentLinkStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BackgroundProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_CursorProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusLeftProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusRightProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusUpProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusDownProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ElementSoundModeProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_FocusStateProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusUpNavigationStrategyProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusDownNavigationStrategyProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusLeftNavigationStrategyProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusRightNavigationStrategyProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_IsTabStopProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_TabIndexProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_get_BackgroundProperty(This, value) \
    ((This)->lpVtbl->get_BackgroundProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_get_CursorProperty(This, value) \
    ((This)->lpVtbl->get_CursorProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_get_XYFocusLeftProperty(This, value) \
    ((This)->lpVtbl->get_XYFocusLeftProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_get_XYFocusRightProperty(This, value) \
    ((This)->lpVtbl->get_XYFocusRightProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_get_XYFocusUpProperty(This, value) \
    ((This)->lpVtbl->get_XYFocusUpProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_get_XYFocusDownProperty(This, value) \
    ((This)->lpVtbl->get_XYFocusDownProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_get_ElementSoundModeProperty(This, value) \
    ((This)->lpVtbl->get_ElementSoundModeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_get_FocusStateProperty(This, value) \
    ((This)->lpVtbl->get_FocusStateProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_get_XYFocusUpNavigationStrategyProperty(This, value) \
    ((This)->lpVtbl->get_XYFocusUpNavigationStrategyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_get_XYFocusDownNavigationStrategyProperty(This, value) \
    ((This)->lpVtbl->get_XYFocusDownNavigationStrategyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_get_XYFocusLeftNavigationStrategyProperty(This, value) \
    ((This)->lpVtbl->get_XYFocusLeftNavigationStrategyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_get_XYFocusRightNavigationStrategyProperty(This, value) \
    ((This)->lpVtbl->get_XYFocusRightNavigationStrategyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_get_IsTabStopProperty(This, value) \
    ((This)->lpVtbl->get_IsTabStopProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_get_TabIndexProperty(This, value) \
    ((This)->lpVtbl->get_TabIndexProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIContentLinkStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IGlyphs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Glyphs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IGlyphs[] = L"Windows.UI.Xaml.Documents.IGlyphs";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UnicodeString)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_UnicodeString)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Indices)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Indices)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_FontUri)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_FontUri)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_StyleSimulations)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CStyleSimulations* value);
    HRESULT (STDMETHODCALLTYPE* put_StyleSimulations)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        enum __x_ABI_CWindows_CUI_CXaml_CMedia_CStyleSimulations value);
    HRESULT (STDMETHODCALLTYPE* get_FontRenderingEmSize)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_FontRenderingEmSize)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_OriginX)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_OriginX)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_OriginY)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_OriginY)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_Fill)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush** value);
    HRESULT (STDMETHODCALLTYPE* put_Fill)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_get_UnicodeString(This, value) \
    ((This)->lpVtbl->get_UnicodeString(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_put_UnicodeString(This, value) \
    ((This)->lpVtbl->put_UnicodeString(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_get_Indices(This, value) \
    ((This)->lpVtbl->get_Indices(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_put_Indices(This, value) \
    ((This)->lpVtbl->put_Indices(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_get_FontUri(This, value) \
    ((This)->lpVtbl->get_FontUri(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_put_FontUri(This, value) \
    ((This)->lpVtbl->put_FontUri(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_get_StyleSimulations(This, value) \
    ((This)->lpVtbl->get_StyleSimulations(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_put_StyleSimulations(This, value) \
    ((This)->lpVtbl->put_StyleSimulations(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_get_FontRenderingEmSize(This, value) \
    ((This)->lpVtbl->get_FontRenderingEmSize(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_put_FontRenderingEmSize(This, value) \
    ((This)->lpVtbl->put_FontRenderingEmSize(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_get_OriginX(This, value) \
    ((This)->lpVtbl->get_OriginX(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_put_OriginX(This, value) \
    ((This)->lpVtbl->put_OriginX(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_get_OriginY(This, value) \
    ((This)->lpVtbl->get_OriginY(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_put_OriginY(This, value) \
    ((This)->lpVtbl->put_OriginY(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_get_Fill(This, value) \
    ((This)->lpVtbl->get_Fill(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_put_Fill(This, value) \
    ((This)->lpVtbl->put_Fill(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IGlyphs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Glyphs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IGlyphs2[] = L"Windows.UI.Xaml.Documents.IGlyphs2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsColorFontEnabled)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsColorFontEnabled)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ColorFontPaletteIndex)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ColorFontPaletteIndex)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2* This,
        INT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_get_IsColorFontEnabled(This, value) \
    ((This)->lpVtbl->get_IsColorFontEnabled(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_put_IsColorFontEnabled(This, value) \
    ((This)->lpVtbl->put_IsColorFontEnabled(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_get_ColorFontPaletteIndex(This, value) \
    ((This)->lpVtbl->get_ColorFontPaletteIndex(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_put_ColorFontPaletteIndex(This, value) \
    ((This)->lpVtbl->put_ColorFontPaletteIndex(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IGlyphsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Glyphs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IGlyphsStatics[] = L"Windows.UI.Xaml.Documents.IGlyphsStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UnicodeStringProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_IndicesProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_FontUriProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_StyleSimulationsProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_FontRenderingEmSizeProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_OriginXProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_OriginYProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_FillProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_get_UnicodeStringProperty(This, value) \
    ((This)->lpVtbl->get_UnicodeStringProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_get_IndicesProperty(This, value) \
    ((This)->lpVtbl->get_IndicesProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_get_FontUriProperty(This, value) \
    ((This)->lpVtbl->get_FontUriProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_get_StyleSimulationsProperty(This, value) \
    ((This)->lpVtbl->get_StyleSimulationsProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_get_FontRenderingEmSizeProperty(This, value) \
    ((This)->lpVtbl->get_FontRenderingEmSizeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_get_OriginXProperty(This, value) \
    ((This)->lpVtbl->get_OriginXProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_get_OriginYProperty(This, value) \
    ((This)->lpVtbl->get_OriginYProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_get_FillProperty(This, value) \
    ((This)->lpVtbl->get_FillProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IGlyphsStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Glyphs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IGlyphsStatics2[] = L"Windows.UI.Xaml.Documents.IGlyphsStatics2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsColorFontEnabledProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ColorFontPaletteIndexProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_get_IsColorFontEnabledProperty(This, value) \
    ((This)->lpVtbl->get_IsColorFontEnabledProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_get_ColorFontPaletteIndexProperty(This, value) \
    ((This)->lpVtbl->get_ColorFontPaletteIndexProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIGlyphsStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlink[] = L"Windows.UI.Xaml.Documents.IHyperlink";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NavigateUri)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_NavigateUri)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* add_Click)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink* This,
        __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CHyperlink_Windows__CUI__CXaml__CDocuments__CHyperlinkClickEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Click)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_get_NavigateUri(This, value) \
    ((This)->lpVtbl->get_NavigateUri(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_put_NavigateUri(This, value) \
    ((This)->lpVtbl->put_NavigateUri(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_add_Click(This, handler, token) \
    ((This)->lpVtbl->add_Click(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_remove_Click(This, token) \
    ((This)->lpVtbl->remove_Click(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlink2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlink2[] = L"Windows.UI.Xaml.Documents.IHyperlink2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UnderlineStyle)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2* This,
        enum __x_ABI_CWindows_CUI_CXaml_CDocuments_CUnderlineStyle* value);
    HRESULT (STDMETHODCALLTYPE* put_UnderlineStyle)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2* This,
        enum __x_ABI_CWindows_CUI_CXaml_CDocuments_CUnderlineStyle value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_get_UnderlineStyle(This, value) \
    ((This)->lpVtbl->get_UnderlineStyle(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_put_UnderlineStyle(This, value) \
    ((This)->lpVtbl->put_UnderlineStyle(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlink3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlink3[] = L"Windows.UI.Xaml.Documents.IHyperlink3";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusLeft)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** value);
    HRESULT (STDMETHODCALLTYPE* put_XYFocusLeft)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusRight)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** value);
    HRESULT (STDMETHODCALLTYPE* put_XYFocusRight)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusUp)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** value);
    HRESULT (STDMETHODCALLTYPE* put_XYFocusUp)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusDown)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** value);
    HRESULT (STDMETHODCALLTYPE* put_XYFocusDown)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* value);
    HRESULT (STDMETHODCALLTYPE* get_ElementSoundMode)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3* This,
        enum __x_ABI_CWindows_CUI_CXaml_CElementSoundMode* value);
    HRESULT (STDMETHODCALLTYPE* put_ElementSoundMode)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3* This,
        enum __x_ABI_CWindows_CUI_CXaml_CElementSoundMode value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_get_XYFocusLeft(This, value) \
    ((This)->lpVtbl->get_XYFocusLeft(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_put_XYFocusLeft(This, value) \
    ((This)->lpVtbl->put_XYFocusLeft(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_get_XYFocusRight(This, value) \
    ((This)->lpVtbl->get_XYFocusRight(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_put_XYFocusRight(This, value) \
    ((This)->lpVtbl->put_XYFocusRight(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_get_XYFocusUp(This, value) \
    ((This)->lpVtbl->get_XYFocusUp(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_put_XYFocusUp(This, value) \
    ((This)->lpVtbl->put_XYFocusUp(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_get_XYFocusDown(This, value) \
    ((This)->lpVtbl->get_XYFocusDown(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_put_XYFocusDown(This, value) \
    ((This)->lpVtbl->put_XYFocusDown(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_get_ElementSoundMode(This, value) \
    ((This)->lpVtbl->get_ElementSoundMode(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_put_ElementSoundMode(This, value) \
    ((This)->lpVtbl->put_ElementSoundMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlink4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlink4[] = L"Windows.UI.Xaml.Documents.IHyperlink4";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FocusState)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        enum __x_ABI_CWindows_CUI_CXaml_CFocusState* value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusUpNavigationStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy* value);
    HRESULT (STDMETHODCALLTYPE* put_XYFocusUpNavigationStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusDownNavigationStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy* value);
    HRESULT (STDMETHODCALLTYPE* put_XYFocusDownNavigationStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusLeftNavigationStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy* value);
    HRESULT (STDMETHODCALLTYPE* put_XYFocusLeftNavigationStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusRightNavigationStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy* value);
    HRESULT (STDMETHODCALLTYPE* put_XYFocusRightNavigationStrategy)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CXYFocusNavigationStrategy value);
    HRESULT (STDMETHODCALLTYPE* add_GotFocus)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        __x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GotFocus)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_LostFocus)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        __x_ABI_CWindows_CUI_CXaml_CIRoutedEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_LostFocus)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* Focus)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4* This,
        enum __x_ABI_CWindows_CUI_CXaml_CFocusState value,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_get_FocusState(This, value) \
    ((This)->lpVtbl->get_FocusState(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_get_XYFocusUpNavigationStrategy(This, value) \
    ((This)->lpVtbl->get_XYFocusUpNavigationStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_put_XYFocusUpNavigationStrategy(This, value) \
    ((This)->lpVtbl->put_XYFocusUpNavigationStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_get_XYFocusDownNavigationStrategy(This, value) \
    ((This)->lpVtbl->get_XYFocusDownNavigationStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_put_XYFocusDownNavigationStrategy(This, value) \
    ((This)->lpVtbl->put_XYFocusDownNavigationStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_get_XYFocusLeftNavigationStrategy(This, value) \
    ((This)->lpVtbl->get_XYFocusLeftNavigationStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_put_XYFocusLeftNavigationStrategy(This, value) \
    ((This)->lpVtbl->put_XYFocusLeftNavigationStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_get_XYFocusRightNavigationStrategy(This, value) \
    ((This)->lpVtbl->get_XYFocusRightNavigationStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_put_XYFocusRightNavigationStrategy(This, value) \
    ((This)->lpVtbl->put_XYFocusRightNavigationStrategy(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_add_GotFocus(This, handler, token) \
    ((This)->lpVtbl->add_GotFocus(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_remove_GotFocus(This, token) \
    ((This)->lpVtbl->remove_GotFocus(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_add_LostFocus(This, handler, token) \
    ((This)->lpVtbl->add_LostFocus(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_remove_LostFocus(This, token) \
    ((This)->lpVtbl->remove_LostFocus(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_Focus(This, value, result) \
    ((This)->lpVtbl->Focus(This, value, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlink5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlink5[] = L"Windows.UI.Xaml.Documents.IHyperlink5";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsTabStop)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsTabStop)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_TabIndex)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TabIndex)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5* This,
        INT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_get_IsTabStop(This, value) \
    ((This)->lpVtbl->get_IsTabStop(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_put_IsTabStop(This, value) \
    ((This)->lpVtbl->put_IsTabStop(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_get_TabIndex(This, value) \
    ((This)->lpVtbl->get_TabIndex(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_put_TabIndex(This, value) \
    ((This)->lpVtbl->put_TabIndex(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlink5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlinkClickEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.HyperlinkClickEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlinkClickEventArgs[] = L"Windows.UI.Xaml.Documents.IHyperlinkClickEventArgs";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkClickEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlinkStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlinkStatics[] = L"Windows.UI.Xaml.Documents.IHyperlinkStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NavigateUriProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_get_NavigateUriProperty(This, value) \
    ((This)->lpVtbl->get_NavigateUriProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlinkStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlinkStatics2[] = L"Windows.UI.Xaml.Documents.IHyperlinkStatics2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UnderlineStyleProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_get_UnderlineStyleProperty(This, value) \
    ((This)->lpVtbl->get_UnderlineStyleProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlinkStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlinkStatics3[] = L"Windows.UI.Xaml.Documents.IHyperlinkStatics3";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusLeftProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusRightProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusUpProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusDownProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ElementSoundModeProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_get_XYFocusLeftProperty(This, value) \
    ((This)->lpVtbl->get_XYFocusLeftProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_get_XYFocusRightProperty(This, value) \
    ((This)->lpVtbl->get_XYFocusRightProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_get_XYFocusUpProperty(This, value) \
    ((This)->lpVtbl->get_XYFocusUpProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_get_XYFocusDownProperty(This, value) \
    ((This)->lpVtbl->get_XYFocusDownProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_get_ElementSoundModeProperty(This, value) \
    ((This)->lpVtbl->get_ElementSoundModeProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlinkStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlinkStatics4[] = L"Windows.UI.Xaml.Documents.IHyperlinkStatics4";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FocusStateProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusUpNavigationStrategyProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusDownNavigationStrategyProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusLeftNavigationStrategyProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_XYFocusRightNavigationStrategyProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_get_FocusStateProperty(This, value) \
    ((This)->lpVtbl->get_FocusStateProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_get_XYFocusUpNavigationStrategyProperty(This, value) \
    ((This)->lpVtbl->get_XYFocusUpNavigationStrategyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_get_XYFocusDownNavigationStrategyProperty(This, value) \
    ((This)->lpVtbl->get_XYFocusDownNavigationStrategyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_get_XYFocusLeftNavigationStrategyProperty(This, value) \
    ((This)->lpVtbl->get_XYFocusLeftNavigationStrategyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_get_XYFocusRightNavigationStrategyProperty(This, value) \
    ((This)->lpVtbl->get_XYFocusRightNavigationStrategyProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IHyperlinkStatics5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Hyperlink
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IHyperlinkStatics5[] = L"Windows.UI.Xaml.Documents.IHyperlinkStatics5";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsTabStopProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_TabIndexProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_get_IsTabStopProperty(This, value) \
    ((This)->lpVtbl->get_IsTabStopProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_get_TabIndexProperty(This, value) \
    ((This)->lpVtbl->get_TabIndexProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIHyperlinkStatics5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IInline
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Inline
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IInline[] = L"Windows.UI.Xaml.Documents.IInline";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IInlineFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Inline
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IInlineFactory[] = L"Windows.UI.Xaml.Documents.IInlineFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory* This,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInline** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_CreateInstance(This, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstance(This, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IInlineUIContainer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.InlineUIContainer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IInlineUIContainer[] = L"Windows.UI.Xaml.Documents.IInlineUIContainer";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Child)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement** value);
    HRESULT (STDMETHODCALLTYPE* put_Child)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainerVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_get_Child(This, value) \
    ((This)->lpVtbl->get_Child(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_put_Child(This, value) \
    ((This)->lpVtbl->put_Child(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIInlineUIContainer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IItalic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Italic
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IItalic[] = L"Windows.UI.Xaml.Documents.IItalic";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalicVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalicVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalicVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIItalic_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ILineBreak
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.LineBreak
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ILineBreak[] = L"Windows.UI.Xaml.Documents.ILineBreak";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreakVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreakVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreakVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CILineBreak_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IParagraph
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Paragraph
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IParagraph[] = L"Windows.UI.Xaml.Documents.IParagraph";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Inlines)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph* This,
        __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline** value);
    HRESULT (STDMETHODCALLTYPE* get_TextIndent)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_TextIndent)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph* This,
        DOUBLE value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_get_Inlines(This, value) \
    ((This)->lpVtbl->get_Inlines(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_get_TextIndent(This, value) \
    ((This)->lpVtbl->get_TextIndent(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_put_TextIndent(This, value) \
    ((This)->lpVtbl->put_TextIndent(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraph_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IParagraphStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Paragraph
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IParagraphStatics[] = L"Windows.UI.Xaml.Documents.IParagraphStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TextIndentProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_get_TextIndentProperty(This, value) \
    ((This)->lpVtbl->get_TextIndentProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIParagraphStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IPlaceContentLinkProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.PlaceContentLinkProvider
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IPlaceContentLinkProvider[] = L"Windows.UI.Xaml.Documents.IPlaceContentLinkProvider";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProviderVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIPlaceContentLinkProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IRun
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Run
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IRun[] = L"Windows.UI.Xaml.Documents.IRun";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Text)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_FlowDirection)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun* This,
        enum __x_ABI_CWindows_CUI_CXaml_CFlowDirection* value);
    HRESULT (STDMETHODCALLTYPE* put_FlowDirection)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun* This,
        enum __x_ABI_CWindows_CUI_CXaml_CFlowDirection value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_put_Text(This, value) \
    ((This)->lpVtbl->put_Text(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_get_FlowDirection(This, value) \
    ((This)->lpVtbl->get_FlowDirection(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_put_FlowDirection(This, value) \
    ((This)->lpVtbl->put_FlowDirection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRun_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IRunStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Run
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IRunStatics[] = L"Windows.UI.Xaml.Documents.IRunStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FlowDirectionProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_get_FlowDirectionProperty(This, value) \
    ((This)->lpVtbl->get_FlowDirectionProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIRunStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ISpan
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Span
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ISpan[] = L"Windows.UI.Xaml.Documents.ISpan";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Inlines)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan* This,
        __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline** value);
    HRESULT (STDMETHODCALLTYPE* put_Inlines)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan* This,
        __FIVector_1_Windows__CUI__CXaml__CDocuments__CInline* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_get_Inlines(This, value) \
    ((This)->lpVtbl->get_Inlines(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_put_Inlines(This, value) \
    ((This)->lpVtbl->put_Inlines(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ISpanFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Span
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ISpanFactory[] = L"Windows.UI.Xaml.Documents.ISpanFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory* This,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpan** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_CreateInstance(This, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstance(This, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CISpanFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElement[] = L"Windows.UI.Xaml.Documents.ITextElement";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FontSize)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_FontSize)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_FontFamily)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIFontFamily** value);
    HRESULT (STDMETHODCALLTYPE* put_FontFamily)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIFontFamily* value);
    HRESULT (STDMETHODCALLTYPE* get_FontWeight)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        struct __x_ABI_CWindows_CUI_CText_CFontWeight* value);
    HRESULT (STDMETHODCALLTYPE* put_FontWeight)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        struct __x_ABI_CWindows_CUI_CText_CFontWeight value);
    HRESULT (STDMETHODCALLTYPE* get_FontStyle)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        enum __x_ABI_CWindows_CUI_CText_CFontStyle* value);
    HRESULT (STDMETHODCALLTYPE* put_FontStyle)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        enum __x_ABI_CWindows_CUI_CText_CFontStyle value);
    HRESULT (STDMETHODCALLTYPE* get_FontStretch)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        enum __x_ABI_CWindows_CUI_CText_CFontStretch* value);
    HRESULT (STDMETHODCALLTYPE* put_FontStretch)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        enum __x_ABI_CWindows_CUI_CText_CFontStretch value);
    HRESULT (STDMETHODCALLTYPE* get_CharacterSpacing)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_CharacterSpacing)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Foreground)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush** value);
    HRESULT (STDMETHODCALLTYPE* put_Foreground)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush* value);
    HRESULT (STDMETHODCALLTYPE* get_Language)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Language)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ContentStart)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer** value);
    HRESULT (STDMETHODCALLTYPE* get_ContentEnd)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer** value);
    HRESULT (STDMETHODCALLTYPE* get_ElementStart)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer** value);
    HRESULT (STDMETHODCALLTYPE* get_ElementEnd)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer** value);
    HRESULT (STDMETHODCALLTYPE* FindName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement* This,
        HSTRING name,
        IInspectable** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_get_FontSize(This, value) \
    ((This)->lpVtbl->get_FontSize(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_put_FontSize(This, value) \
    ((This)->lpVtbl->put_FontSize(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_get_FontFamily(This, value) \
    ((This)->lpVtbl->get_FontFamily(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_put_FontFamily(This, value) \
    ((This)->lpVtbl->put_FontFamily(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_get_FontWeight(This, value) \
    ((This)->lpVtbl->get_FontWeight(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_put_FontWeight(This, value) \
    ((This)->lpVtbl->put_FontWeight(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_get_FontStyle(This, value) \
    ((This)->lpVtbl->get_FontStyle(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_put_FontStyle(This, value) \
    ((This)->lpVtbl->put_FontStyle(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_get_FontStretch(This, value) \
    ((This)->lpVtbl->get_FontStretch(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_put_FontStretch(This, value) \
    ((This)->lpVtbl->put_FontStretch(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_get_CharacterSpacing(This, value) \
    ((This)->lpVtbl->get_CharacterSpacing(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_put_CharacterSpacing(This, value) \
    ((This)->lpVtbl->put_CharacterSpacing(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_get_Foreground(This, value) \
    ((This)->lpVtbl->get_Foreground(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_put_Foreground(This, value) \
    ((This)->lpVtbl->put_Foreground(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_get_Language(This, value) \
    ((This)->lpVtbl->get_Language(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_put_Language(This, value) \
    ((This)->lpVtbl->put_Language(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_get_ContentStart(This, value) \
    ((This)->lpVtbl->get_ContentStart(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_get_ContentEnd(This, value) \
    ((This)->lpVtbl->get_ContentEnd(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_get_ElementStart(This, value) \
    ((This)->lpVtbl->get_ElementStart(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_get_ElementEnd(This, value) \
    ((This)->lpVtbl->get_ElementEnd(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_FindName(This, name, result) \
    ((This)->lpVtbl->FindName(This, name, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElement2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElement2[] = L"Windows.UI.Xaml.Documents.ITextElement2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsTextScaleFactorEnabled)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsTextScaleFactorEnabled)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_get_IsTextScaleFactorEnabled(This, value) \
    ((This)->lpVtbl->get_IsTextScaleFactorEnabled(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_put_IsTextScaleFactorEnabled(This, value) \
    ((This)->lpVtbl->put_IsTextScaleFactorEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElement3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElement3[] = L"Windows.UI.Xaml.Documents.ITextElement3";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AllowFocusOnInteraction)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowFocusOnInteraction)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AccessKey)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_AccessKey)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ExitDisplayModeOnAccessKeyInvoked)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ExitDisplayModeOnAccessKeyInvoked)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_get_AllowFocusOnInteraction(This, value) \
    ((This)->lpVtbl->get_AllowFocusOnInteraction(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_put_AllowFocusOnInteraction(This, value) \
    ((This)->lpVtbl->put_AllowFocusOnInteraction(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_get_AccessKey(This, value) \
    ((This)->lpVtbl->get_AccessKey(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_put_AccessKey(This, value) \
    ((This)->lpVtbl->put_AccessKey(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_get_ExitDisplayModeOnAccessKeyInvoked(This, value) \
    ((This)->lpVtbl->get_ExitDisplayModeOnAccessKeyInvoked(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_put_ExitDisplayModeOnAccessKeyInvoked(This, value) \
    ((This)->lpVtbl->put_ExitDisplayModeOnAccessKeyInvoked(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElement4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElement4[] = L"Windows.UI.Xaml.Documents.ITextElement4";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TextDecorations)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        enum __x_ABI_CWindows_CUI_CText_CTextDecorations* value);
    HRESULT (STDMETHODCALLTYPE* put_TextDecorations)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        enum __x_ABI_CWindows_CUI_CText_CTextDecorations value);
    HRESULT (STDMETHODCALLTYPE* get_IsAccessKeyScope)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsAccessKeyScope)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AccessKeyScopeOwner)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** value);
    HRESULT (STDMETHODCALLTYPE* put_AccessKeyScopeOwner)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* value);
    HRESULT (STDMETHODCALLTYPE* get_KeyTipPlacementMode)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CKeyTipPlacementMode* value);
    HRESULT (STDMETHODCALLTYPE* put_KeyTipPlacementMode)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        enum __x_ABI_CWindows_CUI_CXaml_CInput_CKeyTipPlacementMode value);
    HRESULT (STDMETHODCALLTYPE* get_KeyTipHorizontalOffset)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_KeyTipHorizontalOffset)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_KeyTipVerticalOffset)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_KeyTipVerticalOffset)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* add_AccessKeyDisplayRequested)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AccessKeyDisplayRequested)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_AccessKeyDisplayDismissed)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyDisplayDismissedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AccessKeyDisplayDismissed)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_AccessKeyInvoked)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        __FITypedEventHandler_2_Windows__CUI__CXaml__CDocuments__CTextElement_Windows__CUI__CXaml__CInput__CAccessKeyInvokedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AccessKeyInvoked)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_get_TextDecorations(This, value) \
    ((This)->lpVtbl->get_TextDecorations(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_put_TextDecorations(This, value) \
    ((This)->lpVtbl->put_TextDecorations(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_get_IsAccessKeyScope(This, value) \
    ((This)->lpVtbl->get_IsAccessKeyScope(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_put_IsAccessKeyScope(This, value) \
    ((This)->lpVtbl->put_IsAccessKeyScope(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_get_AccessKeyScopeOwner(This, value) \
    ((This)->lpVtbl->get_AccessKeyScopeOwner(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_put_AccessKeyScopeOwner(This, value) \
    ((This)->lpVtbl->put_AccessKeyScopeOwner(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_get_KeyTipPlacementMode(This, value) \
    ((This)->lpVtbl->get_KeyTipPlacementMode(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_put_KeyTipPlacementMode(This, value) \
    ((This)->lpVtbl->put_KeyTipPlacementMode(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_get_KeyTipHorizontalOffset(This, value) \
    ((This)->lpVtbl->get_KeyTipHorizontalOffset(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_put_KeyTipHorizontalOffset(This, value) \
    ((This)->lpVtbl->put_KeyTipHorizontalOffset(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_get_KeyTipVerticalOffset(This, value) \
    ((This)->lpVtbl->get_KeyTipVerticalOffset(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_put_KeyTipVerticalOffset(This, value) \
    ((This)->lpVtbl->put_KeyTipVerticalOffset(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_add_AccessKeyDisplayRequested(This, handler, token) \
    ((This)->lpVtbl->add_AccessKeyDisplayRequested(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_remove_AccessKeyDisplayRequested(This, token) \
    ((This)->lpVtbl->remove_AccessKeyDisplayRequested(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_add_AccessKeyDisplayDismissed(This, handler, token) \
    ((This)->lpVtbl->add_AccessKeyDisplayDismissed(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_remove_AccessKeyDisplayDismissed(This, token) \
    ((This)->lpVtbl->remove_AccessKeyDisplayDismissed(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_add_AccessKeyInvoked(This, handler, token) \
    ((This)->lpVtbl->add_AccessKeyInvoked(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_remove_AccessKeyInvoked(This, token) \
    ((This)->lpVtbl->remove_AccessKeyInvoked(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElement5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElement5[] = L"Windows.UI.Xaml.Documents.ITextElement5";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_XamlRoot)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5* This,
        __x_ABI_CWindows_CUI_CXaml_CIXamlRoot** value);
    HRESULT (STDMETHODCALLTYPE* put_XamlRoot)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5* This,
        __x_ABI_CWindows_CUI_CXaml_CIXamlRoot* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_get_XamlRoot(This, value) \
    ((This)->lpVtbl->get_XamlRoot(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_put_XamlRoot(This, value) \
    ((This)->lpVtbl->put_XamlRoot(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElement5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElementFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElementFactory[] = L"Windows.UI.Xaml.Documents.ITextElementFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElementOverrides
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElementOverrides[] = L"Windows.UI.Xaml.Documents.ITextElementOverrides";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverridesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OnDisconnectVisualChildren)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverridesVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverridesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_OnDisconnectVisualChildren(This) \
    ((This)->lpVtbl->OnDisconnectVisualChildren(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementOverrides_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElementStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElementStatics[] = L"Windows.UI.Xaml.Documents.ITextElementStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FontSizeProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_FontFamilyProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_FontWeightProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_FontStyleProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_FontStretchProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_CharacterSpacingProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ForegroundProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_LanguageProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_get_FontSizeProperty(This, value) \
    ((This)->lpVtbl->get_FontSizeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_get_FontFamilyProperty(This, value) \
    ((This)->lpVtbl->get_FontFamilyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_get_FontWeightProperty(This, value) \
    ((This)->lpVtbl->get_FontWeightProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_get_FontStyleProperty(This, value) \
    ((This)->lpVtbl->get_FontStyleProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_get_FontStretchProperty(This, value) \
    ((This)->lpVtbl->get_FontStretchProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_get_CharacterSpacingProperty(This, value) \
    ((This)->lpVtbl->get_CharacterSpacingProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_get_ForegroundProperty(This, value) \
    ((This)->lpVtbl->get_ForegroundProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_get_LanguageProperty(This, value) \
    ((This)->lpVtbl->get_LanguageProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElementStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElementStatics2[] = L"Windows.UI.Xaml.Documents.ITextElementStatics2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsTextScaleFactorEnabledProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_get_IsTextScaleFactorEnabledProperty(This, value) \
    ((This)->lpVtbl->get_IsTextScaleFactorEnabledProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElementStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElementStatics3[] = L"Windows.UI.Xaml.Documents.ITextElementStatics3";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AllowFocusOnInteractionProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_AccessKeyProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ExitDisplayModeOnAccessKeyInvokedProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_get_AllowFocusOnInteractionProperty(This, value) \
    ((This)->lpVtbl->get_AllowFocusOnInteractionProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_get_AccessKeyProperty(This, value) \
    ((This)->lpVtbl->get_AccessKeyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_get_ExitDisplayModeOnAccessKeyInvokedProperty(This, value) \
    ((This)->lpVtbl->get_ExitDisplayModeOnAccessKeyInvokedProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextElementStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextElement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextElementStatics4[] = L"Windows.UI.Xaml.Documents.ITextElementStatics4";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TextDecorationsProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_IsAccessKeyScopeProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_AccessKeyScopeOwnerProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_KeyTipPlacementModeProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_KeyTipHorizontalOffsetProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_KeyTipVerticalOffsetProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_get_TextDecorationsProperty(This, value) \
    ((This)->lpVtbl->get_TextDecorationsProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_get_IsAccessKeyScopeProperty(This, value) \
    ((This)->lpVtbl->get_IsAccessKeyScopeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_get_AccessKeyScopeOwnerProperty(This, value) \
    ((This)->lpVtbl->get_AccessKeyScopeOwnerProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_get_KeyTipPlacementModeProperty(This, value) \
    ((This)->lpVtbl->get_KeyTipPlacementModeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_get_KeyTipHorizontalOffsetProperty(This, value) \
    ((This)->lpVtbl->get_KeyTipHorizontalOffsetProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_get_KeyTipVerticalOffsetProperty(This, value) \
    ((This)->lpVtbl->get_KeyTipVerticalOffsetProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextElementStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextHighlighter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextHighlighter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextHighlighter[] = L"Windows.UI.Xaml.Documents.ITextHighlighter";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Ranges)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter* This,
        __FIVector_1_Windows__CUI__CXaml__CDocuments__CTextRange** value);
    HRESULT (STDMETHODCALLTYPE* get_Foreground)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush** value);
    HRESULT (STDMETHODCALLTYPE* put_Foreground)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush* value);
    HRESULT (STDMETHODCALLTYPE* get_Background)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush** value);
    HRESULT (STDMETHODCALLTYPE* put_Background)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter* This,
        __x_ABI_CWindows_CUI_CXaml_CMedia_CIBrush* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_get_Ranges(This, value) \
    ((This)->lpVtbl->get_Ranges(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_get_Foreground(This, value) \
    ((This)->lpVtbl->get_Foreground(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_put_Foreground(This, value) \
    ((This)->lpVtbl->put_Foreground(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_get_Background(This, value) \
    ((This)->lpVtbl->get_Background(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_put_Background(This, value) \
    ((This)->lpVtbl->put_Background(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextHighlighterBase
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextHighlighterBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextHighlighterBase[] = L"Windows.UI.Xaml.Documents.ITextHighlighterBase";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBase_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextHighlighterBaseFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextHighlighterBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextHighlighterBaseFactory[] = L"Windows.UI.Xaml.Documents.ITextHighlighterBaseFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterBaseFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextHighlighterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextHighlighter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextHighlighterFactory[] = L"Windows.UI.Xaml.Documents.ITextHighlighterFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory* This,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighter** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_CreateInstance(This, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstance(This, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextHighlighterStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextHighlighter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextHighlighterStatics[] = L"Windows.UI.Xaml.Documents.ITextHighlighterStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ForegroundProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_BackgroundProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_get_ForegroundProperty(This, value) \
    ((This)->lpVtbl->get_ForegroundProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_get_BackgroundProperty(This, value) \
    ((This)->lpVtbl->get_BackgroundProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextHighlighterStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITextPointer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.TextPointer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITextPointer[] = L"Windows.UI.Xaml.Documents.ITextPointer";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Parent)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** value);
    HRESULT (STDMETHODCALLTYPE* get_VisualParent)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer* This,
        __x_ABI_CWindows_CUI_CXaml_CIFrameworkElement** value);
    HRESULT (STDMETHODCALLTYPE* get_LogicalDirection)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer* This,
        enum __x_ABI_CWindows_CUI_CXaml_CDocuments_CLogicalDirection* value);
    HRESULT (STDMETHODCALLTYPE* get_Offset)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* GetCharacterRect)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer* This,
        enum __x_ABI_CWindows_CUI_CXaml_CDocuments_CLogicalDirection direction,
        struct __x_ABI_CWindows_CFoundation_CRect* result);
    HRESULT (STDMETHODCALLTYPE* GetPositionAtOffset)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer* This,
        INT32 offset,
        enum __x_ABI_CWindows_CUI_CXaml_CDocuments_CLogicalDirection direction,
        __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointerVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_get_Parent(This, value) \
    ((This)->lpVtbl->get_Parent(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_get_VisualParent(This, value) \
    ((This)->lpVtbl->get_VisualParent(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_get_LogicalDirection(This, value) \
    ((This)->lpVtbl->get_LogicalDirection(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_get_Offset(This, value) \
    ((This)->lpVtbl->get_Offset(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_GetCharacterRect(This, direction, result) \
    ((This)->lpVtbl->GetCharacterRect(This, direction, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_GetPositionAtOffset(This, offset, direction, result) \
    ((This)->lpVtbl->GetPositionAtOffset(This, offset, direction, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITextPointer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITypography
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Typography
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITypography[] = L"Windows.UI.Xaml.Documents.ITypography";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypography_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.ITypographyStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Typography
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_ITypographyStatics[] = L"Windows.UI.Xaml.Documents.ITypographyStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AnnotationAlternatesProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetAnnotationAlternates)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* SetAnnotationAlternates)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_EastAsianExpertFormsProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetEastAsianExpertForms)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetEastAsianExpertForms)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_EastAsianLanguageProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetEastAsianLanguage)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CFontEastAsianLanguage* result);
    HRESULT (STDMETHODCALLTYPE* SetEastAsianLanguage)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CFontEastAsianLanguage value);
    HRESULT (STDMETHODCALLTYPE* get_EastAsianWidthsProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetEastAsianWidths)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CFontEastAsianWidths* result);
    HRESULT (STDMETHODCALLTYPE* SetEastAsianWidths)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CFontEastAsianWidths value);
    HRESULT (STDMETHODCALLTYPE* get_StandardLigaturesProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStandardLigatures)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStandardLigatures)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ContextualLigaturesProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetContextualLigatures)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetContextualLigatures)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_DiscretionaryLigaturesProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetDiscretionaryLigatures)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetDiscretionaryLigatures)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_HistoricalLigaturesProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetHistoricalLigatures)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetHistoricalLigatures)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StandardSwashesProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStandardSwashes)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* SetStandardSwashes)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ContextualSwashesProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetContextualSwashes)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* SetContextualSwashes)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ContextualAlternatesProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetContextualAlternates)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetContextualAlternates)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticAlternatesProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticAlternates)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticAlternates)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet1Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet1)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet1)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet2Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet2)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet2)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet3Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet3)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet3)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet4Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet4)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet4)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet5Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet5)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet5)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet6Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet6)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet6)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet7Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet7)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet7)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet8Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet8)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet8)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet9Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet9)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet9)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet10Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet10)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet10)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet11Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet11)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet11)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet12Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet12)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet12)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet13Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet13)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet13)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet14Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet14)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet14)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet15Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet15)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet15)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet16Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet16)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet16)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet17Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet17)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet17)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet18Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet18)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet18)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet19Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet19)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet19)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_StylisticSet20Property)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetStylisticSet20)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetStylisticSet20)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CapitalsProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetCapitals)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CFontCapitals* result);
    HRESULT (STDMETHODCALLTYPE* SetCapitals)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CFontCapitals value);
    HRESULT (STDMETHODCALLTYPE* get_CapitalSpacingProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetCapitalSpacing)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetCapitalSpacing)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_KerningProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetKerning)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetKerning)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CaseSensitiveFormsProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetCaseSensitiveForms)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetCaseSensitiveForms)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_HistoricalFormsProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetHistoricalForms)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetHistoricalForms)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_FractionProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetFraction)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CFontFraction* result);
    HRESULT (STDMETHODCALLTYPE* SetFraction)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CFontFraction value);
    HRESULT (STDMETHODCALLTYPE* get_NumeralStyleProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetNumeralStyle)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CFontNumeralStyle* result);
    HRESULT (STDMETHODCALLTYPE* SetNumeralStyle)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CFontNumeralStyle value);
    HRESULT (STDMETHODCALLTYPE* get_NumeralAlignmentProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetNumeralAlignment)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CFontNumeralAlignment* result);
    HRESULT (STDMETHODCALLTYPE* SetNumeralAlignment)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CFontNumeralAlignment value);
    HRESULT (STDMETHODCALLTYPE* get_SlashedZeroProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetSlashedZero)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetSlashedZero)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_MathematicalGreekProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetMathematicalGreek)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetMathematicalGreek)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_VariantsProperty)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetVariants)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CFontVariants* result);
    HRESULT (STDMETHODCALLTYPE* SetVariants)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CFontVariants value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_AnnotationAlternatesProperty(This, value) \
    ((This)->lpVtbl->get_AnnotationAlternatesProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetAnnotationAlternates(This, element, result) \
    ((This)->lpVtbl->GetAnnotationAlternates(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetAnnotationAlternates(This, element, value) \
    ((This)->lpVtbl->SetAnnotationAlternates(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_EastAsianExpertFormsProperty(This, value) \
    ((This)->lpVtbl->get_EastAsianExpertFormsProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetEastAsianExpertForms(This, element, result) \
    ((This)->lpVtbl->GetEastAsianExpertForms(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetEastAsianExpertForms(This, element, value) \
    ((This)->lpVtbl->SetEastAsianExpertForms(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_EastAsianLanguageProperty(This, value) \
    ((This)->lpVtbl->get_EastAsianLanguageProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetEastAsianLanguage(This, element, result) \
    ((This)->lpVtbl->GetEastAsianLanguage(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetEastAsianLanguage(This, element, value) \
    ((This)->lpVtbl->SetEastAsianLanguage(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_EastAsianWidthsProperty(This, value) \
    ((This)->lpVtbl->get_EastAsianWidthsProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetEastAsianWidths(This, element, result) \
    ((This)->lpVtbl->GetEastAsianWidths(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetEastAsianWidths(This, element, value) \
    ((This)->lpVtbl->SetEastAsianWidths(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StandardLigaturesProperty(This, value) \
    ((This)->lpVtbl->get_StandardLigaturesProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStandardLigatures(This, element, result) \
    ((This)->lpVtbl->GetStandardLigatures(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStandardLigatures(This, element, value) \
    ((This)->lpVtbl->SetStandardLigatures(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_ContextualLigaturesProperty(This, value) \
    ((This)->lpVtbl->get_ContextualLigaturesProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetContextualLigatures(This, element, result) \
    ((This)->lpVtbl->GetContextualLigatures(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetContextualLigatures(This, element, value) \
    ((This)->lpVtbl->SetContextualLigatures(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_DiscretionaryLigaturesProperty(This, value) \
    ((This)->lpVtbl->get_DiscretionaryLigaturesProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetDiscretionaryLigatures(This, element, result) \
    ((This)->lpVtbl->GetDiscretionaryLigatures(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetDiscretionaryLigatures(This, element, value) \
    ((This)->lpVtbl->SetDiscretionaryLigatures(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_HistoricalLigaturesProperty(This, value) \
    ((This)->lpVtbl->get_HistoricalLigaturesProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetHistoricalLigatures(This, element, result) \
    ((This)->lpVtbl->GetHistoricalLigatures(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetHistoricalLigatures(This, element, value) \
    ((This)->lpVtbl->SetHistoricalLigatures(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StandardSwashesProperty(This, value) \
    ((This)->lpVtbl->get_StandardSwashesProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStandardSwashes(This, element, result) \
    ((This)->lpVtbl->GetStandardSwashes(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStandardSwashes(This, element, value) \
    ((This)->lpVtbl->SetStandardSwashes(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_ContextualSwashesProperty(This, value) \
    ((This)->lpVtbl->get_ContextualSwashesProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetContextualSwashes(This, element, result) \
    ((This)->lpVtbl->GetContextualSwashes(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetContextualSwashes(This, element, value) \
    ((This)->lpVtbl->SetContextualSwashes(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_ContextualAlternatesProperty(This, value) \
    ((This)->lpVtbl->get_ContextualAlternatesProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetContextualAlternates(This, element, result) \
    ((This)->lpVtbl->GetContextualAlternates(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetContextualAlternates(This, element, value) \
    ((This)->lpVtbl->SetContextualAlternates(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticAlternatesProperty(This, value) \
    ((This)->lpVtbl->get_StylisticAlternatesProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticAlternates(This, element, result) \
    ((This)->lpVtbl->GetStylisticAlternates(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticAlternates(This, element, value) \
    ((This)->lpVtbl->SetStylisticAlternates(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet1Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet1Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet1(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet1(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet1(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet1(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet2Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet2Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet2(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet2(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet2(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet2(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet3Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet3Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet3(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet3(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet3(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet3(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet4Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet4Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet4(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet4(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet4(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet4(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet5Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet5Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet5(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet5(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet5(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet5(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet6Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet6Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet6(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet6(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet6(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet6(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet7Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet7Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet7(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet7(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet7(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet7(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet8Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet8Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet8(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet8(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet8(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet8(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet9Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet9Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet9(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet9(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet9(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet9(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet10Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet10Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet10(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet10(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet10(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet10(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet11Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet11Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet11(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet11(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet11(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet11(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet12Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet12Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet12(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet12(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet12(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet12(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet13Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet13Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet13(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet13(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet13(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet13(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet14Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet14Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet14(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet14(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet14(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet14(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet15Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet15Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet15(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet15(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet15(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet15(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet16Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet16Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet16(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet16(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet16(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet16(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet17Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet17Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet17(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet17(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet17(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet17(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet18Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet18Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet18(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet18(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet18(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet18(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet19Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet19Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet19(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet19(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet19(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet19(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_StylisticSet20Property(This, value) \
    ((This)->lpVtbl->get_StylisticSet20Property(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetStylisticSet20(This, element, result) \
    ((This)->lpVtbl->GetStylisticSet20(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetStylisticSet20(This, element, value) \
    ((This)->lpVtbl->SetStylisticSet20(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_CapitalsProperty(This, value) \
    ((This)->lpVtbl->get_CapitalsProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetCapitals(This, element, result) \
    ((This)->lpVtbl->GetCapitals(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetCapitals(This, element, value) \
    ((This)->lpVtbl->SetCapitals(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_CapitalSpacingProperty(This, value) \
    ((This)->lpVtbl->get_CapitalSpacingProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetCapitalSpacing(This, element, result) \
    ((This)->lpVtbl->GetCapitalSpacing(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetCapitalSpacing(This, element, value) \
    ((This)->lpVtbl->SetCapitalSpacing(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_KerningProperty(This, value) \
    ((This)->lpVtbl->get_KerningProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetKerning(This, element, result) \
    ((This)->lpVtbl->GetKerning(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetKerning(This, element, value) \
    ((This)->lpVtbl->SetKerning(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_CaseSensitiveFormsProperty(This, value) \
    ((This)->lpVtbl->get_CaseSensitiveFormsProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetCaseSensitiveForms(This, element, result) \
    ((This)->lpVtbl->GetCaseSensitiveForms(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetCaseSensitiveForms(This, element, value) \
    ((This)->lpVtbl->SetCaseSensitiveForms(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_HistoricalFormsProperty(This, value) \
    ((This)->lpVtbl->get_HistoricalFormsProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetHistoricalForms(This, element, result) \
    ((This)->lpVtbl->GetHistoricalForms(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetHistoricalForms(This, element, value) \
    ((This)->lpVtbl->SetHistoricalForms(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_FractionProperty(This, value) \
    ((This)->lpVtbl->get_FractionProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetFraction(This, element, result) \
    ((This)->lpVtbl->GetFraction(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetFraction(This, element, value) \
    ((This)->lpVtbl->SetFraction(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_NumeralStyleProperty(This, value) \
    ((This)->lpVtbl->get_NumeralStyleProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetNumeralStyle(This, element, result) \
    ((This)->lpVtbl->GetNumeralStyle(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetNumeralStyle(This, element, value) \
    ((This)->lpVtbl->SetNumeralStyle(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_NumeralAlignmentProperty(This, value) \
    ((This)->lpVtbl->get_NumeralAlignmentProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetNumeralAlignment(This, element, result) \
    ((This)->lpVtbl->GetNumeralAlignment(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetNumeralAlignment(This, element, value) \
    ((This)->lpVtbl->SetNumeralAlignment(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_SlashedZeroProperty(This, value) \
    ((This)->lpVtbl->get_SlashedZeroProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetSlashedZero(This, element, result) \
    ((This)->lpVtbl->GetSlashedZero(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetSlashedZero(This, element, value) \
    ((This)->lpVtbl->SetSlashedZero(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_MathematicalGreekProperty(This, value) \
    ((This)->lpVtbl->get_MathematicalGreekProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetMathematicalGreek(This, element, result) \
    ((This)->lpVtbl->GetMathematicalGreek(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetMathematicalGreek(This, element, value) \
    ((This)->lpVtbl->SetMathematicalGreek(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_get_VariantsProperty(This, value) \
    ((This)->lpVtbl->get_VariantsProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_GetVariants(This, element, result) \
    ((This)->lpVtbl->GetVariants(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_SetVariants(This, element, value) \
    ((This)->lpVtbl->SetVariants(This, element, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CITypographyStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Documents.IUnderline
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Documents.Underline
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Documents_IUnderline[] = L"Windows.UI.Xaml.Documents.IUnderline";
typedef struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderlineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderlineVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderlineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CDocuments_CIUnderline_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Block
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IBlockStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IBlockStatics2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IBlock ** Default Interface **
 *    Windows.UI.Xaml.Documents.IBlock2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Block_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Block_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Block[] = L"Windows.UI.Xaml.Documents.Block";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.BlockCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVector`1<Windows.UI.Xaml.Documents.Block> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.UI.Xaml.Documents.Block>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_BlockCollection_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_BlockCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_BlockCollection[] = L"Windows.UI.Xaml.Documents.BlockCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Bold
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IBold ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Bold_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Bold_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Bold[] = L"Windows.UI.Xaml.Documents.Bold";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.ContactContentLinkProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IContactContentLinkProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_ContactContentLinkProvider_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_ContactContentLinkProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_ContactContentLinkProvider[] = L"Windows.UI.Xaml.Documents.ContactContentLinkProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Xaml.Documents.ContentLink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IContentLinkStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IContentLink ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_ContentLink_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_ContentLink_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_ContentLink[] = L"Windows.UI.Xaml.Documents.ContentLink";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Xaml.Documents.ContentLinkInvokedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IContentLinkInvokedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_ContentLinkInvokedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_ContentLinkInvokedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_ContentLinkInvokedEventArgs[] = L"Windows.UI.Xaml.Documents.ContentLinkInvokedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Xaml.Documents.ContentLinkProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IContentLinkProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_ContentLinkProvider_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_ContentLinkProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_ContentLinkProvider[] = L"Windows.UI.Xaml.Documents.ContentLinkProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Xaml.Documents.ContentLinkProviderCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IContentLinkProviderCollection ** Default Interface **
 *    Windows.Foundation.Collections.IVector`1<Windows.UI.Xaml.Documents.ContentLinkProvider>
 *    Windows.Foundation.Collections.IIterable`1<Windows.UI.Xaml.Documents.ContentLinkProvider>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_ContentLinkProviderCollection_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_ContentLinkProviderCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_ContentLinkProviderCollection[] = L"Windows.UI.Xaml.Documents.ContentLinkProviderCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Xaml.Documents.Glyphs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IGlyphsStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IGlyphsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IGlyphs ** Default Interface **
 *    Windows.UI.Xaml.Documents.IGlyphs2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Glyphs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Glyphs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Glyphs[] = L"Windows.UI.Xaml.Documents.Glyphs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Hyperlink
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IHyperlinkStatics4 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IHyperlinkStatics5 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IHyperlinkStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IHyperlinkStatics3 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IHyperlinkStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IHyperlink ** Default Interface **
 *    Windows.UI.Xaml.Documents.IHyperlink2
 *    Windows.UI.Xaml.Documents.IHyperlink3
 *    Windows.UI.Xaml.Documents.IHyperlink4
 *    Windows.UI.Xaml.Documents.IHyperlink5
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Hyperlink_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Hyperlink_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Hyperlink[] = L"Windows.UI.Xaml.Documents.Hyperlink";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.HyperlinkClickEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IHyperlinkClickEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_HyperlinkClickEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_HyperlinkClickEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_HyperlinkClickEventArgs[] = L"Windows.UI.Xaml.Documents.HyperlinkClickEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Inline
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IInline ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Inline_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Inline_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Inline[] = L"Windows.UI.Xaml.Documents.Inline";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.InlineCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IVector`1<Windows.UI.Xaml.Documents.Inline> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.UI.Xaml.Documents.Inline>
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_InlineCollection_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_InlineCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_InlineCollection[] = L"Windows.UI.Xaml.Documents.InlineCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.InlineUIContainer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IInlineUIContainer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_InlineUIContainer_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_InlineUIContainer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_InlineUIContainer[] = L"Windows.UI.Xaml.Documents.InlineUIContainer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Italic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IItalic ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Italic_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Italic_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Italic[] = L"Windows.UI.Xaml.Documents.Italic";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.LineBreak
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.ILineBreak ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_LineBreak_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_LineBreak_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_LineBreak[] = L"Windows.UI.Xaml.Documents.LineBreak";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Paragraph
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IParagraphStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IParagraph ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Paragraph_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Paragraph_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Paragraph[] = L"Windows.UI.Xaml.Documents.Paragraph";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.PlaceContentLinkProvider
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IPlaceContentLinkProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_PlaceContentLinkProvider_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_PlaceContentLinkProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_PlaceContentLinkProvider[] = L"Windows.UI.Xaml.Documents.PlaceContentLinkProvider";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Xaml.Documents.Run
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.IRunStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IRun ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Run_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Run_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Run[] = L"Windows.UI.Xaml.Documents.Run";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Span
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.ISpan ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Span_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Span_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Span[] = L"Windows.UI.Xaml.Documents.Span";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.TextElement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.ITextElementStatics3 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.ITextElementStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.ITextElementStatics4 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Documents.ITextElementStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.ITextElement ** Default Interface **
 *    Windows.UI.Xaml.Documents.ITextElement2
 *    Windows.UI.Xaml.Documents.ITextElement3
 *    Windows.UI.Xaml.Documents.ITextElement4
 *    Windows.UI.Xaml.Documents.ITextElement5
 *    Windows.UI.Xaml.Documents.ITextElementOverrides
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_TextElement_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_TextElement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_TextElement[] = L"Windows.UI.Xaml.Documents.TextElement";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.TextHighlighter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.ITextHighlighterStatics interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.ITextHighlighter ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_TextHighlighter_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_TextHighlighter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_TextHighlighter[] = L"Windows.UI.Xaml.Documents.TextHighlighter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Xaml.Documents.TextHighlighterBase
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.ITextHighlighterBase ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_TextHighlighterBase_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_TextHighlighterBase_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_TextHighlighterBase[] = L"Windows.UI.Xaml.Documents.TextHighlighterBase";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.UI.Xaml.Documents.TextPointer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.ITextPointer ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_TextPointer_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_TextPointer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_TextPointer[] = L"Windows.UI.Xaml.Documents.TextPointer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Typography
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Documents.ITypographyStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.ITypography ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Typography_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Typography_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Typography[] = L"Windows.UI.Xaml.Documents.Typography";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Documents.Underline
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Documents.IUnderline ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Documents_Underline_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Documents_Underline_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Documents_Underline[] = L"Windows.UI.Xaml.Documents.Underline";
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
#endif // __windows2Eui2Examl2Edocuments_p_h__

#endif // __windows2Eui2Examl2Edocuments_h__
