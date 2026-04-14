
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
#ifndef __windows2Eui2Etext2Ecore_h__
#define __windows2Eui2Etext2Ecore_h__
#ifndef __windows2Eui2Etext2Ecore_p_h__
#define __windows2Eui2Etext2Ecore_p_h__


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

#if !defined(WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION)
#define WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_GLOBALIZATION_GLOBALIZATIONJAPANESEPHONETICANALYZERCONTRACT_VERSION)

#if !defined(WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION)
#define WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_UI_VIEWMANAGEMENT_VIEWMANAGEMENTVIEWSCALINGCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Globalization.h"
#include "Windows.UI.Text.h"
#include "Windows.UI.ViewManagement.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextCompositionCompletedEventArgs;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs ABI::Windows::UI::Text::Core::ICoreTextCompositionCompletedEventArgs

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextCompositionSegment;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment ABI::Windows::UI::Text::Core::ICoreTextCompositionSegment

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextCompositionStartedEventArgs;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs ABI::Windows::UI::Text::Core::ICoreTextCompositionStartedEventArgs

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextEditContext;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext ABI::Windows::UI::Text::Core::ICoreTextEditContext

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextEditContext2;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2 ABI::Windows::UI::Text::Core::ICoreTextEditContext2

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextFormatUpdatingEventArgs;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs ABI::Windows::UI::Text::Core::ICoreTextFormatUpdatingEventArgs

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextLayoutBounds;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds ABI::Windows::UI::Text::Core::ICoreTextLayoutBounds

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextLayoutRequest;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest ABI::Windows::UI::Text::Core::ICoreTextLayoutRequest

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextLayoutRequest2;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2 ABI::Windows::UI::Text::Core::ICoreTextLayoutRequest2

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextLayoutRequestedEventArgs;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs ABI::Windows::UI::Text::Core::ICoreTextLayoutRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextSelectionRequest;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest ABI::Windows::UI::Text::Core::ICoreTextSelectionRequest

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextSelectionRequestedEventArgs;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs ABI::Windows::UI::Text::Core::ICoreTextSelectionRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextSelectionUpdatingEventArgs;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs ABI::Windows::UI::Text::Core::ICoreTextSelectionUpdatingEventArgs

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextServicesManager;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager ABI::Windows::UI::Text::Core::ICoreTextServicesManager

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextServicesManagerStatics;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics ABI::Windows::UI::Text::Core::ICoreTextServicesManagerStatics

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextServicesStatics;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics ABI::Windows::UI::Text::Core::ICoreTextServicesStatics

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextTextRequest;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest ABI::Windows::UI::Text::Core::ICoreTextTextRequest

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextTextRequestedEventArgs;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs ABI::Windows::UI::Text::Core::ICoreTextTextRequestedEventArgs

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    interface ICoreTextTextUpdatingEventArgs;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs ABI::Windows::UI::Text::Core::ICoreTextTextUpdatingEventArgs

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    class CoreTextCompositionSegment;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_USE
#define DEF___FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("39b4528d-2370-57fa-b5d4-b5a2079a7cea"))
IIterator<ABI::Windows::UI::Text::Core::CoreTextCompositionSegment*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextCompositionSegment*, ABI::Windows::UI::Text::Core::ICoreTextCompositionSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Text.Core.CoreTextCompositionSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Text::Core::CoreTextCompositionSegment*> __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_t;
#define __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_USE
#define DEF___FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("38372bd2-d3fe-5ad2-9d39-d166b68e78e7"))
IIterable<ABI::Windows::UI::Text::Core::CoreTextCompositionSegment*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextCompositionSegment*, ABI::Windows::UI::Text::Core::ICoreTextCompositionSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Text.Core.CoreTextCompositionSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Text::Core::CoreTextCompositionSegment*> __FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_t;
#define __FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_USE
#define DEF___FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("214b64ff-cf4d-5dd4-932a-7bc66e69036e"))
IVectorView<ABI::Windows::UI::Text::Core::CoreTextCompositionSegment*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextCompositionSegment*, ABI::Windows::UI::Text::Core::ICoreTextCompositionSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Text.Core.CoreTextCompositionSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Text::Core::CoreTextCompositionSegment*> __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_t;
#define __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                typedef enum UnderlineType : int UnderlineType;
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CUI__CText__CUnderlineType_USE
#define DEF___FIReference_1_Windows__CUI__CText__CUnderlineType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1b63ec17-7b2b-59fe-ab9d-b60ea4f9c9b8"))
IReference<enum ABI::Windows::UI::Text::UnderlineType> : IReference_impl<enum ABI::Windows::UI::Text::UnderlineType>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.UI.Text.UnderlineType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<enum ABI::Windows::UI::Text::UnderlineType> __FIReference_1_Windows__CUI__CText__CUnderlineType_t;
#define __FIReference_1_Windows__CUI__CText__CUnderlineType ABI::Windows::Foundation::__FIReference_1_Windows__CUI__CText__CUnderlineType_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CUI__CText__CUnderlineType_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace ViewManagement {
                typedef enum UIElementType : int UIElementType;
            } /* ViewManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CUI__CViewManagement__CUIElementType_USE
#define DEF___FIReference_1_Windows__CUI__CViewManagement__CUIElementType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e17e08c9-7deb-51d1-8487-334eb3fe4691"))
IReference<enum ABI::Windows::UI::ViewManagement::UIElementType> : IReference_impl<enum ABI::Windows::UI::ViewManagement::UIElementType>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.UI.ViewManagement.UIElementType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<enum ABI::Windows::UI::ViewManagement::UIElementType> __FIReference_1_Windows__CUI__CViewManagement__CUIElementType_t;
#define __FIReference_1_Windows__CUI__CViewManagement__CUIElementType ABI::Windows::Foundation::__FIReference_1_Windows__CUI__CViewManagement__CUIElementType_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CUI__CViewManagement__CUIElementType_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    class CoreTextEditContext;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ef53b467-c472-5b59-a827-38adc3a9d326"))
ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::ICoreTextEditContext*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Text.Core.CoreTextEditContext, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    class CoreTextCompositionCompletedEventArgs;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a2d7059e-68ed-5260-8d8e-1dcf3d25d663"))
ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::CoreTextCompositionCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::ICoreTextEditContext*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextCompositionCompletedEventArgs*, ABI::Windows::UI::Text::Core::ICoreTextCompositionCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Text.Core.CoreTextEditContext, Windows.UI.Text.Core.CoreTextCompositionCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::CoreTextCompositionCompletedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    class CoreTextCompositionStartedEventArgs;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e671d625-5b59-57e6-a92e-40009507853a"))
ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::CoreTextCompositionStartedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::ICoreTextEditContext*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextCompositionStartedEventArgs*, ABI::Windows::UI::Text::Core::ICoreTextCompositionStartedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Text.Core.CoreTextEditContext, Windows.UI.Text.Core.CoreTextCompositionStartedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::CoreTextCompositionStartedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    class CoreTextFormatUpdatingEventArgs;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3e4ef599-4cc2-5248-bf2d-13f17613b0a6"))
ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::CoreTextFormatUpdatingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::ICoreTextEditContext*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextFormatUpdatingEventArgs*, ABI::Windows::UI::Text::Core::ICoreTextFormatUpdatingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Text.Core.CoreTextEditContext, Windows.UI.Text.Core.CoreTextFormatUpdatingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::CoreTextFormatUpdatingEventArgs*> __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    class CoreTextLayoutRequestedEventArgs;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1a5f5596-561c-57f6-a4ff-cb85013c6544"))
ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::CoreTextLayoutRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::ICoreTextEditContext*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextLayoutRequestedEventArgs*, ABI::Windows::UI::Text::Core::ICoreTextLayoutRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Text.Core.CoreTextEditContext, Windows.UI.Text.Core.CoreTextLayoutRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::CoreTextLayoutRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    class CoreTextSelectionRequestedEventArgs;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("882e51e1-f4a3-57e5-9392-6a8c38579181"))
ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::CoreTextSelectionRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::ICoreTextEditContext*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextSelectionRequestedEventArgs*, ABI::Windows::UI::Text::Core::ICoreTextSelectionRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Text.Core.CoreTextEditContext, Windows.UI.Text.Core.CoreTextSelectionRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::CoreTextSelectionRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    class CoreTextSelectionUpdatingEventArgs;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6b140a40-d461-555a-b6eb-5dbb8e2101e5"))
ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::CoreTextSelectionUpdatingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::ICoreTextEditContext*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextSelectionUpdatingEventArgs*, ABI::Windows::UI::Text::Core::ICoreTextSelectionUpdatingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Text.Core.CoreTextEditContext, Windows.UI.Text.Core.CoreTextSelectionUpdatingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::CoreTextSelectionUpdatingEventArgs*> __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    class CoreTextTextRequestedEventArgs;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c7e08176-4134-50b7-bc73-729e9f9ad22a"))
ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::CoreTextTextRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::ICoreTextEditContext*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextTextRequestedEventArgs*, ABI::Windows::UI::Text::Core::ICoreTextTextRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Text.Core.CoreTextEditContext, Windows.UI.Text.Core.CoreTextTextRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::CoreTextTextRequestedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    class CoreTextTextUpdatingEventArgs;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fd896a84-df7c-50d5-9167-58f616bddb6e"))
ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::CoreTextTextUpdatingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::ICoreTextEditContext*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextTextUpdatingEventArgs*, ABI::Windows::UI::Text::Core::ICoreTextTextUpdatingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Text.Core.CoreTextEditContext, Windows.UI.Text.Core.CoreTextTextUpdatingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextEditContext*, ABI::Windows::UI::Text::Core::CoreTextTextUpdatingEventArgs*> __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    class CoreTextServicesManager;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("112fb01d-260a-51c6-9198-5db3e6e9ef3d"))
ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextServicesManager*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Text::Core::CoreTextServicesManager*, ABI::Windows::UI::Text::Core::ICoreTextServicesManager*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Text.Core.CoreTextServicesManager, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Text::Core::CoreTextServicesManager*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
        namespace Foundation {
            typedef struct Rect Rect;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Globalization {
            class Language;
        } /* Globalization */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Globalization {
            interface ILanguage;
        } /* Globalization */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGlobalization_CILanguage ABI::Windows::Globalization::ILanguage

