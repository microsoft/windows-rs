
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
#ifndef __windows2Eui2Examl2Eprinting_h__
#define __windows2Eui2Examl2Eprinting_h__
#ifndef __windows2Eui2Examl2Eprinting_p_h__
#define __windows2Eui2Examl2Eprinting_p_h__


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
#include "Windows.Graphics.Printing.h"
#include "Windows.UI.Xaml.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    interface IAddPagesEventHandler;
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler ABI::Windows::UI::Xaml::Printing::IAddPagesEventHandler

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    interface IGetPreviewPageEventHandler;
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler ABI::Windows::UI::Xaml::Printing::IGetPreviewPageEventHandler

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    interface IPaginateEventHandler;
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler ABI::Windows::UI::Xaml::Printing::IPaginateEventHandler

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    interface IAddPagesEventArgs;
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs ABI::Windows::UI::Xaml::Printing::IAddPagesEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    interface IGetPreviewPageEventArgs;
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs ABI::Windows::UI::Xaml::Printing::IGetPreviewPageEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    interface IPaginateEventArgs;
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs ABI::Windows::UI::Xaml::Printing::IPaginateEventArgs

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    interface IPrintDocument;
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument ABI::Windows::UI::Xaml::Printing::IPrintDocument

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    interface IPrintDocumentFactory;
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory ABI::Windows::UI::Xaml::Printing::IPrintDocumentFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    interface IPrintDocumentStatics;
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics ABI::Windows::UI::Xaml::Printing::IPrintDocumentStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintDocumentSource;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource ABI::Windows::Graphics::Printing::IPrintDocumentSource

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                class PrintTaskOptions;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTaskOptionsCore;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore ABI::Windows::Graphics::Printing::IPrintTaskOptionsCore

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_FWD_DEFINED__

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
                namespace Printing {
                    typedef enum PreviewPageCountType : int PreviewPageCountType;
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    class AddPagesEventArgs;
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    class GetPreviewPageEventArgs;
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    class PaginateEventArgs;
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    class PrintDocument;
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Xaml.Printing.PreviewPageCountType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    enum PreviewPageCountType : int
                    {
                        PreviewPageCountType_Final = 0,
                        PreviewPageCountType_Intermediate = 1,
                    };
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Printing.AddPagesEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    MIDL_INTERFACE("d4b57970-57a0-4209-847c-c093b54bc729")
                    IAddPagesEventHandler : public IUnknown
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Invoke(
                            IInspectable* sender,
                            ABI::Windows::UI::Xaml::Printing::IAddPagesEventArgs* e
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAddPagesEventHandler = __uuidof(IAddPagesEventHandler);
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Printing.GetPreviewPageEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    MIDL_INTERFACE("ccb3e9ed-9c11-4e50-ab49-e98086bbfdef")
                    IGetPreviewPageEventHandler : public IUnknown
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Invoke(
                            IInspectable* sender,
                            ABI::Windows::UI::Xaml::Printing::IGetPreviewPageEventArgs* e
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGetPreviewPageEventHandler = __uuidof(IGetPreviewPageEventHandler);
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Printing.PaginateEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    MIDL_INTERFACE("0cc05b61-811b-4a32-9965-13eb78dbb01b")
                    IPaginateEventHandler : public IUnknown
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Invoke(
                            IInspectable* sender,
                            ABI::Windows::UI::Xaml::Printing::IPaginateEventArgs* e
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPaginateEventHandler = __uuidof(IPaginateEventHandler);
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Printing.IAddPagesEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Printing.AddPagesEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Printing_IAddPagesEventArgs[] = L"Windows.UI.Xaml.Printing.IAddPagesEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    MIDL_INTERFACE("e2e52be5-056c-4420-9795-cb3526ce0c20")
                    IAddPagesEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PrintTaskOptions(
                            ABI::Windows::Graphics::Printing::IPrintTaskOptionsCore** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAddPagesEventArgs = __uuidof(IAddPagesEventArgs);
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Printing.IGetPreviewPageEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Printing.GetPreviewPageEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Printing_IGetPreviewPageEventArgs[] = L"Windows.UI.Xaml.Printing.IGetPreviewPageEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    MIDL_INTERFACE("a43d703d-dea9-4df6-a7ed-35049cd485c7")
                    IGetPreviewPageEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PageNumber(
                            INT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGetPreviewPageEventArgs = __uuidof(IGetPreviewPageEventArgs);
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Printing.IPaginateEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Printing.PaginateEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Printing_IPaginateEventArgs[] = L"Windows.UI.Xaml.Printing.IPaginateEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    MIDL_INTERFACE("ed945fd6-79ab-42b7-930a-3d6e09011d21")
                    IPaginateEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PrintTaskOptions(
                            ABI::Windows::Graphics::Printing::IPrintTaskOptionsCore** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CurrentPreviewPageNumber(
                            INT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPaginateEventArgs = __uuidof(IPaginateEventArgs);
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Printing.IPrintDocument
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Printing.PrintDocument
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Printing_IPrintDocument[] = L"Windows.UI.Xaml.Printing.IPrintDocument";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    MIDL_INTERFACE("e44327c3-a999-485b-b1d8-72dc517821e6")
                    IPrintDocument : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentSource(
                            ABI::Windows::Graphics::Printing::IPrintDocumentSource** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_Paginate(
                            ABI::Windows::UI::Xaml::Printing::IPaginateEventHandler* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_Paginate(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_GetPreviewPage(
                            ABI::Windows::UI::Xaml::Printing::IGetPreviewPageEventHandler* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_GetPreviewPage(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_AddPages(
                            ABI::Windows::UI::Xaml::Printing::IAddPagesEventHandler* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_AddPages(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AddPage(
                            ABI::Windows::UI::Xaml::IUIElement* pageVisual
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AddPagesComplete(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPreviewPageCount(
                            INT32 count,
                            ABI::Windows::UI::Xaml::Printing::PreviewPageCountType type
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPreviewPage(
                            INT32 pageNumber,
                            ABI::Windows::UI::Xaml::IUIElement* pageVisual
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE InvalidatePreview(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintDocument = __uuidof(IPrintDocument);
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Printing.IPrintDocumentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Printing.PrintDocument
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Printing_IPrintDocumentFactory[] = L"Windows.UI.Xaml.Printing.IPrintDocumentFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    MIDL_INTERFACE("fb87b18f-2606-4a2f-99d4-a7cdbc35d7c7")
                    IPrintDocumentFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            IInspectable* baseInterface,
                            IInspectable** innerInterface,
                            ABI::Windows::UI::Xaml::Printing::IPrintDocument** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintDocumentFactory = __uuidof(IPrintDocumentFactory);
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Printing.IPrintDocumentStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Printing.PrintDocument
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Printing_IPrintDocumentStatics[] = L"Windows.UI.Xaml.Printing.IPrintDocumentStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Printing {
                    MIDL_INTERFACE("fd970a3c-b152-49e0-a6bd-6aa6477e43c7")
                    IPrintDocumentStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DocumentSourceProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintDocumentStatics = __uuidof(IPrintDocumentStatics);
                } /* Printing */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Printing.AddPagesEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Printing.IAddPagesEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Printing_AddPagesEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Printing_AddPagesEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Printing_AddPagesEventArgs[] = L"Windows.UI.Xaml.Printing.AddPagesEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Printing.GetPreviewPageEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Printing.IGetPreviewPageEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Printing_GetPreviewPageEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Printing_GetPreviewPageEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Printing_GetPreviewPageEventArgs[] = L"Windows.UI.Xaml.Printing.GetPreviewPageEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Printing.PaginateEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Printing.IPaginateEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Printing_PaginateEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Printing_PaginateEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Printing_PaginateEventArgs[] = L"Windows.UI.Xaml.Printing.PaginateEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Printing.PrintDocument
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Printing.IPrintDocumentStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Printing.IPrintDocument ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Printing_PrintDocument_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Printing_PrintDocument_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Printing_PrintDocument[] = L"Windows.UI.Xaml.Printing.PrintDocument";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler;

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler;

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler;

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs;

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument;

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIUIElement __x_ABI_CWindows_CUI_CXaml_CIUIElement;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CXaml_CPrinting_CPreviewPageCountType __x_ABI_CWindows_CUI_CXaml_CPrinting_CPreviewPageCountType;

/*
 *
 * Struct Windows.UI.Xaml.Printing.PreviewPageCountType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CPrinting_CPreviewPageCountType
{
    PreviewPageCountType_Final = 0,
    PreviewPageCountType_Intermediate = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Printing.AddPagesEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Printing.GetPreviewPageEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.UI.Xaml.Printing.PaginateEventHandler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler_INTERFACE_DEFINED__
typedef struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler* This,
        IInspectable* sender,
        __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandlerVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Printing.IAddPagesEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Printing.AddPagesEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Printing_IAddPagesEventArgs[] = L"Windows.UI.Xaml.Printing.IAddPagesEventArgs";
typedef struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PrintTaskOptions)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_get_PrintTaskOptions(This, value) \
    ((This)->lpVtbl->get_PrintTaskOptions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Printing.IGetPreviewPageEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Printing.GetPreviewPageEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Printing_IGetPreviewPageEventArgs[] = L"Windows.UI.Xaml.Printing.IGetPreviewPageEventArgs";
typedef struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PageNumber)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_get_PageNumber(This, value) \
    ((This)->lpVtbl->get_PageNumber(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Printing.IPaginateEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Printing.PaginateEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Printing_IPaginateEventArgs[] = L"Windows.UI.Xaml.Printing.IPaginateEventArgs";
typedef struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PrintTaskOptions)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore** value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentPreviewPageNumber)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_get_PrintTaskOptions(This, value) \
    ((This)->lpVtbl->get_PrintTaskOptions(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_get_CurrentPreviewPageNumber(This, value) \
    ((This)->lpVtbl->get_CurrentPreviewPageNumber(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Printing.IPrintDocument
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Printing.PrintDocument
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Printing_IPrintDocument[] = L"Windows.UI.Xaml.Printing.IPrintDocument";
typedef struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DocumentSource)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintDocumentSource** value);
    HRESULT (STDMETHODCALLTYPE* add_Paginate)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This,
        __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPaginateEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Paginate)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_GetPreviewPage)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This,
        __x_ABI_CWindows_CUI_CXaml_CPrinting_CIGetPreviewPageEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GetPreviewPage)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_AddPages)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This,
        __x_ABI_CWindows_CUI_CXaml_CPrinting_CIAddPagesEventHandler* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AddPages)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* AddPage)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* pageVisual);
    HRESULT (STDMETHODCALLTYPE* AddPagesComplete)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This);
    HRESULT (STDMETHODCALLTYPE* SetPreviewPageCount)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This,
        INT32 count,
        enum __x_ABI_CWindows_CUI_CXaml_CPrinting_CPreviewPageCountType type);
    HRESULT (STDMETHODCALLTYPE* SetPreviewPage)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This,
        INT32 pageNumber,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* pageVisual);
    HRESULT (STDMETHODCALLTYPE* InvalidatePreview)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_get_DocumentSource(This, value) \
    ((This)->lpVtbl->get_DocumentSource(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_add_Paginate(This, handler, token) \
    ((This)->lpVtbl->add_Paginate(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_remove_Paginate(This, token) \
    ((This)->lpVtbl->remove_Paginate(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_add_GetPreviewPage(This, handler, token) \
    ((This)->lpVtbl->add_GetPreviewPage(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_remove_GetPreviewPage(This, token) \
    ((This)->lpVtbl->remove_GetPreviewPage(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_add_AddPages(This, handler, token) \
    ((This)->lpVtbl->add_AddPages(This, handler, token))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_remove_AddPages(This, token) \
    ((This)->lpVtbl->remove_AddPages(This, token))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_AddPage(This, pageVisual) \
    ((This)->lpVtbl->AddPage(This, pageVisual))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_AddPagesComplete(This) \
    ((This)->lpVtbl->AddPagesComplete(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_SetPreviewPageCount(This, count, type) \
    ((This)->lpVtbl->SetPreviewPageCount(This, count, type))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_SetPreviewPage(This, pageNumber, pageVisual) \
    ((This)->lpVtbl->SetPreviewPage(This, pageNumber, pageVisual))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_InvalidatePreview(This) \
    ((This)->lpVtbl->InvalidatePreview(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Printing.IPrintDocumentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Printing.PrintDocument
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Printing_IPrintDocumentFactory[] = L"Windows.UI.Xaml.Printing.IPrintDocumentFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory* This,
        IInspectable* baseInterface,
        IInspectable** innerInterface,
        __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocument** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_CreateInstance(This, baseInterface, innerInterface, value) \
    ((This)->lpVtbl->CreateInstance(This, baseInterface, innerInterface, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Printing.IPrintDocumentStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Printing.PrintDocument
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Printing_IPrintDocumentStatics[] = L"Windows.UI.Xaml.Printing.IPrintDocumentStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DocumentSourceProperty)(__x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_get_DocumentSourceProperty(This, value) \
    ((This)->lpVtbl->get_DocumentSourceProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CPrinting_CIPrintDocumentStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Printing.AddPagesEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Printing.IAddPagesEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Printing_AddPagesEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Printing_AddPagesEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Printing_AddPagesEventArgs[] = L"Windows.UI.Xaml.Printing.AddPagesEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Printing.GetPreviewPageEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Printing.IGetPreviewPageEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Printing_GetPreviewPageEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Printing_GetPreviewPageEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Printing_GetPreviewPageEventArgs[] = L"Windows.UI.Xaml.Printing.GetPreviewPageEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Printing.PaginateEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Printing.IPaginateEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Printing_PaginateEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Printing_PaginateEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Printing_PaginateEventArgs[] = L"Windows.UI.Xaml.Printing.PaginateEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Printing.PrintDocument
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Printing.IPrintDocumentStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Printing.IPrintDocument ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Printing_PrintDocument_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Printing_PrintDocument_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Printing_PrintDocument[] = L"Windows.UI.Xaml.Printing.PrintDocument";
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
#endif // __windows2Eui2Examl2Eprinting_p_h__

#endif // __windows2Eui2Examl2Eprinting_h__