#endif // ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__

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
                    typedef enum CoreTextFormatUpdatingResult : int CoreTextFormatUpdatingResult;
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
                    typedef enum CoreTextInputPaneDisplayPolicy : int CoreTextInputPaneDisplayPolicy;
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
                    typedef enum CoreTextSelectionUpdatingResult : int CoreTextSelectionUpdatingResult;
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
                    typedef enum CoreTextTextUpdatingResult : int CoreTextTextUpdatingResult;
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
                namespace Core {
                    class CoreTextLayoutBounds;
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
                    class CoreTextLayoutRequest;
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
                    class CoreTextSelectionRequest;
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
                    class CoreTextTextRequest;
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Text.Core.CoreTextFormatUpdatingReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    enum CoreTextFormatUpdatingReason : int
                    {
                        CoreTextFormatUpdatingReason_None = 0,
                        CoreTextFormatUpdatingReason_CompositionUnconverted = 1,
                        CoreTextFormatUpdatingReason_CompositionConverted = 2,
                        CoreTextFormatUpdatingReason_CompositionTargetUnconverted = 3,
                        CoreTextFormatUpdatingReason_CompositionTargetConverted = 4,
                    };
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Text.Core.CoreTextFormatUpdatingResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    enum CoreTextFormatUpdatingResult : int
                    {
                        CoreTextFormatUpdatingResult_Succeeded = 0,
                        CoreTextFormatUpdatingResult_Failed = 1,
                    };
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Text.Core.CoreTextInputPaneDisplayPolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    enum CoreTextInputPaneDisplayPolicy : int
                    {
                        CoreTextInputPaneDisplayPolicy_Automatic = 0,
                        CoreTextInputPaneDisplayPolicy_Manual = 1,
                    };
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Text.Core.CoreTextInputScope
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    enum CoreTextInputScope : int
                    {
                        CoreTextInputScope_Default = 0,
                        CoreTextInputScope_Url = 1,
                        CoreTextInputScope_FilePath = 2,
                        CoreTextInputScope_FileName = 3,
                        CoreTextInputScope_EmailUserName = 4,
                        CoreTextInputScope_EmailAddress = 5,
                        CoreTextInputScope_UserName = 6,
                        CoreTextInputScope_PersonalFullName = 7,
                        CoreTextInputScope_PersonalNamePrefix = 8,
                        CoreTextInputScope_PersonalGivenName = 9,
                        CoreTextInputScope_PersonalMiddleName = 10,
                        CoreTextInputScope_PersonalSurname = 11,
                        CoreTextInputScope_PersonalNameSuffix = 12,
                        CoreTextInputScope_Address = 13,
                        CoreTextInputScope_AddressPostalCode = 14,
                        CoreTextInputScope_AddressStreet = 15,
                        CoreTextInputScope_AddressStateOrProvince = 16,
                        CoreTextInputScope_AddressCity = 17,
                        CoreTextInputScope_AddressCountryName = 18,
                        CoreTextInputScope_AddressCountryShortName = 19,
                        CoreTextInputScope_CurrencyAmountAndSymbol = 20,
                        CoreTextInputScope_CurrencyAmount = 21,
                        CoreTextInputScope_Date = 22,
                        CoreTextInputScope_DateMonth = 23,
                        CoreTextInputScope_DateDay = 24,
                        CoreTextInputScope_DateYear = 25,
                        CoreTextInputScope_DateMonthName = 26,
                        CoreTextInputScope_DateDayName = 27,
                        CoreTextInputScope_Number = 29,
                        CoreTextInputScope_SingleCharacter = 30,
                        CoreTextInputScope_Password = 31,
                        CoreTextInputScope_TelephoneNumber = 32,
                        CoreTextInputScope_TelephoneCountryCode = 33,
                        CoreTextInputScope_TelephoneAreaCode = 34,
                        CoreTextInputScope_TelephoneLocalNumber = 35,
                        CoreTextInputScope_Time = 36,
                        CoreTextInputScope_TimeHour = 37,
                        CoreTextInputScope_TimeMinuteOrSecond = 38,
                        CoreTextInputScope_NumberFullWidth = 39,
                        CoreTextInputScope_AlphanumericHalfWidth = 40,
                        CoreTextInputScope_AlphanumericFullWidth = 41,
                        CoreTextInputScope_CurrencyChinese = 42,
                        CoreTextInputScope_Bopomofo = 43,
                        CoreTextInputScope_Hiragana = 44,
                        CoreTextInputScope_KatakanaHalfWidth = 45,
                        CoreTextInputScope_KatakanaFullWidth = 46,
                        CoreTextInputScope_Hanja = 47,
                        CoreTextInputScope_HangulHalfWidth = 48,
                        CoreTextInputScope_HangulFullWidth = 49,
                        CoreTextInputScope_Search = 50,
                        CoreTextInputScope_Formula = 51,
                        CoreTextInputScope_SearchIncremental = 52,
                        CoreTextInputScope_ChineseHalfWidth = 53,
                        CoreTextInputScope_ChineseFullWidth = 54,
                        CoreTextInputScope_NativeScript = 55,
                        CoreTextInputScope_Text = 57,
                        CoreTextInputScope_Chat = 58,
                        CoreTextInputScope_NameOrPhoneNumber = 59,
                        CoreTextInputScope_EmailUserNameOrAddress = 60,
                        CoreTextInputScope_Private = 61,
                        CoreTextInputScope_Maps = 62,
                        CoreTextInputScope_PasswordNumeric = 63,
                        CoreTextInputScope_FormulaNumber = 67,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        CoreTextInputScope_ChatWithoutEmoji = 68,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                        CoreTextInputScope_Digits = 28,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                        CoreTextInputScope_PinNumeric = 64,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                        CoreTextInputScope_PinAlphanumeric = 65,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                    };
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Text.Core.CoreTextSelectionUpdatingResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    enum CoreTextSelectionUpdatingResult : int
                    {
                        CoreTextSelectionUpdatingResult_Succeeded = 0,
                        CoreTextSelectionUpdatingResult_Failed = 1,
                    };
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Text.Core.CoreTextTextUpdatingResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    enum CoreTextTextUpdatingResult : int
                    {
                        CoreTextTextUpdatingResult_Succeeded = 0,
                        CoreTextTextUpdatingResult_Failed = 1,
                    };
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Text.Core.CoreTextRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    struct CoreTextRange
                    {
                        INT32 StartCaretPosition;
                        INT32 EndCaretPosition;
                    };
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextCompositionCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextCompositionCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextCompositionCompletedEventArgs[] = L"Windows.UI.Text.Core.ICoreTextCompositionCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("1f34ebb6-b79f-4121-a5e7-fda9b8616e30")
                    ICoreTextCompositionCompletedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsCanceled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CompositionSegments(
                            __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextCompositionCompletedEventArgs = __uuidof(ICoreTextCompositionCompletedEventArgs);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextCompositionSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextCompositionSegment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextCompositionSegment[] = L"Windows.UI.Text.Core.ICoreTextCompositionSegment";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("776c6bd9-4ead-4da7-8f47-3a88b523cc34")
                    ICoreTextCompositionSegment : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PreconversionString(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Range(
                            ABI::Windows::UI::Text::Core::CoreTextRange* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextCompositionSegment = __uuidof(ICoreTextCompositionSegment);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextCompositionStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextCompositionStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextCompositionStartedEventArgs[] = L"Windows.UI.Text.Core.ICoreTextCompositionStartedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("276b16a9-64e7-4ab0-bc4b-a02d73835bfb")
                    ICoreTextCompositionStartedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsCanceled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextCompositionStartedEventArgs = __uuidof(ICoreTextCompositionStartedEventArgs);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextEditContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextEditContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextEditContext[] = L"Windows.UI.Text.Core.ICoreTextEditContext";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("bf6608af-4041-47c3-b263-a918eb5eaef2")
                    ICoreTextEditContext : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Name(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_InputScope(
                            ABI::Windows::UI::Text::Core::CoreTextInputScope* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_InputScope(
                            ABI::Windows::UI::Text::Core::CoreTextInputScope value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsReadOnly(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsReadOnly(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_InputPaneDisplayPolicy(
                            ABI::Windows::UI::Text::Core::CoreTextInputPaneDisplayPolicy* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_InputPaneDisplayPolicy(
                            ABI::Windows::UI::Text::Core::CoreTextInputPaneDisplayPolicy value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_TextRequested(
                            __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_TextRequested(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SelectionRequested(
                            __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SelectionRequested(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_LayoutRequested(
                            __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_LayoutRequested(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_TextUpdating(
                            __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_TextUpdating(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SelectionUpdating(
                            __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SelectionUpdating(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_FormatUpdating(
                            __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_FormatUpdating(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_CompositionStarted(
                            __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_CompositionStarted(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_CompositionCompleted(
                            __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_CompositionCompleted(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_FocusRemoved(
                            __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_FocusRemoved(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE NotifyFocusEnter(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE NotifyFocusLeave(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE NotifyTextChanged(
                            ABI::Windows::UI::Text::Core::CoreTextRange modifiedRange,
                            INT32 newLength,
                            ABI::Windows::UI::Text::Core::CoreTextRange newSelection
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE NotifySelectionChanged(
                            ABI::Windows::UI::Text::Core::CoreTextRange selection
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE NotifyLayoutChanged(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextEditContext = __uuidof(ICoreTextEditContext);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextEditContext2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextEditContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextEditContext2[] = L"Windows.UI.Text.Core.ICoreTextEditContext2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("b1867dbb-083b-49e1-b281-2b35d62bf466")
                    ICoreTextEditContext2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_NotifyFocusLeaveCompleted(
                            __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_NotifyFocusLeaveCompleted(
                            EventRegistrationToken cookie
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextEditContext2 = __uuidof(ICoreTextEditContext2);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextFormatUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextFormatUpdatingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextFormatUpdatingEventArgs[] = L"Windows.UI.Text.Core.ICoreTextFormatUpdatingEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("7310bd33-b4a8-43b1-b37b-0724d4aca7ab")
                    ICoreTextFormatUpdatingEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Range(
                            ABI::Windows::UI::Text::Core::CoreTextRange* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TextColor(
                            __FIReference_1_Windows__CUI__CViewManagement__CUIElementType** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BackgroundColor(
                            __FIReference_1_Windows__CUI__CViewManagement__CUIElementType** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UnderlineColor(
                            __FIReference_1_Windows__CUI__CViewManagement__CUIElementType** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UnderlineType(
                            __FIReference_1_Windows__CUI__CText__CUnderlineType** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Reason(
                            ABI::Windows::UI::Text::Core::CoreTextFormatUpdatingReason* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Result(
                            ABI::Windows::UI::Text::Core::CoreTextFormatUpdatingResult* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Result(
                            ABI::Windows::UI::Text::Core::CoreTextFormatUpdatingResult value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsCanceled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextFormatUpdatingEventArgs = __uuidof(ICoreTextFormatUpdatingEventArgs);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextLayoutBounds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextLayoutBounds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextLayoutBounds[] = L"Windows.UI.Text.Core.ICoreTextLayoutBounds";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("e972c974-4436-4917-80d0-a525e4ca6780")
                    ICoreTextLayoutBounds : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TextBounds(
                            ABI::Windows::Foundation::Rect* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_TextBounds(
                            ABI::Windows::Foundation::Rect value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ControlBounds(
                            ABI::Windows::Foundation::Rect* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ControlBounds(
                            ABI::Windows::Foundation::Rect value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextLayoutBounds = __uuidof(ICoreTextLayoutBounds);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextLayoutRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextLayoutRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextLayoutRequest[] = L"Windows.UI.Text.Core.ICoreTextLayoutRequest";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("2555a8cc-51fd-4f03-98bf-ac78174d68e0")
                    ICoreTextLayoutRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Range(
                            ABI::Windows::UI::Text::Core::CoreTextRange* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LayoutBounds(
                            ABI::Windows::UI::Text::Core::ICoreTextLayoutBounds** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsCanceled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextLayoutRequest = __uuidof(ICoreTextLayoutRequest);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextLayoutRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextLayoutRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextLayoutRequest2[] = L"Windows.UI.Text.Core.ICoreTextLayoutRequest2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("676de624-cd3d-4bcd-bf01-7f7110954511")
                    ICoreTextLayoutRequest2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_LayoutBoundsVisualPixels(
                            ABI::Windows::UI::Text::Core::ICoreTextLayoutBounds** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextLayoutRequest2 = __uuidof(ICoreTextLayoutRequest2);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextLayoutRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextLayoutRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextLayoutRequestedEventArgs[] = L"Windows.UI.Text.Core.ICoreTextLayoutRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("b1dc6ae0-9a7b-4e9e-a566-4a6b5f8ad676")
                    ICoreTextLayoutRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::UI::Text::Core::ICoreTextLayoutRequest** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextLayoutRequestedEventArgs = __uuidof(ICoreTextLayoutRequestedEventArgs);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextSelectionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextSelectionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextSelectionRequest[] = L"Windows.UI.Text.Core.ICoreTextSelectionRequest";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("f0a70403-208b-4301-883c-74ca7485fd8d")
                    ICoreTextSelectionRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Selection(
                            ABI::Windows::UI::Text::Core::CoreTextRange* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Selection(
                            ABI::Windows::UI::Text::Core::CoreTextRange value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsCanceled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextSelectionRequest = __uuidof(ICoreTextSelectionRequest);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextSelectionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextSelectionRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextSelectionRequestedEventArgs[] = L"Windows.UI.Text.Core.ICoreTextSelectionRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("13c6682b-f614-421a-8f4b-9ec8a5a37fcd")
                    ICoreTextSelectionRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::UI::Text::Core::ICoreTextSelectionRequest** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextSelectionRequestedEventArgs = __uuidof(ICoreTextSelectionRequestedEventArgs);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextSelectionUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextSelectionUpdatingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextSelectionUpdatingEventArgs[] = L"Windows.UI.Text.Core.ICoreTextSelectionUpdatingEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("d445839f-fe7f-4bd5-8a26-0922c1b3e639")
                    ICoreTextSelectionUpdatingEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Selection(
                            ABI::Windows::UI::Text::Core::CoreTextRange* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Result(
                            ABI::Windows::UI::Text::Core::CoreTextSelectionUpdatingResult* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Result(
                            ABI::Windows::UI::Text::Core::CoreTextSelectionUpdatingResult value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsCanceled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextSelectionUpdatingEventArgs = __uuidof(ICoreTextSelectionUpdatingEventArgs);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextServicesManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextServicesManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextServicesManager[] = L"Windows.UI.Text.Core.ICoreTextServicesManager";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("c2507d83-6e0a-4a8a-bdf8-1948874854ba")
                    ICoreTextServicesManager : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InputLanguage(
                            ABI::Windows::Globalization::ILanguage** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_InputLanguageChanged(
                            __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_InputLanguageChanged(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateEditContext(
                            ABI::Windows::UI::Text::Core::ICoreTextEditContext** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextServicesManager = __uuidof(ICoreTextServicesManager);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextServicesManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextServicesManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextServicesManagerStatics[] = L"Windows.UI.Text.Core.ICoreTextServicesManagerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("1520a388-e2cf-4d65-aeb9-b32d86fe39b9")
                    ICoreTextServicesManagerStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                            ABI::Windows::UI::Text::Core::ICoreTextServicesManager** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextServicesManagerStatics = __uuidof(ICoreTextServicesManagerStatics);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextServicesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextServicesConstants
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextServicesStatics[] = L"Windows.UI.Text.Core.ICoreTextServicesStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("91859a46-eccf-47a4-8ae7-098a9c6fbb15")
                    ICoreTextServicesStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_HiddenCharacter(
                            WCHAR* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextServicesStatics = __uuidof(ICoreTextServicesStatics);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextTextRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextTextRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextTextRequest[] = L"Windows.UI.Text.Core.ICoreTextTextRequest";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("50d950a9-f51e-4cc1-8ca1-e6346d1a61be")
                    ICoreTextTextRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Range(
                            ABI::Windows::UI::Text::Core::CoreTextRange* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Text(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Text(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsCanceled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextTextRequest = __uuidof(ICoreTextTextRequest);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextTextRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextTextRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextTextRequestedEventArgs[] = L"Windows.UI.Text.Core.ICoreTextTextRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("f096a2d0-41c6-4c02-8b1a-d953b00cabb3")
                    ICoreTextTextRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::UI::Text::Core::ICoreTextTextRequest** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextTextRequestedEventArgs = __uuidof(ICoreTextTextRequestedEventArgs);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextTextUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextTextUpdatingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextTextUpdatingEventArgs[] = L"Windows.UI.Text.Core.ICoreTextTextUpdatingEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Text {
                namespace Core {
                    MIDL_INTERFACE("eea7918d-cc2b-4f03-8ff6-02fd217db450")
                    ICoreTextTextUpdatingEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Range(
                            ABI::Windows::UI::Text::Core::CoreTextRange* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Text(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NewSelection(
                            ABI::Windows::UI::Text::Core::CoreTextRange* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_InputLanguage(
                            ABI::Windows::Globalization::ILanguage** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Result(
                            ABI::Windows::UI::Text::Core::CoreTextTextUpdatingResult* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Result(
                            ABI::Windows::UI::Text::Core::CoreTextTextUpdatingResult value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsCanceled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ICoreTextTextUpdatingEventArgs = __uuidof(ICoreTextTextUpdatingEventArgs);
                } /* Core */
            } /* Text */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextCompositionCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextCompositionCompletedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextCompositionCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextCompositionCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextCompositionCompletedEventArgs[] = L"Windows.UI.Text.Core.CoreTextCompositionCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextCompositionSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextCompositionSegment ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextCompositionSegment_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextCompositionSegment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextCompositionSegment[] = L"Windows.UI.Text.Core.CoreTextCompositionSegment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextCompositionStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextCompositionStartedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextCompositionStartedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextCompositionStartedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextCompositionStartedEventArgs[] = L"Windows.UI.Text.Core.CoreTextCompositionStartedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextEditContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextEditContext ** Default Interface **
 *    Windows.UI.Text.Core.ICoreTextEditContext2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextEditContext_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextEditContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextEditContext[] = L"Windows.UI.Text.Core.CoreTextEditContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextFormatUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextFormatUpdatingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextFormatUpdatingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextFormatUpdatingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextFormatUpdatingEventArgs[] = L"Windows.UI.Text.Core.CoreTextFormatUpdatingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextLayoutBounds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextLayoutBounds ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextLayoutBounds_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextLayoutBounds_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextLayoutBounds[] = L"Windows.UI.Text.Core.CoreTextLayoutBounds";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextLayoutRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextLayoutRequest ** Default Interface **
 *    Windows.UI.Text.Core.ICoreTextLayoutRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextLayoutRequest_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextLayoutRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextLayoutRequest[] = L"Windows.UI.Text.Core.CoreTextLayoutRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextLayoutRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextLayoutRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextLayoutRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextLayoutRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextLayoutRequestedEventArgs[] = L"Windows.UI.Text.Core.CoreTextLayoutRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextSelectionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextSelectionRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextSelectionRequest_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextSelectionRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextSelectionRequest[] = L"Windows.UI.Text.Core.CoreTextSelectionRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextSelectionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextSelectionRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextSelectionRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextSelectionRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextSelectionRequestedEventArgs[] = L"Windows.UI.Text.Core.CoreTextSelectionRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextSelectionUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextSelectionUpdatingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextSelectionUpdatingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextSelectionUpdatingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextSelectionUpdatingEventArgs[] = L"Windows.UI.Text.Core.CoreTextSelectionUpdatingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextServicesConstants
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Text.Core.ICoreTextServicesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextServicesConstants_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextServicesConstants_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextServicesConstants[] = L"Windows.UI.Text.Core.CoreTextServicesConstants";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextServicesManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Text.Core.ICoreTextServicesManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextServicesManager ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextServicesManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextServicesManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextServicesManager[] = L"Windows.UI.Text.Core.CoreTextServicesManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextTextRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextTextRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextTextRequest_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextTextRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextTextRequest[] = L"Windows.UI.Text.Core.CoreTextTextRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextTextRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextTextRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextTextRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextTextRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextTextRequestedEventArgs[] = L"Windows.UI.Text.Core.CoreTextTextRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextTextUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextTextUpdatingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextTextUpdatingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextTextUpdatingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextTextUpdatingEventArgs[] = L"Windows.UI.Text.Core.CoreTextTextUpdatingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2 __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2 __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs;

#endif // ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment;

typedef struct __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegmentVtbl;

interface __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment __FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment;

typedef struct __FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        __FIIterator_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegmentVtbl;

interface __FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment;

typedef struct __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegmentVtbl;

interface __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CUI_CText_CUnderlineType __x_ABI_CWindows_CUI_CText_CUnderlineType;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CUI__CText__CUnderlineType_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CUI__CText__CUnderlineType_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CUI__CText__CUnderlineType __FIReference_1_Windows__CUI__CText__CUnderlineType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CUI__CText__CUnderlineType;

typedef struct __FIReference_1_Windows__CUI__CText__CUnderlineTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CUI__CText__CUnderlineType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CUI__CText__CUnderlineType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CUI__CText__CUnderlineType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CUI__CText__CUnderlineType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CUI__CText__CUnderlineType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CUI__CText__CUnderlineType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CUI__CText__CUnderlineType* This,
        enum __x_ABI_CWindows_CUI_CText_CUnderlineType* result);

    END_INTERFACE
} __FIReference_1_Windows__CUI__CText__CUnderlineTypeVtbl;

interface __FIReference_1_Windows__CUI__CText__CUnderlineType
{
    CONST_VTBL struct __FIReference_1_Windows__CUI__CText__CUnderlineTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CUI__CText__CUnderlineType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CUI__CText__CUnderlineType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CUI__CText__CUnderlineType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CUI__CText__CUnderlineType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CUI__CText__CUnderlineType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CUI__CText__CUnderlineType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CUI__CText__CUnderlineType_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CUI__CText__CUnderlineType_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CUI_CViewManagement_CUIElementType __x_ABI_CWindows_CUI_CViewManagement_CUIElementType;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CUI__CViewManagement__CUIElementType_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CUI__CViewManagement__CUIElementType_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CUI__CViewManagement__CUIElementType __FIReference_1_Windows__CUI__CViewManagement__CUIElementType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CUI__CViewManagement__CUIElementType;

typedef struct __FIReference_1_Windows__CUI__CViewManagement__CUIElementTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CUI__CViewManagement__CUIElementType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CUI__CViewManagement__CUIElementType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CUI__CViewManagement__CUIElementType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CUI__CViewManagement__CUIElementType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CUI__CViewManagement__CUIElementType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CUI__CViewManagement__CUIElementType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CUI__CViewManagement__CUIElementType* This,
        enum __x_ABI_CWindows_CUI_CViewManagement_CUIElementType* result);

    END_INTERFACE
} __FIReference_1_Windows__CUI__CViewManagement__CUIElementTypeVtbl;

interface __FIReference_1_Windows__CUI__CViewManagement__CUIElementType
{
    CONST_VTBL struct __FIReference_1_Windows__CUI__CViewManagement__CUIElementTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CUI__CViewManagement__CUIElementType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CUI__CViewManagement__CUIElementType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CUI__CViewManagement__CUIElementType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CUI__CViewManagement__CUIElementType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CUI__CViewManagement__CUIElementType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CUI__CViewManagement__CUIElementType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CUI__CViewManagement__CUIElementType_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CUI__CViewManagement__CUIElementType_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* sender,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* sender,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* sender,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* sender,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* sender,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* sender,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* sender,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* sender,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

#ifndef ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__
#define ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGlobalization_CILanguage __x_ABI_CWindows_CGlobalization_CILanguage;

#endif // ____x_ABI_CWindows_CGlobalization_CILanguage_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextFormatUpdatingReason __x_ABI_CWindows_CUI_CText_CCore_CCoreTextFormatUpdatingReason;

typedef enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextFormatUpdatingResult __x_ABI_CWindows_CUI_CText_CCore_CCoreTextFormatUpdatingResult;

typedef enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputPaneDisplayPolicy __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputPaneDisplayPolicy;

typedef enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputScope __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputScope;

typedef enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextSelectionUpdatingResult __x_ABI_CWindows_CUI_CText_CCore_CCoreTextSelectionUpdatingResult;

typedef enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextTextUpdatingResult __x_ABI_CWindows_CUI_CText_CCore_CCoreTextTextUpdatingResult;

typedef struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange;

/*
 *
 * Struct Windows.UI.Text.Core.CoreTextFormatUpdatingReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextFormatUpdatingReason
{
    CoreTextFormatUpdatingReason_None = 0,
    CoreTextFormatUpdatingReason_CompositionUnconverted = 1,
    CoreTextFormatUpdatingReason_CompositionConverted = 2,
    CoreTextFormatUpdatingReason_CompositionTargetUnconverted = 3,
    CoreTextFormatUpdatingReason_CompositionTargetConverted = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Text.Core.CoreTextFormatUpdatingResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextFormatUpdatingResult
{
    CoreTextFormatUpdatingResult_Succeeded = 0,
    CoreTextFormatUpdatingResult_Failed = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Text.Core.CoreTextInputPaneDisplayPolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputPaneDisplayPolicy
{
    CoreTextInputPaneDisplayPolicy_Automatic = 0,
    CoreTextInputPaneDisplayPolicy_Manual = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Text.Core.CoreTextInputScope
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputScope
{
    CoreTextInputScope_Default = 0,
    CoreTextInputScope_Url = 1,
    CoreTextInputScope_FilePath = 2,
    CoreTextInputScope_FileName = 3,
    CoreTextInputScope_EmailUserName = 4,
    CoreTextInputScope_EmailAddress = 5,
    CoreTextInputScope_UserName = 6,
    CoreTextInputScope_PersonalFullName = 7,
    CoreTextInputScope_PersonalNamePrefix = 8,
    CoreTextInputScope_PersonalGivenName = 9,
    CoreTextInputScope_PersonalMiddleName = 10,
    CoreTextInputScope_PersonalSurname = 11,
    CoreTextInputScope_PersonalNameSuffix = 12,
    CoreTextInputScope_Address = 13,
    CoreTextInputScope_AddressPostalCode = 14,
    CoreTextInputScope_AddressStreet = 15,
    CoreTextInputScope_AddressStateOrProvince = 16,
    CoreTextInputScope_AddressCity = 17,
    CoreTextInputScope_AddressCountryName = 18,
    CoreTextInputScope_AddressCountryShortName = 19,
    CoreTextInputScope_CurrencyAmountAndSymbol = 20,
    CoreTextInputScope_CurrencyAmount = 21,
    CoreTextInputScope_Date = 22,
    CoreTextInputScope_DateMonth = 23,
    CoreTextInputScope_DateDay = 24,
    CoreTextInputScope_DateYear = 25,
    CoreTextInputScope_DateMonthName = 26,
    CoreTextInputScope_DateDayName = 27,
    CoreTextInputScope_Number = 29,
    CoreTextInputScope_SingleCharacter = 30,
    CoreTextInputScope_Password = 31,
    CoreTextInputScope_TelephoneNumber = 32,
    CoreTextInputScope_TelephoneCountryCode = 33,
    CoreTextInputScope_TelephoneAreaCode = 34,
    CoreTextInputScope_TelephoneLocalNumber = 35,
    CoreTextInputScope_Time = 36,
    CoreTextInputScope_TimeHour = 37,
    CoreTextInputScope_TimeMinuteOrSecond = 38,
    CoreTextInputScope_NumberFullWidth = 39,
    CoreTextInputScope_AlphanumericHalfWidth = 40,
    CoreTextInputScope_AlphanumericFullWidth = 41,
    CoreTextInputScope_CurrencyChinese = 42,
    CoreTextInputScope_Bopomofo = 43,
    CoreTextInputScope_Hiragana = 44,
    CoreTextInputScope_KatakanaHalfWidth = 45,
    CoreTextInputScope_KatakanaFullWidth = 46,
    CoreTextInputScope_Hanja = 47,
    CoreTextInputScope_HangulHalfWidth = 48,
    CoreTextInputScope_HangulFullWidth = 49,
    CoreTextInputScope_Search = 50,
    CoreTextInputScope_Formula = 51,
    CoreTextInputScope_SearchIncremental = 52,
    CoreTextInputScope_ChineseHalfWidth = 53,
    CoreTextInputScope_ChineseFullWidth = 54,
    CoreTextInputScope_NativeScript = 55,
    CoreTextInputScope_Text = 57,
    CoreTextInputScope_Chat = 58,
    CoreTextInputScope_NameOrPhoneNumber = 59,
    CoreTextInputScope_EmailUserNameOrAddress = 60,
    CoreTextInputScope_Private = 61,
    CoreTextInputScope_Maps = 62,
    CoreTextInputScope_PasswordNumeric = 63,
    CoreTextInputScope_FormulaNumber = 67,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    CoreTextInputScope_ChatWithoutEmoji = 68,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    CoreTextInputScope_Digits = 28,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    CoreTextInputScope_PinNumeric = 64,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    CoreTextInputScope_PinAlphanumeric = 65,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Text.Core.CoreTextSelectionUpdatingResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextSelectionUpdatingResult
{
    CoreTextSelectionUpdatingResult_Succeeded = 0,
    CoreTextSelectionUpdatingResult_Failed = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Text.Core.CoreTextTextUpdatingResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextTextUpdatingResult
{
    CoreTextTextUpdatingResult_Succeeded = 0,
    CoreTextTextUpdatingResult_Failed = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Text.Core.CoreTextRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange
{
    INT32 StartCaretPosition;
    INT32 EndCaretPosition;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextCompositionCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextCompositionCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextCompositionCompletedEventArgs[] = L"Windows.UI.Text.Core.ICoreTextCompositionCompletedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsCanceled)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_CompositionSegments)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs* This,
        __FIVectorView_1_Windows__CUI__CText__CCore__CCoreTextCompositionSegment** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_get_IsCanceled(This, value) \
    ((This)->lpVtbl->get_IsCanceled(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_get_CompositionSegments(This, value) \
    ((This)->lpVtbl->get_CompositionSegments(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextCompositionSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextCompositionSegment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextCompositionSegment[] = L"Windows.UI.Text.Core.ICoreTextCompositionSegment";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PreconversionString)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Range)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegmentVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_get_PreconversionString(This, value) \
    ((This)->lpVtbl->get_PreconversionString(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_get_Range(This, value) \
    ((This)->lpVtbl->get_Range(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionSegment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextCompositionStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextCompositionStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextCompositionStartedEventArgs[] = L"Windows.UI.Text.Core.ICoreTextCompositionStartedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsCanceled)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_get_IsCanceled(This, value) \
    ((This)->lpVtbl->get_IsCanceled(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextCompositionStartedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextEditContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextEditContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextEditContext[] = L"Windows.UI.Text.Core.ICoreTextEditContext";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Name)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_InputScope)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputScope* value);
    HRESULT (STDMETHODCALLTYPE* put_InputScope)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputScope value);
    HRESULT (STDMETHODCALLTYPE* get_IsReadOnly)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsReadOnly)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_InputPaneDisplayPolicy)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputPaneDisplayPolicy* value);
    HRESULT (STDMETHODCALLTYPE* put_InputPaneDisplayPolicy)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextInputPaneDisplayPolicy value);
    HRESULT (STDMETHODCALLTYPE* add_TextRequested)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextRequestedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_TextRequested)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_SelectionRequested)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionRequestedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_SelectionRequested)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_LayoutRequested)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextLayoutRequestedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_LayoutRequested)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_TextUpdating)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextTextUpdatingEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_TextUpdating)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_SelectionUpdating)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextSelectionUpdatingEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_SelectionUpdating)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_FormatUpdating)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextFormatUpdatingEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_FormatUpdating)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_CompositionStarted)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionStartedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_CompositionStarted)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_CompositionCompleted)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_Windows__CUI__CText__CCore__CCoreTextCompositionCompletedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_CompositionCompleted)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_FocusRemoved)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_FocusRemoved)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* NotifyFocusEnter)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This);
    HRESULT (STDMETHODCALLTYPE* NotifyFocusLeave)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This);
    HRESULT (STDMETHODCALLTYPE* NotifyTextChanged)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange modifiedRange,
        INT32 newLength,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange newSelection);
    HRESULT (STDMETHODCALLTYPE* NotifySelectionChanged)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange selection);
    HRESULT (STDMETHODCALLTYPE* NotifyLayoutChanged)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContextVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_put_Name(This, value) \
    ((This)->lpVtbl->put_Name(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_get_InputScope(This, value) \
    ((This)->lpVtbl->get_InputScope(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_put_InputScope(This, value) \
    ((This)->lpVtbl->put_InputScope(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_get_IsReadOnly(This, value) \
    ((This)->lpVtbl->get_IsReadOnly(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_put_IsReadOnly(This, value) \
    ((This)->lpVtbl->put_IsReadOnly(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_get_InputPaneDisplayPolicy(This, value) \
    ((This)->lpVtbl->get_InputPaneDisplayPolicy(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_put_InputPaneDisplayPolicy(This, value) \
    ((This)->lpVtbl->put_InputPaneDisplayPolicy(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_add_TextRequested(This, handler, cookie) \
    ((This)->lpVtbl->add_TextRequested(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_remove_TextRequested(This, cookie) \
    ((This)->lpVtbl->remove_TextRequested(This, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_add_SelectionRequested(This, handler, cookie) \
    ((This)->lpVtbl->add_SelectionRequested(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_remove_SelectionRequested(This, cookie) \
    ((This)->lpVtbl->remove_SelectionRequested(This, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_add_LayoutRequested(This, handler, cookie) \
    ((This)->lpVtbl->add_LayoutRequested(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_remove_LayoutRequested(This, cookie) \
    ((This)->lpVtbl->remove_LayoutRequested(This, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_add_TextUpdating(This, handler, cookie) \
    ((This)->lpVtbl->add_TextUpdating(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_remove_TextUpdating(This, cookie) \
    ((This)->lpVtbl->remove_TextUpdating(This, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_add_SelectionUpdating(This, handler, cookie) \
    ((This)->lpVtbl->add_SelectionUpdating(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_remove_SelectionUpdating(This, cookie) \
    ((This)->lpVtbl->remove_SelectionUpdating(This, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_add_FormatUpdating(This, handler, cookie) \
    ((This)->lpVtbl->add_FormatUpdating(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_remove_FormatUpdating(This, cookie) \
    ((This)->lpVtbl->remove_FormatUpdating(This, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_add_CompositionStarted(This, handler, cookie) \
    ((This)->lpVtbl->add_CompositionStarted(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_remove_CompositionStarted(This, cookie) \
    ((This)->lpVtbl->remove_CompositionStarted(This, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_add_CompositionCompleted(This, handler, cookie) \
    ((This)->lpVtbl->add_CompositionCompleted(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_remove_CompositionCompleted(This, cookie) \
    ((This)->lpVtbl->remove_CompositionCompleted(This, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_add_FocusRemoved(This, handler, cookie) \
    ((This)->lpVtbl->add_FocusRemoved(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_remove_FocusRemoved(This, cookie) \
    ((This)->lpVtbl->remove_FocusRemoved(This, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_NotifyFocusEnter(This) \
    ((This)->lpVtbl->NotifyFocusEnter(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_NotifyFocusLeave(This) \
    ((This)->lpVtbl->NotifyFocusLeave(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_NotifyTextChanged(This, modifiedRange, newLength, newSelection) \
    ((This)->lpVtbl->NotifyTextChanged(This, modifiedRange, newLength, newSelection))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_NotifySelectionChanged(This, selection) \
    ((This)->lpVtbl->NotifySelectionChanged(This, selection))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_NotifyLayoutChanged(This) \
    ((This)->lpVtbl->NotifyLayoutChanged(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextEditContext2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextEditContext
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextEditContext2[] = L"Windows.UI.Text.Core.ICoreTextEditContext2";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_NotifyFocusLeaveCompleted)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2* This,
        __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextEditContext_IInspectable* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_NotifyFocusLeaveCompleted)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2Vtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_add_NotifyFocusLeaveCompleted(This, handler, cookie) \
    ((This)->lpVtbl->add_NotifyFocusLeaveCompleted(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_remove_NotifyFocusLeaveCompleted(This, cookie) \
    ((This)->lpVtbl->remove_NotifyFocusLeaveCompleted(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextFormatUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextFormatUpdatingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextFormatUpdatingEventArgs[] = L"Windows.UI.Text.Core.ICoreTextFormatUpdatingEventArgs";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Range)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange* value);
    HRESULT (STDMETHODCALLTYPE* get_TextColor)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* This,
        __FIReference_1_Windows__CUI__CViewManagement__CUIElementType** value);
    HRESULT (STDMETHODCALLTYPE* get_BackgroundColor)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* This,
        __FIReference_1_Windows__CUI__CViewManagement__CUIElementType** value);
    HRESULT (STDMETHODCALLTYPE* get_UnderlineColor)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* This,
        __FIReference_1_Windows__CUI__CViewManagement__CUIElementType** value);
    HRESULT (STDMETHODCALLTYPE* get_UnderlineType)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* This,
        __FIReference_1_Windows__CUI__CText__CUnderlineType** value);
    HRESULT (STDMETHODCALLTYPE* get_Reason)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* This,
        enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextFormatUpdatingReason* value);
    HRESULT (STDMETHODCALLTYPE* get_Result)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* This,
        enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextFormatUpdatingResult* value);
    HRESULT (STDMETHODCALLTYPE* put_Result)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* This,
        enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextFormatUpdatingResult value);
    HRESULT (STDMETHODCALLTYPE* get_IsCanceled)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_get_Range(This, value) \
    ((This)->lpVtbl->get_Range(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_get_TextColor(This, value) \
    ((This)->lpVtbl->get_TextColor(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_get_BackgroundColor(This, value) \
    ((This)->lpVtbl->get_BackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_get_UnderlineColor(This, value) \
    ((This)->lpVtbl->get_UnderlineColor(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_get_UnderlineType(This, value) \
    ((This)->lpVtbl->get_UnderlineType(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_get_Reason(This, value) \
    ((This)->lpVtbl->get_Reason(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_get_Result(This, value) \
    ((This)->lpVtbl->get_Result(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_put_Result(This, value) \
    ((This)->lpVtbl->put_Result(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_get_IsCanceled(This, value) \
    ((This)->lpVtbl->get_IsCanceled(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextFormatUpdatingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextLayoutBounds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextLayoutBounds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextLayoutBounds[] = L"Windows.UI.Text.Core.ICoreTextLayoutBounds";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBoundsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TextBounds)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* put_TextBounds)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds* This,
        struct __x_ABI_CWindows_CFoundation_CRect value);
    HRESULT (STDMETHODCALLTYPE* get_ControlBounds)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* put_ControlBounds)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds* This,
        struct __x_ABI_CWindows_CFoundation_CRect value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBoundsVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBoundsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_get_TextBounds(This, value) \
    ((This)->lpVtbl->get_TextBounds(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_put_TextBounds(This, value) \
    ((This)->lpVtbl->put_TextBounds(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_get_ControlBounds(This, value) \
    ((This)->lpVtbl->get_ControlBounds(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_put_ControlBounds(This, value) \
    ((This)->lpVtbl->put_ControlBounds(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextLayoutRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextLayoutRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextLayoutRequest[] = L"Windows.UI.Text.Core.ICoreTextLayoutRequest";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Range)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange* value);
    HRESULT (STDMETHODCALLTYPE* get_LayoutBounds)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds** value);
    HRESULT (STDMETHODCALLTYPE* get_IsCanceled)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_get_Range(This, value) \
    ((This)->lpVtbl->get_Range(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_get_LayoutBounds(This, value) \
    ((This)->lpVtbl->get_LayoutBounds(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_get_IsCanceled(This, value) \
    ((This)->lpVtbl->get_IsCanceled(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextLayoutRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextLayoutRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextLayoutRequest2[] = L"Windows.UI.Text.Core.ICoreTextLayoutRequest2";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LayoutBoundsVisualPixels)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutBounds** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2Vtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_get_LayoutBoundsVisualPixels(This, value) \
    ((This)->lpVtbl->get_LayoutBoundsVisualPixels(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextLayoutRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextLayoutRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextLayoutRequestedEventArgs[] = L"Windows.UI.Text.Core.ICoreTextLayoutRequestedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextLayoutRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextSelectionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextSelectionRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextSelectionRequest[] = L"Windows.UI.Text.Core.ICoreTextSelectionRequest";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Selection)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange* value);
    HRESULT (STDMETHODCALLTYPE* put_Selection)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange value);
    HRESULT (STDMETHODCALLTYPE* get_IsCanceled)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_get_Selection(This, value) \
    ((This)->lpVtbl->get_Selection(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_put_Selection(This, value) \
    ((This)->lpVtbl->put_Selection(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_get_IsCanceled(This, value) \
    ((This)->lpVtbl->get_IsCanceled(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextSelectionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextSelectionRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextSelectionRequestedEventArgs[] = L"Windows.UI.Text.Core.ICoreTextSelectionRequestedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextSelectionUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextSelectionUpdatingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextSelectionUpdatingEventArgs[] = L"Windows.UI.Text.Core.ICoreTextSelectionUpdatingEventArgs";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Selection)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange* value);
    HRESULT (STDMETHODCALLTYPE* get_Result)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs* This,
        enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextSelectionUpdatingResult* value);
    HRESULT (STDMETHODCALLTYPE* put_Result)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs* This,
        enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextSelectionUpdatingResult value);
    HRESULT (STDMETHODCALLTYPE* get_IsCanceled)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_get_Selection(This, value) \
    ((This)->lpVtbl->get_Selection(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_get_Result(This, value) \
    ((This)->lpVtbl->get_Result(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_put_Result(This, value) \
    ((This)->lpVtbl->put_Result(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_get_IsCanceled(This, value) \
    ((This)->lpVtbl->get_IsCanceled(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextSelectionUpdatingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextServicesManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextServicesManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextServicesManager[] = L"Windows.UI.Text.Core.ICoreTextServicesManager";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InputLanguage)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager* This,
        __x_ABI_CWindows_CGlobalization_CILanguage** value);
    HRESULT (STDMETHODCALLTYPE* add_InputLanguageChanged)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager* This,
        __FITypedEventHandler_2_Windows__CUI__CText__CCore__CCoreTextServicesManager_IInspectable* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_InputLanguageChanged)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* CreateEditContext)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextEditContext** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_get_InputLanguage(This, value) \
    ((This)->lpVtbl->get_InputLanguage(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_add_InputLanguageChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_InputLanguageChanged(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_remove_InputLanguageChanged(This, cookie) \
    ((This)->lpVtbl->remove_InputLanguageChanged(This, cookie))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_CreateEditContext(This, value) \
    ((This)->lpVtbl->CreateEditContext(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextServicesManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextServicesManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextServicesManagerStatics[] = L"Windows.UI.Text.Core.ICoreTextServicesManagerStatics";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManager** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_GetForCurrentView(This, value) \
    ((This)->lpVtbl->GetForCurrentView(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextServicesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextServicesConstants
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextServicesStatics[] = L"Windows.UI.Text.Core.ICoreTextServicesStatics";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HiddenCharacter)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics* This,
        WCHAR* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStaticsVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_get_HiddenCharacter(This, value) \
    ((This)->lpVtbl->get_HiddenCharacter(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextServicesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextTextRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextTextRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextTextRequest[] = L"Windows.UI.Text.Core.ICoreTextTextRequest";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Range)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange* value);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Text)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IsCanceled)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_get_Range(This, value) \
    ((This)->lpVtbl->get_Range(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_put_Text(This, value) \
    ((This)->lpVtbl->put_Text(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_get_IsCanceled(This, value) \
    ((This)->lpVtbl->get_IsCanceled(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextTextRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextTextRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextTextRequestedEventArgs[] = L"Windows.UI.Text.Core.ICoreTextTextRequestedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs* This,
        __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequest** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Text.Core.ICoreTextTextUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Text.Core.CoreTextTextUpdatingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Text_Core_ICoreTextTextUpdatingEventArgs[] = L"Windows.UI.Text.Core.ICoreTextTextUpdatingEventArgs";
typedef struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Range)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange* value);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NewSelection)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs* This,
        struct __x_ABI_CWindows_CUI_CText_CCore_CCoreTextRange* value);
    HRESULT (STDMETHODCALLTYPE* get_InputLanguage)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs* This,
        __x_ABI_CWindows_CGlobalization_CILanguage** value);
    HRESULT (STDMETHODCALLTYPE* get_Result)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs* This,
        enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextTextUpdatingResult* value);
    HRESULT (STDMETHODCALLTYPE* put_Result)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs* This,
        enum __x_ABI_CWindows_CUI_CText_CCore_CCoreTextTextUpdatingResult value);
    HRESULT (STDMETHODCALLTYPE* get_IsCanceled)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_get_Range(This, value) \
    ((This)->lpVtbl->get_Range(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_get_NewSelection(This, value) \
    ((This)->lpVtbl->get_NewSelection(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_get_InputLanguage(This, value) \
    ((This)->lpVtbl->get_InputLanguage(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_get_Result(This, value) \
    ((This)->lpVtbl->get_Result(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_put_Result(This, value) \
    ((This)->lpVtbl->put_Result(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_get_IsCanceled(This, value) \
    ((This)->lpVtbl->get_IsCanceled(This, value))

#define __x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CText_CCore_CICoreTextTextUpdatingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextCompositionCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextCompositionCompletedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextCompositionCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextCompositionCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextCompositionCompletedEventArgs[] = L"Windows.UI.Text.Core.CoreTextCompositionCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextCompositionSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextCompositionSegment ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextCompositionSegment_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextCompositionSegment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextCompositionSegment[] = L"Windows.UI.Text.Core.CoreTextCompositionSegment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextCompositionStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextCompositionStartedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextCompositionStartedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextCompositionStartedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextCompositionStartedEventArgs[] = L"Windows.UI.Text.Core.CoreTextCompositionStartedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextEditContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextEditContext ** Default Interface **
 *    Windows.UI.Text.Core.ICoreTextEditContext2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextEditContext_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextEditContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextEditContext[] = L"Windows.UI.Text.Core.CoreTextEditContext";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextFormatUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextFormatUpdatingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextFormatUpdatingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextFormatUpdatingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextFormatUpdatingEventArgs[] = L"Windows.UI.Text.Core.CoreTextFormatUpdatingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextLayoutBounds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextLayoutBounds ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextLayoutBounds_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextLayoutBounds_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextLayoutBounds[] = L"Windows.UI.Text.Core.CoreTextLayoutBounds";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextLayoutRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextLayoutRequest ** Default Interface **
 *    Windows.UI.Text.Core.ICoreTextLayoutRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextLayoutRequest_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextLayoutRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextLayoutRequest[] = L"Windows.UI.Text.Core.CoreTextLayoutRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextLayoutRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextLayoutRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextLayoutRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextLayoutRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextLayoutRequestedEventArgs[] = L"Windows.UI.Text.Core.CoreTextLayoutRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextSelectionRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextSelectionRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextSelectionRequest_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextSelectionRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextSelectionRequest[] = L"Windows.UI.Text.Core.CoreTextSelectionRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextSelectionRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextSelectionRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextSelectionRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextSelectionRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextSelectionRequestedEventArgs[] = L"Windows.UI.Text.Core.CoreTextSelectionRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextSelectionUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextSelectionUpdatingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextSelectionUpdatingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextSelectionUpdatingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextSelectionUpdatingEventArgs[] = L"Windows.UI.Text.Core.CoreTextSelectionUpdatingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextServicesConstants
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Text.Core.ICoreTextServicesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextServicesConstants_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextServicesConstants_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextServicesConstants[] = L"Windows.UI.Text.Core.CoreTextServicesConstants";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextServicesManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Text.Core.ICoreTextServicesManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextServicesManager ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextServicesManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextServicesManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextServicesManager[] = L"Windows.UI.Text.Core.CoreTextServicesManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextTextRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextTextRequest ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextTextRequest_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextTextRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextTextRequest[] = L"Windows.UI.Text.Core.CoreTextTextRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextTextRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextTextRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextTextRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextTextRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextTextRequestedEventArgs[] = L"Windows.UI.Text.Core.CoreTextTextRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Text.Core.CoreTextTextUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Text.Core.ICoreTextTextUpdatingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Text_Core_CoreTextTextUpdatingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Text_Core_CoreTextTextUpdatingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Text_Core_CoreTextTextUpdatingEventArgs[] = L"Windows.UI.Text.Core.CoreTextTextUpdatingEventArgs";
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
#endif // __windows2Eui2Etext2Ecore_p_h__

#endif // __windows2Eui2Etext2Ecore_h__
