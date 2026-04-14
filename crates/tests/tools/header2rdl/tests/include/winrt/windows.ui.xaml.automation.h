
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
#ifndef __windows2Eui2Examl2Eautomation_h__
#define __windows2Eui2Examl2Eautomation_h__
#ifndef __windows2Eui2Examl2Eautomation_p_h__
#define __windows2Eui2Examl2Eautomation_p_h__


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
#include "Windows.UI.Xaml.h"
#include "Windows.UI.Xaml.Automation.Peers.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAnnotationPatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers ABI::Windows::UI::Xaml::Automation::IAnnotationPatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAnnotationPatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::IAnnotationPatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationAnnotation;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation ABI::Windows::UI::Xaml::Automation::IAutomationAnnotation

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationAnnotationFactory;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory ABI::Windows::UI::Xaml::Automation::IAutomationAnnotationFactory

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationAnnotationStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics ABI::Windows::UI::Xaml::Automation::IAutomationAnnotationStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationElementIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers ABI::Windows::UI::Xaml::Automation::IAutomationElementIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationElementIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics ABI::Windows::UI::Xaml::Automation::IAutomationElementIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationElementIdentifiersStatics2;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2 ABI::Windows::UI::Xaml::Automation::IAutomationElementIdentifiersStatics2

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationElementIdentifiersStatics3;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3 ABI::Windows::UI::Xaml::Automation::IAutomationElementIdentifiersStatics3

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationElementIdentifiersStatics4;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4 ABI::Windows::UI::Xaml::Automation::IAutomationElementIdentifiersStatics4

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationElementIdentifiersStatics5;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5 ABI::Windows::UI::Xaml::Automation::IAutomationElementIdentifiersStatics5

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationElementIdentifiersStatics6;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6 ABI::Windows::UI::Xaml::Automation::IAutomationElementIdentifiersStatics6

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationElementIdentifiersStatics7;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7 ABI::Windows::UI::Xaml::Automation::IAutomationElementIdentifiersStatics7

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationElementIdentifiersStatics8;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8 ABI::Windows::UI::Xaml::Automation::IAutomationElementIdentifiersStatics8

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationProperties;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties ABI::Windows::UI::Xaml::Automation::IAutomationProperties

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationPropertiesStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics ABI::Windows::UI::Xaml::Automation::IAutomationPropertiesStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationPropertiesStatics2;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2 ABI::Windows::UI::Xaml::Automation::IAutomationPropertiesStatics2

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationPropertiesStatics3;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3 ABI::Windows::UI::Xaml::Automation::IAutomationPropertiesStatics3

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationPropertiesStatics4;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4 ABI::Windows::UI::Xaml::Automation::IAutomationPropertiesStatics4

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationPropertiesStatics5;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5 ABI::Windows::UI::Xaml::Automation::IAutomationPropertiesStatics5

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationPropertiesStatics6;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6 ABI::Windows::UI::Xaml::Automation::IAutomationPropertiesStatics6

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationPropertiesStatics7;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7 ABI::Windows::UI::Xaml::Automation::IAutomationPropertiesStatics7

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationPropertiesStatics8;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8 ABI::Windows::UI::Xaml::Automation::IAutomationPropertiesStatics8

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationPropertiesStatics9;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9 ABI::Windows::UI::Xaml::Automation::IAutomationPropertiesStatics9

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IAutomationProperty;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty ABI::Windows::UI::Xaml::Automation::IAutomationProperty

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IDockPatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers ABI::Windows::UI::Xaml::Automation::IDockPatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IDockPatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::IDockPatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IDragPatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers ABI::Windows::UI::Xaml::Automation::IDragPatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IDragPatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::IDragPatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IDropTargetPatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers ABI::Windows::UI::Xaml::Automation::IDropTargetPatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IDropTargetPatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::IDropTargetPatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IExpandCollapsePatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers ABI::Windows::UI::Xaml::Automation::IExpandCollapsePatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IExpandCollapsePatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::IExpandCollapsePatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IGridItemPatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers ABI::Windows::UI::Xaml::Automation::IGridItemPatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IGridItemPatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::IGridItemPatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IGridPatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers ABI::Windows::UI::Xaml::Automation::IGridPatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IGridPatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::IGridPatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IMultipleViewPatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers ABI::Windows::UI::Xaml::Automation::IMultipleViewPatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IMultipleViewPatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::IMultipleViewPatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IRangeValuePatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers ABI::Windows::UI::Xaml::Automation::IRangeValuePatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IRangeValuePatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::IRangeValuePatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IScrollPatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers ABI::Windows::UI::Xaml::Automation::IScrollPatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IScrollPatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::IScrollPatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface ISelectionItemPatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers ABI::Windows::UI::Xaml::Automation::ISelectionItemPatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface ISelectionItemPatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::ISelectionItemPatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface ISelectionPatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers ABI::Windows::UI::Xaml::Automation::ISelectionPatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface ISelectionPatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::ISelectionPatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface ISpreadsheetItemPatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers ABI::Windows::UI::Xaml::Automation::ISpreadsheetItemPatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface ISpreadsheetItemPatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::ISpreadsheetItemPatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IStylesPatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers ABI::Windows::UI::Xaml::Automation::IStylesPatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IStylesPatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::IStylesPatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface ITableItemPatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers ABI::Windows::UI::Xaml::Automation::ITableItemPatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface ITableItemPatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::ITableItemPatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface ITablePatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers ABI::Windows::UI::Xaml::Automation::ITablePatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface ITablePatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::ITablePatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface ITogglePatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers ABI::Windows::UI::Xaml::Automation::ITogglePatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface ITogglePatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::ITogglePatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface ITransformPattern2Identifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers ABI::Windows::UI::Xaml::Automation::ITransformPattern2Identifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface ITransformPattern2IdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics ABI::Windows::UI::Xaml::Automation::ITransformPattern2IdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface ITransformPatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers ABI::Windows::UI::Xaml::Automation::ITransformPatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface ITransformPatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::ITransformPatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IValuePatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers ABI::Windows::UI::Xaml::Automation::IValuePatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IValuePatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::IValuePatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IWindowPatternIdentifiers;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers ABI::Windows::UI::Xaml::Automation::IWindowPatternIdentifiers

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    interface IWindowPatternIdentifiersStatics;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics ABI::Windows::UI::Xaml::Automation::IWindowPatternIdentifiersStatics

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    class AutomationAnnotation;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_USE
#define DEF___FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7ed32ba3-db0c-5a54-ab43-30628afbc2d8"))
IIterator<ABI::Windows::UI::Xaml::Automation::AutomationAnnotation*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Automation::AutomationAnnotation*, ABI::Windows::UI::Xaml::Automation::IAutomationAnnotation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Xaml.Automation.AutomationAnnotation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Xaml::Automation::AutomationAnnotation*> __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_t;
#define __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_USE
#define DEF___FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c6e210cb-1c7b-5e6a-936b-61d4e536a291"))
IIterable<ABI::Windows::UI::Xaml::Automation::AutomationAnnotation*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Automation::AutomationAnnotation*, ABI::Windows::UI::Xaml::Automation::IAutomationAnnotation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Xaml.Automation.AutomationAnnotation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Xaml::Automation::AutomationAnnotation*> __FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_t;
#define __FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CXaml__CDependencyObject_USE
#define DEF___FIIterator_1_Windows__CUI__CXaml__CDependencyObject_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("29f8d454-905d-587e-b9d8-bfd418805a65"))
IIterator<ABI::Windows::UI::Xaml::DependencyObject*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::DependencyObject*, ABI::Windows::UI::Xaml::IDependencyObject*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Xaml.DependencyObject>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Xaml::DependencyObject*> __FIIterator_1_Windows__CUI__CXaml__CDependencyObject_t;
#define __FIIterator_1_Windows__CUI__CXaml__CDependencyObject ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CXaml__CDependencyObject_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CXaml__CDependencyObject_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CXaml__CDependencyObject_USE
#define DEF___FIIterable_1_Windows__CUI__CXaml__CDependencyObject_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f66c6bd3-55b4-5bbb-b82a-6d9ce383091a"))
IIterable<ABI::Windows::UI::Xaml::DependencyObject*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::DependencyObject*, ABI::Windows::UI::Xaml::IDependencyObject*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Xaml.DependencyObject>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Xaml::DependencyObject*> __FIIterable_1_Windows__CUI__CXaml__CDependencyObject_t;
#define __FIIterable_1_Windows__CUI__CXaml__CDependencyObject ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CXaml__CDependencyObject_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CXaml__CDependencyObject_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CXaml__CUIElement_USE
#define DEF___FIIterator_1_Windows__CUI__CXaml__CUIElement_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1d1f9d60-d53b-57f7-b144-8f7c487846e8"))
IIterator<ABI::Windows::UI::Xaml::UIElement*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::UIElement*, ABI::Windows::UI::Xaml::IUIElement*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Xaml.UIElement>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Xaml::UIElement*> __FIIterator_1_Windows__CUI__CXaml__CUIElement_t;
#define __FIIterator_1_Windows__CUI__CXaml__CUIElement ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CXaml__CUIElement_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CXaml__CUIElement_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CXaml__CUIElement_USE
#define DEF___FIIterable_1_Windows__CUI__CXaml__CUIElement_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("42e26ae1-d357-57e8-bb48-f75c9ff69d91"))
IIterable<ABI::Windows::UI::Xaml::UIElement*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::UIElement*, ABI::Windows::UI::Xaml::IUIElement*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Xaml.UIElement>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Xaml::UIElement*> __FIIterable_1_Windows__CUI__CXaml__CUIElement_t;
#define __FIIterable_1_Windows__CUI__CXaml__CUIElement ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CXaml__CUIElement_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CXaml__CUIElement_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_USE
#define DEF___FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a4b8a26c-9009-5394-98c8-98107e80db61"))
IVectorView<ABI::Windows::UI::Xaml::Automation::AutomationAnnotation*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Automation::AutomationAnnotation*, ABI::Windows::UI::Xaml::Automation::IAutomationAnnotation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Xaml.Automation.AutomationAnnotation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Xaml::Automation::AutomationAnnotation*> __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_t;
#define __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_USE
#define DEF___FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fe750d77-1307-5df2-a021-1c7a8d6b80ad"))
IVectorView<ABI::Windows::UI::Xaml::DependencyObject*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::DependencyObject*, ABI::Windows::UI::Xaml::IDependencyObject*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Xaml.DependencyObject>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Xaml::DependencyObject*> __FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_t;
#define __FIVectorView_1_Windows__CUI__CXaml__CDependencyObject ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CXaml__CUIElement_USE
#define DEF___FIVectorView_1_Windows__CUI__CXaml__CUIElement_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f3864c10-14a4-5516-b1d9-63b6579429b1"))
IVectorView<ABI::Windows::UI::Xaml::UIElement*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::UIElement*, ABI::Windows::UI::Xaml::IUIElement*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Xaml.UIElement>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Xaml::UIElement*> __FIVectorView_1_Windows__CUI__CXaml__CUIElement_t;
#define __FIVectorView_1_Windows__CUI__CXaml__CUIElement ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CXaml__CUIElement_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CXaml__CUIElement_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_USE
#define DEF___FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("73596c82-f6e5-5b57-8dc5-556cb7a8fbe6"))
IVector<ABI::Windows::UI::Xaml::Automation::AutomationAnnotation*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::Automation::AutomationAnnotation*, ABI::Windows::UI::Xaml::Automation::IAutomationAnnotation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Xaml.Automation.AutomationAnnotation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::Xaml::Automation::AutomationAnnotation*> __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_t;
#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CUI__CXaml__CDependencyObject_USE
#define DEF___FIVector_1_Windows__CUI__CXaml__CDependencyObject_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("771b857e-ab5c-5db8-a021-397c92cdc44c"))
IVector<ABI::Windows::UI::Xaml::DependencyObject*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::DependencyObject*, ABI::Windows::UI::Xaml::IDependencyObject*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Xaml.DependencyObject>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::Xaml::DependencyObject*> __FIVector_1_Windows__CUI__CXaml__CDependencyObject_t;
#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CXaml__CDependencyObject_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CXaml__CDependencyObject_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CUI__CXaml__CUIElement_USE
#define DEF___FIVector_1_Windows__CUI__CXaml__CUIElement_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b4c1e3ac-8768-5b9d-a661-f63330b8507b"))
IVector<ABI::Windows::UI::Xaml::UIElement*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Xaml::UIElement*, ABI::Windows::UI::Xaml::IUIElement*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Xaml.UIElement>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::Xaml::UIElement*> __FIVector_1_Windows__CUI__CXaml__CUIElement_t;
#define __FIVector_1_Windows__CUI__CXaml__CUIElement ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CXaml__CUIElement_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CXaml__CUIElement_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    namespace Peers {
                        typedef enum AccessibilityView : int AccessibilityView;
                    } /* Peers */
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    namespace Peers {
                        typedef enum AutomationControlType : int AutomationControlType;
                    } /* Peers */
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    namespace Peers {
                        typedef enum AutomationHeadingLevel : int AutomationHeadingLevel;
                    } /* Peers */
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    namespace Peers {
                        typedef enum AutomationLandmarkType : int AutomationLandmarkType;
                    } /* Peers */
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    namespace Peers {
                        typedef enum AutomationLiveSetting : int AutomationLiveSetting;
                    } /* Peers */
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

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
                namespace Automation {
                    typedef enum AnnotationType : int AnnotationType;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    class AutomationProperty;
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Xaml.Automation.AnnotationType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum AnnotationType : int
                    {
                        AnnotationType_Unknown = 60000,
                        AnnotationType_SpellingError = 60001,
                        AnnotationType_GrammarError = 60002,
                        AnnotationType_Comment = 60003,
                        AnnotationType_FormulaError = 60004,
                        AnnotationType_TrackChanges = 60005,
                        AnnotationType_Header = 60006,
                        AnnotationType_Footer = 60007,
                        AnnotationType_Highlighted = 60008,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                        AnnotationType_Endnote = 60009,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                        AnnotationType_Footnote = 60010,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        AnnotationType_InsertionChange = 60011,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        AnnotationType_DeletionChange = 60012,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        AnnotationType_MoveChange = 60013,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        AnnotationType_FormatChange = 60014,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        AnnotationType_UnsyncedChange = 60015,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        AnnotationType_EditingLockedChange = 60016,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        AnnotationType_ExternalChange = 60017,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        AnnotationType_ConflictingChange = 60018,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        AnnotationType_Author = 60019,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        AnnotationType_AdvancedProofingIssue = 60020,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        AnnotationType_DataValidationError = 60021,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        AnnotationType_CircularReferenceError = 60022,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationActiveEnd
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum AutomationActiveEnd : int
                    {
                        AutomationActiveEnd_None = 0,
                        AutomationActiveEnd_Start = 1,
                        AutomationActiveEnd_End = 2,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationAnimationStyle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum AutomationAnimationStyle : int
                    {
                        AutomationAnimationStyle_None = 0,
                        AutomationAnimationStyle_LasVegasLights = 1,
                        AutomationAnimationStyle_BlinkingBackground = 2,
                        AutomationAnimationStyle_SparkleText = 3,
                        AutomationAnimationStyle_MarchingBlackAnts = 4,
                        AutomationAnimationStyle_MarchingRedAnts = 5,
                        AutomationAnimationStyle_Shimmer = 6,
                        AutomationAnimationStyle_Other = 7,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationBulletStyle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum AutomationBulletStyle : int
                    {
                        AutomationBulletStyle_None = 0,
                        AutomationBulletStyle_HollowRoundBullet = 1,
                        AutomationBulletStyle_FilledRoundBullet = 2,
                        AutomationBulletStyle_HollowSquareBullet = 3,
                        AutomationBulletStyle_FilledSquareBullet = 4,
                        AutomationBulletStyle_DashBullet = 5,
                        AutomationBulletStyle_Other = 6,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationCaretBidiMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum AutomationCaretBidiMode : int
                    {
                        AutomationCaretBidiMode_LTR = 0,
                        AutomationCaretBidiMode_RTL = 1,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationCaretPosition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum AutomationCaretPosition : int
                    {
                        AutomationCaretPosition_Unknown = 0,
                        AutomationCaretPosition_EndOfLine = 1,
                        AutomationCaretPosition_BeginningOfLine = 2,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationFlowDirections
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum AutomationFlowDirections : int
                    {
                        AutomationFlowDirections_Default = 0,
                        AutomationFlowDirections_RightToLeft = 1,
                        AutomationFlowDirections_BottomToTop = 2,
                        AutomationFlowDirections_Vertical = 3,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationOutlineStyles
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum AutomationOutlineStyles : int
                    {
                        AutomationOutlineStyles_None = 0,
                        AutomationOutlineStyles_Outline = 1,
                        AutomationOutlineStyles_Shadow = 2,
                        AutomationOutlineStyles_Engraved = 3,
                        AutomationOutlineStyles_Embossed = 4,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationStyleId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum AutomationStyleId : int
                    {
                        AutomationStyleId_Heading1 = 70001,
                        AutomationStyleId_Heading2 = 70002,
                        AutomationStyleId_Heading3 = 70003,
                        AutomationStyleId_Heading4 = 70004,
                        AutomationStyleId_Heading5 = 70005,
                        AutomationStyleId_Heading6 = 70006,
                        AutomationStyleId_Heading7 = 70007,
                        AutomationStyleId_Heading8 = 70008,
                        AutomationStyleId_Heading9 = 70009,
                        AutomationStyleId_Title = 70010,
                        AutomationStyleId_Subtitle = 70011,
                        AutomationStyleId_Normal = 70012,
                        AutomationStyleId_Emphasis = 70013,
                        AutomationStyleId_Quote = 70014,
                        AutomationStyleId_BulletedList = 70015,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationTextDecorationLineStyle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum AutomationTextDecorationLineStyle : int
                    {
                        AutomationTextDecorationLineStyle_None = 0,
                        AutomationTextDecorationLineStyle_Single = 1,
                        AutomationTextDecorationLineStyle_WordsOnly = 2,
                        AutomationTextDecorationLineStyle_Double = 3,
                        AutomationTextDecorationLineStyle_Dot = 4,
                        AutomationTextDecorationLineStyle_Dash = 5,
                        AutomationTextDecorationLineStyle_DashDot = 6,
                        AutomationTextDecorationLineStyle_DashDotDot = 7,
                        AutomationTextDecorationLineStyle_Wavy = 8,
                        AutomationTextDecorationLineStyle_ThickSingle = 9,
                        AutomationTextDecorationLineStyle_DoubleWavy = 10,
                        AutomationTextDecorationLineStyle_ThickWavy = 11,
                        AutomationTextDecorationLineStyle_LongDash = 12,
                        AutomationTextDecorationLineStyle_ThickDash = 13,
                        AutomationTextDecorationLineStyle_ThickDashDot = 14,
                        AutomationTextDecorationLineStyle_ThickDashDotDot = 15,
                        AutomationTextDecorationLineStyle_ThickDot = 16,
                        AutomationTextDecorationLineStyle_ThickLongDash = 17,
                        AutomationTextDecorationLineStyle_Other = 18,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationTextEditChangeType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum AutomationTextEditChangeType : int
                    {
                        AutomationTextEditChangeType_None = 0,
                        AutomationTextEditChangeType_AutoCorrect = 1,
                        AutomationTextEditChangeType_Composition = 2,
                        AutomationTextEditChangeType_CompositionFinalized = 3,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.DockPosition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum DockPosition : int
                    {
                        DockPosition_Top = 0,
                        DockPosition_Left = 1,
                        DockPosition_Bottom = 2,
                        DockPosition_Right = 3,
                        DockPosition_Fill = 4,
                        DockPosition_None = 5,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.ExpandCollapseState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum ExpandCollapseState : int
                    {
                        ExpandCollapseState_Collapsed = 0,
                        ExpandCollapseState_Expanded = 1,
                        ExpandCollapseState_PartiallyExpanded = 2,
                        ExpandCollapseState_LeafNode = 3,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.RowOrColumnMajor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum RowOrColumnMajor : int
                    {
                        RowOrColumnMajor_RowMajor = 0,
                        RowOrColumnMajor_ColumnMajor = 1,
                        RowOrColumnMajor_Indeterminate = 2,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.ScrollAmount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum ScrollAmount : int
                    {
                        ScrollAmount_LargeDecrement = 0,
                        ScrollAmount_SmallDecrement = 1,
                        ScrollAmount_NoAmount = 2,
                        ScrollAmount_LargeIncrement = 3,
                        ScrollAmount_SmallIncrement = 4,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.SupportedTextSelection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum SupportedTextSelection : int
                    {
                        SupportedTextSelection_None = 0,
                        SupportedTextSelection_Single = 1,
                        SupportedTextSelection_Multiple = 2,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.SynchronizedInputType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum SynchronizedInputType : int
                    {
                        SynchronizedInputType_KeyUp = 1,
                        SynchronizedInputType_KeyDown = 2,
                        SynchronizedInputType_LeftMouseUp = 4,
                        SynchronizedInputType_LeftMouseDown = 8,
                        SynchronizedInputType_RightMouseUp = 16,
                        SynchronizedInputType_RightMouseDown = 32,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.ToggleState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum ToggleState : int
                    {
                        ToggleState_Off = 0,
                        ToggleState_On = 1,
                        ToggleState_Indeterminate = 2,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.WindowInteractionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum WindowInteractionState : int
                    {
                        WindowInteractionState_Running = 0,
                        WindowInteractionState_Closing = 1,
                        WindowInteractionState_ReadyForUserInteraction = 2,
                        WindowInteractionState_BlockedByModalWindow = 3,
                        WindowInteractionState_NotResponding = 4,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.WindowVisualState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum WindowVisualState : int
                    {
                        WindowVisualState_Normal = 0,
                        WindowVisualState_Maximized = 1,
                        WindowVisualState_Minimized = 2,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.ZoomUnit
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    enum ZoomUnit : int
                    {
                        ZoomUnit_NoAmount = 0,
                        ZoomUnit_LargeDecrement = 1,
                        ZoomUnit_SmallDecrement = 2,
                        ZoomUnit_LargeIncrement = 3,
                        ZoomUnit_SmallIncrement = 4,
                    };
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAnnotationPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AnnotationPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAnnotationPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IAnnotationPatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("d475a0c1-48b2-4e40-a6cf-3dc4b638c0de")
                    IAnnotationPatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IAnnotationPatternIdentifiers = __uuidof(IAnnotationPatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAnnotationPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AnnotationPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAnnotationPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IAnnotationPatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("e0e3a35d-d167-46dc-95ab-330af61aebb5")
                    IAnnotationPatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AnnotationTypeIdProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AnnotationTypeNameProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AuthorProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DateTimeProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TargetProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAnnotationPatternIdentifiersStatics = __uuidof(IAnnotationPatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationAnnotation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationAnnotation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationAnnotation[] = L"Windows.UI.Xaml.Automation.IAutomationAnnotation";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("fb3c30ca-03d8-4618-91bf-e4d84f4af318")
                    IAutomationAnnotation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Type(
                            ABI::Windows::UI::Xaml::Automation::AnnotationType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Type(
                            ABI::Windows::UI::Xaml::Automation::AnnotationType value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Element(
                            ABI::Windows::UI::Xaml::IUIElement** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Element(
                            ABI::Windows::UI::Xaml::IUIElement* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationAnnotation = __uuidof(IAutomationAnnotation);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationAnnotationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationAnnotation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationAnnotationFactory[] = L"Windows.UI.Xaml.Automation.IAutomationAnnotationFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("4906fa52-ddc0-4e69-b76b-019d928d822f")
                    IAutomationAnnotationFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            ABI::Windows::UI::Xaml::Automation::AnnotationType type,
                            ABI::Windows::UI::Xaml::Automation::IAutomationAnnotation** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateWithElementParameter(
                            ABI::Windows::UI::Xaml::Automation::AnnotationType type,
                            ABI::Windows::UI::Xaml::IUIElement* element,
                            ABI::Windows::UI::Xaml::Automation::IAutomationAnnotation** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationAnnotationFactory = __uuidof(IAutomationAnnotationFactory);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationAnnotationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationAnnotation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationAnnotationStatics[] = L"Windows.UI.Xaml.Automation.IAutomationAnnotationStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("e503eab7-4ee5-48cb-b5b8-bbcd46c9d1da")
                    IAutomationAnnotationStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TypeProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ElementProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationAnnotationStatics = __uuidof(IAutomationAnnotationStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiers[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("e68a63cf-4345-4e2d-8a6a-49cce1fa2dcc")
                    IAutomationElementIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IAutomationElementIdentifiers = __uuidof(IAutomationElementIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("4549399f-8340-4d67-b9bf-8c2ac6a0773a")
                    IAutomationElementIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AcceleratorKeyProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AccessKeyProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AutomationIdProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BoundingRectangleProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ClassNameProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ClickablePointProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ControlTypeProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HasKeyboardFocusProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HelpTextProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsContentElementProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsControlElementProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsEnabledProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsKeyboardFocusableProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsOffscreenProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsPasswordProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsRequiredForFormProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ItemStatusProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ItemTypeProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LabeledByProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LocalizedControlTypeProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NameProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OrientationProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LiveSettingProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationElementIdentifiersStatics = __uuidof(IAutomationElementIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiersStatics2[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("b5cbb1e2-d55f-46a9-9eda-1a4742515dc3")
                    IAutomationElementIdentifiersStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ControlledPeersProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationElementIdentifiersStatics2 = __uuidof(IAutomationElementIdentifiersStatics2);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiersStatics3[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("0f5cbebd-b3eb-4083-adc7-0c2f39bb3543")
                    IAutomationElementIdentifiersStatics3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PositionInSetProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SizeOfSetProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LevelProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AnnotationsProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationElementIdentifiersStatics3 = __uuidof(IAutomationElementIdentifiersStatics3);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiersStatics4[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("5af51f75-5913-4d78-b330-a6f50b73ed9b")
                    IAutomationElementIdentifiersStatics4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_LandmarkTypeProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LocalizedLandmarkTypeProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationElementIdentifiersStatics4 = __uuidof(IAutomationElementIdentifiersStatics4);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiersStatics5[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics5";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("986a8206-de59-42f9-a1e7-62b8af9e756d")
                    IAutomationElementIdentifiersStatics5 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsPeripheralProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsDataValidForFormProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FullDescriptionProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DescribedByProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FlowsToProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FlowsFromProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationElementIdentifiersStatics5 = __uuidof(IAutomationElementIdentifiersStatics5);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiersStatics6[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics6";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("de52b00d-8328-4eae-8035-f8db99c8bac4")
                    IAutomationElementIdentifiersStatics6 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CultureProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationElementIdentifiersStatics6 = __uuidof(IAutomationElementIdentifiersStatics6);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics7
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiersStatics7[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics7";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("00f1abb2-742c-446a-a8f6-1672b10d2874")
                    IAutomationElementIdentifiersStatics7 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_HeadingLevelProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationElementIdentifiersStatics7 = __uuidof(IAutomationElementIdentifiersStatics7);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics8
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiersStatics8[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics8";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("8517b060-806c-5dc5-bc41-891bb5a47adf")
                    IAutomationElementIdentifiersStatics8 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsDialogProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationElementIdentifiersStatics8 = __uuidof(IAutomationElementIdentifiersStatics8);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationProperties[] = L"Windows.UI.Xaml.Automation.IAutomationProperties";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("68d7232c-e622-48e9-af0b-1ffa33cc5cba")
                    IAutomationProperties : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IAutomationProperties = __uuidof(IAutomationProperties);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("b618fd7b-32d0-4970-9c42-7c039ac7be78")
                    IAutomationPropertiesStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AcceleratorKeyProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAcceleratorKey(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAcceleratorKey(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AccessKeyProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAccessKey(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAccessKey(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AutomationIdProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAutomationId(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAutomationId(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HelpTextProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetHelpText(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetHelpText(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsRequiredForFormProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetIsRequiredForForm(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetIsRequiredForForm(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ItemStatusProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetItemStatus(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetItemStatus(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ItemTypeProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetItemType(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetItemType(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LabeledByProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetLabeledBy(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::IUIElement** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetLabeledBy(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::IUIElement* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NameProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetName(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetName(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LiveSettingProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetLiveSetting(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::Automation::Peers::AutomationLiveSetting* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetLiveSetting(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::Automation::Peers::AutomationLiveSetting value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationPropertiesStatics = __uuidof(IAutomationPropertiesStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics2[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("3976547f-7089-4801-8f1d-aab78090d1a0")
                    IAutomationPropertiesStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AccessibilityViewProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAccessibilityView(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::Automation::Peers::AccessibilityView* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAccessibilityView(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::Automation::Peers::AccessibilityView value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ControlledPeersProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetControlledPeers(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            __FIVector_1_Windows__CUI__CXaml__CUIElement** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationPropertiesStatics2 = __uuidof(IAutomationPropertiesStatics2);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics3[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("7b75d735-5cb1-42ad-9b57-5faba8c1867f")
                    IAutomationPropertiesStatics3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PositionInSetProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetPositionInSet(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            INT32* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPositionInSet(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SizeOfSetProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetSizeOfSet(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            INT32* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetSizeOfSet(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LevelProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetLevel(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            INT32* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetLevel(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            INT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AnnotationsProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAnnotations(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationPropertiesStatics3 = __uuidof(IAutomationPropertiesStatics3);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics4[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("f7d62655-311a-4b7c-a131-524e89cd3cf9")
                    IAutomationPropertiesStatics4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_LandmarkTypeProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetLandmarkType(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::Automation::Peers::AutomationLandmarkType* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetLandmarkType(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::Automation::Peers::AutomationLandmarkType value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LocalizedLandmarkTypeProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetLocalizedLandmarkType(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetLocalizedLandmarkType(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationPropertiesStatics4 = __uuidof(IAutomationPropertiesStatics4);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics5[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics5";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("0be35b26-c8f9-41a2-b4db-e6a7a32b0c34")
                    IAutomationPropertiesStatics5 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsPeripheralProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetIsPeripheral(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetIsPeripheral(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsDataValidForFormProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetIsDataValidForForm(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetIsDataValidForForm(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FullDescriptionProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetFullDescription(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetFullDescription(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LocalizedControlTypeProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetLocalizedControlType(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetLocalizedControlType(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DescribedByProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDescribedBy(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            __FIVector_1_Windows__CUI__CXaml__CDependencyObject** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FlowsToProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetFlowsTo(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            __FIVector_1_Windows__CUI__CXaml__CDependencyObject** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FlowsFromProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetFlowsFrom(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            __FIVector_1_Windows__CUI__CXaml__CDependencyObject** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationPropertiesStatics5 = __uuidof(IAutomationPropertiesStatics5);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics6[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics6";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("c61e030f-eb49-4e5d-b012-4c1c96c3901b")
                    IAutomationPropertiesStatics6 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CultureProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetCulture(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            INT32* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetCulture(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            INT32 value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationPropertiesStatics6 = __uuidof(IAutomationPropertiesStatics6);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics7
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics7[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics7";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("f7e98bf3-8f91-4068-a4ad-b7b402d10a2c")
                    IAutomationPropertiesStatics7 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_HeadingLevelProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetHeadingLevel(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::Automation::Peers::AutomationHeadingLevel* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetHeadingLevel(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            ABI::Windows::UI::Xaml::Automation::Peers::AutomationHeadingLevel value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationPropertiesStatics7 = __uuidof(IAutomationPropertiesStatics7);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics8
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics8[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics8";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("432eca20-171a-560d-8524-3e651d3ad6ca")
                    IAutomationPropertiesStatics8 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsDialogProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetIsDialog(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetIsDialog(
                            ABI::Windows::UI::Xaml::IDependencyObject* element,
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationPropertiesStatics8 = __uuidof(IAutomationPropertiesStatics8);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics9
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 14.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics9[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics9";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("2f20b1d1-87b2-5562-8077-da593edafd2d")
                    IAutomationPropertiesStatics9 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AutomationControlTypeProperty(
                            ABI::Windows::UI::Xaml::IDependencyProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAutomationControlType(
                            ABI::Windows::UI::Xaml::IUIElement* element,
                            ABI::Windows::UI::Xaml::Automation::Peers::AutomationControlType* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetAutomationControlType(
                            ABI::Windows::UI::Xaml::IUIElement* element,
                            ABI::Windows::UI::Xaml::Automation::Peers::AutomationControlType value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAutomationPropertiesStatics9 = __uuidof(IAutomationPropertiesStatics9);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationProperty
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperty
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationProperty[] = L"Windows.UI.Xaml.Automation.IAutomationProperty";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("b627195b-3227-4e16-9534-ddece30ddb46")
                    IAutomationProperty : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IAutomationProperty = __uuidof(IAutomationProperty);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IDockPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.DockPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IDockPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IDockPatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("ccd7f4e6-e4f9-47ff-bde7-378b11f78e09")
                    IDockPatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IDockPatternIdentifiers = __uuidof(IDockPatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IDockPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.DockPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IDockPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IDockPatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("2b87245c-ed80-4fe5-8eb4-708a39c841e5")
                    IDockPatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DockPositionProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDockPatternIdentifiersStatics = __uuidof(IDockPatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IDragPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.DragPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IDragPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IDragPatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("6266e985-4d07-4e80-82eb-8f96690a1a0c")
                    IDragPatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IDragPatternIdentifiers = __uuidof(IDragPatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IDragPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.DragPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IDragPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IDragPatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("2a05379d-1755-4082-9d90-46f1411d7986")
                    IDragPatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DropEffectProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DropEffectsProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_GrabbedItemsProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsGrabbedProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDragPatternIdentifiersStatics = __uuidof(IDragPatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IDropTargetPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.DropTargetPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IDropTargetPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IDropTargetPatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("11865133-a6fe-4634-bd18-0ef612b7b208")
                    IDropTargetPatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IDropTargetPatternIdentifiers = __uuidof(IDropTargetPatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IDropTargetPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.DropTargetPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IDropTargetPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IDropTargetPatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("1b693304-89fb-4b0a-9452-ca2c66aaf9f3")
                    IDropTargetPatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DropTargetEffectProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DropTargetEffectsProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDropTargetPatternIdentifiersStatics = __uuidof(IDropTargetPatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IExpandCollapsePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IExpandCollapsePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IExpandCollapsePatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("b006bac0-751b-4d55-92cb-613ec1bdf5d0")
                    IExpandCollapsePatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IExpandCollapsePatternIdentifiers = __uuidof(IExpandCollapsePatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IExpandCollapsePatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IExpandCollapsePatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IExpandCollapsePatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("d7816fd4-6ee0-4f38-8e14-56ef21adacfd")
                    IExpandCollapsePatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ExpandCollapseStateProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IExpandCollapsePatternIdentifiersStatics = __uuidof(IExpandCollapsePatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IGridItemPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.GridItemPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IGridItemPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IGridItemPatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("757744f1-3285-4fb1-803b-2545bd431599")
                    IGridItemPatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IGridItemPatternIdentifiers = __uuidof(IGridItemPatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IGridItemPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.GridItemPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IGridItemPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IGridItemPatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("217d2402-5e46-4d61-8794-b8ee8e774714")
                    IGridItemPatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ColumnProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ColumnSpanProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContainingGridProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RowProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RowSpanProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGridItemPatternIdentifiersStatics = __uuidof(IGridItemPatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IGridPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.GridPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IGridPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IGridPatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("c902980f-96c5-450c-9044-7e52c24f9e94")
                    IGridPatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IGridPatternIdentifiers = __uuidof(IGridPatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IGridPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.GridPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IGridPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IGridPatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("7bc452f3-a181-4137-8de9-1f9b1a8320ed")
                    IGridPatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ColumnCountProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RowCountProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IGridPatternIdentifiersStatics = __uuidof(IGridPatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IMultipleViewPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.MultipleViewPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IMultipleViewPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IMultipleViewPatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("5d5cd3b8-1e12-488b-b0ea-5e6cb89816e1")
                    IMultipleViewPatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IMultipleViewPatternIdentifiers = __uuidof(IMultipleViewPatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IMultipleViewPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.MultipleViewPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IMultipleViewPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IMultipleViewPatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("a9cfa66f-6b84-4d71-9e48-d764d3bcda8e")
                    IMultipleViewPatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CurrentViewProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SupportedViewsProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMultipleViewPatternIdentifiersStatics = __uuidof(IMultipleViewPatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IRangeValuePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.RangeValuePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IRangeValuePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IRangeValuePatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("f8760f45-33c9-467d-bc9e-d1515263ace1")
                    IRangeValuePatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IRangeValuePatternIdentifiers = __uuidof(IRangeValuePatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IRangeValuePatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.RangeValuePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IRangeValuePatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IRangeValuePatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("ce23450f-1c27-457f-b815-7a5e46863dbb")
                    IRangeValuePatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsReadOnlyProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LargeChangeProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MaximumProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MinimumProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SmallChangeProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ValueProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IRangeValuePatternIdentifiersStatics = __uuidof(IRangeValuePatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IScrollPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.ScrollPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IScrollPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IScrollPatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("366b1003-425c-4951-ae83-d521e73bc696")
                    IScrollPatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IScrollPatternIdentifiers = __uuidof(IScrollPatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IScrollPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.ScrollPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IScrollPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IScrollPatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("4bf8e0a1-fb7f-4fa4-83b3-cfaeb103a685")
                    IScrollPatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_HorizontallyScrollableProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HorizontalScrollPercentProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HorizontalViewSizeProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NoScroll(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VerticallyScrollableProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VerticalScrollPercentProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VerticalViewSizeProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IScrollPatternIdentifiersStatics = __uuidof(IScrollPatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ISelectionItemPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.SelectionItemPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ISelectionItemPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ISelectionItemPatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("2dafa41a-3ef8-4bb5-a02b-3ee1b2274740")
                    ISelectionItemPatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ISelectionItemPatternIdentifiers = __uuidof(ISelectionItemPatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ISelectionItemPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.SelectionItemPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ISelectionItemPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.ISelectionItemPatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("a918d163-487e-4e3e-9f86-7b44acbe27ce")
                    ISelectionItemPatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsSelectedProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SelectionContainerProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISelectionItemPatternIdentifiersStatics = __uuidof(ISelectionItemPatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ISelectionPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.SelectionPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ISelectionPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ISelectionPatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("4aa66fb0-e3f7-475f-b78d-f8a83bb730c4")
                    ISelectionPatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ISelectionPatternIdentifiers = __uuidof(ISelectionPatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ISelectionPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.SelectionPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ISelectionPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.ISelectionPatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("93035b4c-6b50-40a1-b23f-5c78ddbd479a")
                    ISelectionPatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CanSelectMultipleProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsSelectionRequiredProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SelectionProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISelectionPatternIdentifiersStatics = __uuidof(ISelectionPatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ISpreadsheetItemPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ISpreadsheetItemPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ISpreadsheetItemPatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("84347e19-ca4b-46a2-a794-c87928a3b1ab")
                    ISpreadsheetItemPatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ISpreadsheetItemPatternIdentifiers = __uuidof(ISpreadsheetItemPatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ISpreadsheetItemPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ISpreadsheetItemPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.ISpreadsheetItemPatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("43658779-5380-4f12-b468-b4f368ad4499")
                    ISpreadsheetItemPatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FormulaProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ISpreadsheetItemPatternIdentifiersStatics = __uuidof(ISpreadsheetItemPatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IStylesPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.StylesPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IStylesPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IStylesPatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("b0e4e201-e89d-436b-8287-4f7903466879")
                    IStylesPatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IStylesPatternIdentifiers = __uuidof(IStylesPatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IStylesPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.StylesPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IStylesPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IStylesPatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("528a457a-bc3c-4d48-94af-1f68703ca296")
                    IStylesPatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ExtendedPropertiesProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FillColorProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FillPatternColorProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FillPatternStyleProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ShapeProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StyleIdProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StyleNameProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IStylesPatternIdentifiersStatics = __uuidof(IStylesPatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITableItemPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TableItemPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITableItemPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ITableItemPatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("c326e5ad-8077-4c64-98e4-e83bcf1b4389")
                    ITableItemPatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ITableItemPatternIdentifiers = __uuidof(ITableItemPatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITableItemPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TableItemPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITableItemPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.ITableItemPatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("24c4b923-e9a2-4de9-b2a4-a8b22d0be362")
                    ITableItemPatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ColumnHeaderItemsProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RowHeaderItemsProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITableItemPatternIdentifiersStatics = __uuidof(ITableItemPatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITablePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TablePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITablePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ITablePatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("38d104fe-0d0c-412a-bf8d-51ede683baf5")
                    ITablePatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ITablePatternIdentifiers = __uuidof(ITablePatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITablePatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TablePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITablePatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.ITablePatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("75073d25-32c9-4903-aecf-dc3504cbd244")
                    ITablePatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ColumnHeadersProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RowHeadersProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RowOrColumnMajorProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITablePatternIdentifiersStatics = __uuidof(ITablePatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITogglePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TogglePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITogglePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ITogglePatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("7e191f6b-34d4-4ae7-83ac-29f88882d985")
                    ITogglePatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ITogglePatternIdentifiers = __uuidof(ITogglePatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITogglePatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TogglePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITogglePatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.ITogglePatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("c7f75544-14a5-4f2f-92fc-760524de06ea")
                    ITogglePatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ToggleStateProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITogglePatternIdentifiersStatics = __uuidof(ITogglePatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITransformPattern2Identifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TransformPattern2Identifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITransformPattern2Identifiers[] = L"Windows.UI.Xaml.Automation.ITransformPattern2Identifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("08aaa03d-dea7-402f-8097-9a2783d60e5d")
                    ITransformPattern2Identifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ITransformPattern2Identifiers = __uuidof(ITransformPattern2Identifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITransformPattern2IdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TransformPattern2Identifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITransformPattern2IdentifiersStatics[] = L"Windows.UI.Xaml.Automation.ITransformPattern2IdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("78963644-11f0-467c-a72b-5dac41c1f6fe")
                    ITransformPattern2IdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CanZoomProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ZoomLevelProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MaxZoomProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MinZoomProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITransformPattern2IdentifiersStatics = __uuidof(ITransformPattern2IdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITransformPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TransformPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITransformPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ITransformPatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("e4115b8c-c3c8-4a37-b994-2709a7811665")
                    ITransformPatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_ITransformPatternIdentifiers = __uuidof(ITransformPatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITransformPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TransformPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITransformPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.ITransformPatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("4570edab-d705-40c4-a1dc-e9acfcef85f6")
                    ITransformPatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CanMoveProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CanResizeProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CanRotateProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_ITransformPatternIdentifiersStatics = __uuidof(ITransformPatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IValuePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.ValuePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IValuePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IValuePatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("425bf64c-5333-4e41-b470-2bad14ecd085")
                    IValuePatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IValuePatternIdentifiers = __uuidof(IValuePatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IValuePatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.ValuePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IValuePatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IValuePatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("c247e8f7-adcc-440f-b123-33788a40525a")
                    IValuePatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsReadOnlyProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ValueProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IValuePatternIdentifiersStatics = __uuidof(IValuePatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IWindowPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.WindowPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IWindowPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IWindowPatternIdentifiers";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("39f78bb4-7032-41e2-b79e-27b74a8628de")
                    IWindowPatternIdentifiers : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IWindowPatternIdentifiers = __uuidof(IWindowPatternIdentifiers);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IWindowPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.WindowPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IWindowPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IWindowPatternIdentifiersStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Xaml {
                namespace Automation {
                    MIDL_INTERFACE("07d0ad06-6302-4d29-878b-19da03fc228d")
                    IWindowPatternIdentifiersStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_CanMaximizeProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CanMinimizeProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsModalProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsTopmostProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WindowInteractionStateProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WindowVisualStateProperty(
                            ABI::Windows::UI::Xaml::Automation::IAutomationProperty** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWindowPatternIdentifiersStatics = __uuidof(IWindowPatternIdentifiersStatics);
                } /* Automation */
            } /* Xaml */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.AnnotationPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAnnotationPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IAnnotationPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_AnnotationPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_AnnotationPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_AnnotationPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.AnnotationPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.AutomationAnnotation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.UI.Xaml.Automation.IAutomationAnnotationFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationAnnotationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IAutomationAnnotation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_AutomationAnnotation_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_AutomationAnnotation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_AutomationAnnotation[] = L"Windows.UI.Xaml.Automation.AutomationAnnotation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics6 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics5 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics7 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics4 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics3 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics8 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IAutomationElementIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_AutomationElementIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_AutomationElementIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_AutomationElementIdentifiers[] = L"Windows.UI.Xaml.Automation.AutomationElementIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.AutomationProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics6 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics5 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics7 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics9 interface starting with version 14.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics8 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics4 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics3 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IAutomationProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_AutomationProperties_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_AutomationProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_AutomationProperties[] = L"Windows.UI.Xaml.Automation.AutomationProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.AutomationProperty
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IAutomationProperty ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_AutomationProperty_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_AutomationProperty_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_AutomationProperty[] = L"Windows.UI.Xaml.Automation.AutomationProperty";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.DockPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IDockPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IDockPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_DockPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_DockPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_DockPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.DockPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.DragPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IDragPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IDragPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_DragPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_DragPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_DragPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.DragPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.DropTargetPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IDropTargetPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IDropTargetPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_DropTargetPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_DropTargetPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_DropTargetPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.DropTargetPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IExpandCollapsePatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IExpandCollapsePatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_ExpandCollapsePatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_ExpandCollapsePatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_ExpandCollapsePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.GridItemPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IGridItemPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IGridItemPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_GridItemPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_GridItemPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_GridItemPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.GridItemPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.GridPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IGridPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IGridPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_GridPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_GridPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_GridPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.GridPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.MultipleViewPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IMultipleViewPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IMultipleViewPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_MultipleViewPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_MultipleViewPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_MultipleViewPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.MultipleViewPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.RangeValuePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IRangeValuePatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IRangeValuePatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_RangeValuePatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_RangeValuePatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_RangeValuePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.RangeValuePatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.ScrollPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IScrollPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IScrollPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_ScrollPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_ScrollPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_ScrollPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ScrollPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.SelectionItemPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.ISelectionItemPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.ISelectionItemPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_SelectionItemPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_SelectionItemPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_SelectionItemPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.SelectionItemPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.SelectionPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.ISelectionPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.ISelectionPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_SelectionPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_SelectionPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_SelectionPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.SelectionPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.ISpreadsheetItemPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.ISpreadsheetItemPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_SpreadsheetItemPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_SpreadsheetItemPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_SpreadsheetItemPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.StylesPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IStylesPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IStylesPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_StylesPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_StylesPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_StylesPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.StylesPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.TableItemPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.ITableItemPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.ITableItemPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_TableItemPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_TableItemPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_TableItemPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.TableItemPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.TablePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.ITablePatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.ITablePatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_TablePatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_TablePatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_TablePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.TablePatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.TogglePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.ITogglePatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.ITogglePatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_TogglePatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_TogglePatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_TogglePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.TogglePatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.TransformPattern2Identifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.ITransformPattern2IdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.ITransformPattern2Identifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_TransformPattern2Identifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_TransformPattern2Identifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_TransformPattern2Identifiers[] = L"Windows.UI.Xaml.Automation.TransformPattern2Identifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.TransformPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.ITransformPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.ITransformPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_TransformPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_TransformPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_TransformPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.TransformPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.ValuePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IValuePatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IValuePatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_ValuePatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_ValuePatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_ValuePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ValuePatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.WindowPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IWindowPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IWindowPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_WindowPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_WindowPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_WindowPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.WindowPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2 __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3 __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4 __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5 __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6 __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7 __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8 __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2 __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3 __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4 __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5 __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6 __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7 __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8 __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9 __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics;

#endif // ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation;

typedef struct __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotationVtbl;

interface __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation __FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation;

typedef struct __FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        __FIIterator_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotationVtbl;

interface __FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIDependencyObject_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIDependencyObject_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIDependencyObject __x_ABI_CWindows_CUI_CXaml_CIDependencyObject;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIDependencyObject_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CXaml__CDependencyObject_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CXaml__CDependencyObject_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CXaml__CDependencyObject __FIIterator_1_Windows__CUI__CXaml__CDependencyObject;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CXaml__CDependencyObject;

typedef struct __FIIterator_1_Windows__CUI__CXaml__CDependencyObjectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CXaml__CDependencyObject* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CXaml__CDependencyObject* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CXaml__CDependencyObject* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CXaml__CDependencyObject* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CXaml__CDependencyObject* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CXaml__CDependencyObject* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CXaml__CDependencyObject* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CXaml__CDependencyObject* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CXaml__CDependencyObject* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CXaml__CDependencyObject* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CXaml__CDependencyObjectVtbl;

interface __FIIterator_1_Windows__CUI__CXaml__CDependencyObject
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CXaml__CDependencyObjectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CXaml__CDependencyObject_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CXaml__CDependencyObject_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CXaml__CDependencyObject_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CXaml__CDependencyObject_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CXaml__CDependencyObject_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CXaml__CDependencyObject_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CXaml__CDependencyObject_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CDependencyObject_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CDependencyObject_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CDependencyObject_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CXaml__CDependencyObject_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CXaml__CDependencyObject_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CXaml__CDependencyObject_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CXaml__CDependencyObject __FIIterable_1_Windows__CUI__CXaml__CDependencyObject;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CXaml__CDependencyObject;

typedef struct __FIIterable_1_Windows__CUI__CXaml__CDependencyObjectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CXaml__CDependencyObject* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CXaml__CDependencyObject* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CXaml__CDependencyObject* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CXaml__CDependencyObject* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CXaml__CDependencyObject* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CXaml__CDependencyObject* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CXaml__CDependencyObject* This,
        __FIIterator_1_Windows__CUI__CXaml__CDependencyObject** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CXaml__CDependencyObjectVtbl;

interface __FIIterable_1_Windows__CUI__CXaml__CDependencyObject
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CXaml__CDependencyObjectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CXaml__CDependencyObject_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CXaml__CDependencyObject_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CXaml__CDependencyObject_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CXaml__CDependencyObject_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CXaml__CDependencyObject_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CXaml__CDependencyObject_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CXaml__CDependencyObject_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CXaml__CDependencyObject_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIUIElement __x_ABI_CWindows_CUI_CXaml_CIUIElement;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIUIElement_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CXaml__CUIElement_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CXaml__CUIElement_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CXaml__CUIElement __FIIterator_1_Windows__CUI__CXaml__CUIElement;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CXaml__CUIElement;

typedef struct __FIIterator_1_Windows__CUI__CXaml__CUIElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CXaml__CUIElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CXaml__CUIElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CXaml__CUIElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CXaml__CUIElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CXaml__CUIElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CXaml__CUIElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CXaml__CUIElement* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CXaml__CUIElement* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CXaml__CUIElement* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CXaml__CUIElement* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CXaml__CUIElementVtbl;

interface __FIIterator_1_Windows__CUI__CXaml__CUIElement
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CXaml__CUIElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CXaml__CUIElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CXaml__CUIElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CXaml__CUIElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CXaml__CUIElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CXaml__CUIElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CXaml__CUIElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CXaml__CUIElement_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CUIElement_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CUIElement_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CXaml__CUIElement_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CXaml__CUIElement_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CXaml__CUIElement_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CXaml__CUIElement_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CXaml__CUIElement __FIIterable_1_Windows__CUI__CXaml__CUIElement;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CXaml__CUIElement;

typedef struct __FIIterable_1_Windows__CUI__CXaml__CUIElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CXaml__CUIElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CXaml__CUIElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CXaml__CUIElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CXaml__CUIElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CXaml__CUIElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CXaml__CUIElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CXaml__CUIElement* This,
        __FIIterator_1_Windows__CUI__CXaml__CUIElement** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CXaml__CUIElementVtbl;

interface __FIIterable_1_Windows__CUI__CXaml__CUIElement
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CXaml__CUIElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CXaml__CUIElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CXaml__CUIElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CXaml__CUIElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CXaml__CUIElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CXaml__CUIElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CXaml__CUIElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CXaml__CUIElement_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CXaml__CUIElement_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation;

typedef struct __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotationVtbl;

interface __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CXaml__CDependencyObject __FIVectorView_1_Windows__CUI__CXaml__CDependencyObject;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CXaml__CDependencyObject;

typedef struct __FIVectorView_1_Windows__CUI__CXaml__CDependencyObjectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CXaml__CDependencyObject* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CXaml__CDependencyObject* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CXaml__CDependencyObject* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CXaml__CDependencyObject* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CXaml__CDependencyObject* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CXaml__CDependencyObject* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CXaml__CDependencyObject* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CXaml__CDependencyObject* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CXaml__CDependencyObject* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CXaml__CDependencyObject* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CXaml__CDependencyObjectVtbl;

interface __FIVectorView_1_Windows__CUI__CXaml__CDependencyObject
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CXaml__CDependencyObjectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CXaml__CDependencyObject_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CXaml__CUIElement_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CXaml__CUIElement_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CXaml__CUIElement __FIVectorView_1_Windows__CUI__CXaml__CUIElement;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CXaml__CUIElement;

typedef struct __FIVectorView_1_Windows__CUI__CXaml__CUIElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CXaml__CUIElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CXaml__CUIElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CXaml__CUIElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CXaml__CUIElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CXaml__CUIElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CXaml__CUIElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CXaml__CUIElement* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CXaml__CUIElement* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CXaml__CUIElement* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CXaml__CUIElement* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CXaml__CUIElementVtbl;

interface __FIVectorView_1_Windows__CUI__CXaml__CUIElement
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CXaml__CUIElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CXaml__CUIElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CXaml__CUIElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CXaml__CUIElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CXaml__CUIElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CXaml__CUIElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CXaml__CUIElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CXaml__CUIElement_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CUIElement_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CUIElement_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CXaml__CUIElement_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CXaml__CUIElement_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation;

typedef struct __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        __FIVectorView_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotationVtbl;

interface __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CUI__CXaml__CDependencyObject_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CXaml__CDependencyObject_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CXaml__CDependencyObject __FIVector_1_Windows__CUI__CXaml__CDependencyObject;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CXaml__CDependencyObject;

typedef struct __FIVector_1_Windows__CUI__CXaml__CDependencyObjectVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This,
        __FIVectorView_1_Windows__CUI__CXaml__CDependencyObject** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CXaml__CDependencyObject* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CXaml__CDependencyObjectVtbl;

interface __FIVector_1_Windows__CUI__CXaml__CDependencyObject
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CXaml__CDependencyObjectVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CXaml__CDependencyObject_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CXaml__CDependencyObject_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CUI__CXaml__CUIElement_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CXaml__CUIElement_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CXaml__CUIElement __FIVector_1_Windows__CUI__CXaml__CUIElement;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CXaml__CUIElement;

typedef struct __FIVector_1_Windows__CUI__CXaml__CUIElementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This,
        __FIVectorView_1_Windows__CUI__CXaml__CUIElement** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CXaml__CUIElement* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CXaml__CUIElementVtbl;

interface __FIVector_1_Windows__CUI__CXaml__CUIElement
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CXaml__CUIElementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CXaml__CUIElement_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CXaml__CUIElement_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAccessibilityView __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAccessibilityView;

typedef enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAutomationControlType __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAutomationControlType;

typedef enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAutomationHeadingLevel __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAutomationHeadingLevel;

typedef enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAutomationLandmarkType __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAutomationLandmarkType;

typedef enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAutomationLiveSetting __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAutomationLiveSetting;

#ifndef ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty;

#endif // ____x_ABI_CWindows_CUI_CXaml_CIDependencyProperty_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CAnnotationType __x_ABI_CWindows_CUI_CXaml_CAutomation_CAnnotationType;

/*
 *
 * Struct Windows.UI.Xaml.Automation.AnnotationType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CAnnotationType
{
    AnnotationType_Unknown = 60000,
    AnnotationType_SpellingError = 60001,
    AnnotationType_GrammarError = 60002,
    AnnotationType_Comment = 60003,
    AnnotationType_FormulaError = 60004,
    AnnotationType_TrackChanges = 60005,
    AnnotationType_Header = 60006,
    AnnotationType_Footer = 60007,
    AnnotationType_Highlighted = 60008,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    AnnotationType_Endnote = 60009,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    AnnotationType_Footnote = 60010,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AnnotationType_InsertionChange = 60011,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AnnotationType_DeletionChange = 60012,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AnnotationType_MoveChange = 60013,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AnnotationType_FormatChange = 60014,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AnnotationType_UnsyncedChange = 60015,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AnnotationType_EditingLockedChange = 60016,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AnnotationType_ExternalChange = 60017,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AnnotationType_ConflictingChange = 60018,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AnnotationType_Author = 60019,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AnnotationType_AdvancedProofingIssue = 60020,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AnnotationType_DataValidationError = 60021,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AnnotationType_CircularReferenceError = 60022,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationActiveEnd
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CAutomationActiveEnd
{
    AutomationActiveEnd_None = 0,
    AutomationActiveEnd_Start = 1,
    AutomationActiveEnd_End = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationAnimationStyle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CAutomationAnimationStyle
{
    AutomationAnimationStyle_None = 0,
    AutomationAnimationStyle_LasVegasLights = 1,
    AutomationAnimationStyle_BlinkingBackground = 2,
    AutomationAnimationStyle_SparkleText = 3,
    AutomationAnimationStyle_MarchingBlackAnts = 4,
    AutomationAnimationStyle_MarchingRedAnts = 5,
    AutomationAnimationStyle_Shimmer = 6,
    AutomationAnimationStyle_Other = 7,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationBulletStyle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CAutomationBulletStyle
{
    AutomationBulletStyle_None = 0,
    AutomationBulletStyle_HollowRoundBullet = 1,
    AutomationBulletStyle_FilledRoundBullet = 2,
    AutomationBulletStyle_HollowSquareBullet = 3,
    AutomationBulletStyle_FilledSquareBullet = 4,
    AutomationBulletStyle_DashBullet = 5,
    AutomationBulletStyle_Other = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationCaretBidiMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CAutomationCaretBidiMode
{
    AutomationCaretBidiMode_LTR = 0,
    AutomationCaretBidiMode_RTL = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationCaretPosition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CAutomationCaretPosition
{
    AutomationCaretPosition_Unknown = 0,
    AutomationCaretPosition_EndOfLine = 1,
    AutomationCaretPosition_BeginningOfLine = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationFlowDirections
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CAutomationFlowDirections
{
    AutomationFlowDirections_Default = 0,
    AutomationFlowDirections_RightToLeft = 1,
    AutomationFlowDirections_BottomToTop = 2,
    AutomationFlowDirections_Vertical = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationOutlineStyles
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CAutomationOutlineStyles
{
    AutomationOutlineStyles_None = 0,
    AutomationOutlineStyles_Outline = 1,
    AutomationOutlineStyles_Shadow = 2,
    AutomationOutlineStyles_Engraved = 3,
    AutomationOutlineStyles_Embossed = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationStyleId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CAutomationStyleId
{
    AutomationStyleId_Heading1 = 70001,
    AutomationStyleId_Heading2 = 70002,
    AutomationStyleId_Heading3 = 70003,
    AutomationStyleId_Heading4 = 70004,
    AutomationStyleId_Heading5 = 70005,
    AutomationStyleId_Heading6 = 70006,
    AutomationStyleId_Heading7 = 70007,
    AutomationStyleId_Heading8 = 70008,
    AutomationStyleId_Heading9 = 70009,
    AutomationStyleId_Title = 70010,
    AutomationStyleId_Subtitle = 70011,
    AutomationStyleId_Normal = 70012,
    AutomationStyleId_Emphasis = 70013,
    AutomationStyleId_Quote = 70014,
    AutomationStyleId_BulletedList = 70015,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationTextDecorationLineStyle
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CAutomationTextDecorationLineStyle
{
    AutomationTextDecorationLineStyle_None = 0,
    AutomationTextDecorationLineStyle_Single = 1,
    AutomationTextDecorationLineStyle_WordsOnly = 2,
    AutomationTextDecorationLineStyle_Double = 3,
    AutomationTextDecorationLineStyle_Dot = 4,
    AutomationTextDecorationLineStyle_Dash = 5,
    AutomationTextDecorationLineStyle_DashDot = 6,
    AutomationTextDecorationLineStyle_DashDotDot = 7,
    AutomationTextDecorationLineStyle_Wavy = 8,
    AutomationTextDecorationLineStyle_ThickSingle = 9,
    AutomationTextDecorationLineStyle_DoubleWavy = 10,
    AutomationTextDecorationLineStyle_ThickWavy = 11,
    AutomationTextDecorationLineStyle_LongDash = 12,
    AutomationTextDecorationLineStyle_ThickDash = 13,
    AutomationTextDecorationLineStyle_ThickDashDot = 14,
    AutomationTextDecorationLineStyle_ThickDashDotDot = 15,
    AutomationTextDecorationLineStyle_ThickDot = 16,
    AutomationTextDecorationLineStyle_ThickLongDash = 17,
    AutomationTextDecorationLineStyle_Other = 18,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.AutomationTextEditChangeType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CAutomationTextEditChangeType
{
    AutomationTextEditChangeType_None = 0,
    AutomationTextEditChangeType_AutoCorrect = 1,
    AutomationTextEditChangeType_Composition = 2,
    AutomationTextEditChangeType_CompositionFinalized = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.DockPosition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CDockPosition
{
    DockPosition_Top = 0,
    DockPosition_Left = 1,
    DockPosition_Bottom = 2,
    DockPosition_Right = 3,
    DockPosition_Fill = 4,
    DockPosition_None = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.ExpandCollapseState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CExpandCollapseState
{
    ExpandCollapseState_Collapsed = 0,
    ExpandCollapseState_Expanded = 1,
    ExpandCollapseState_PartiallyExpanded = 2,
    ExpandCollapseState_LeafNode = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.RowOrColumnMajor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CRowOrColumnMajor
{
    RowOrColumnMajor_RowMajor = 0,
    RowOrColumnMajor_ColumnMajor = 1,
    RowOrColumnMajor_Indeterminate = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.ScrollAmount
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CScrollAmount
{
    ScrollAmount_LargeDecrement = 0,
    ScrollAmount_SmallDecrement = 1,
    ScrollAmount_NoAmount = 2,
    ScrollAmount_LargeIncrement = 3,
    ScrollAmount_SmallIncrement = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.SupportedTextSelection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CSupportedTextSelection
{
    SupportedTextSelection_None = 0,
    SupportedTextSelection_Single = 1,
    SupportedTextSelection_Multiple = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.SynchronizedInputType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CSynchronizedInputType
{
    SynchronizedInputType_KeyUp = 1,
    SynchronizedInputType_KeyDown = 2,
    SynchronizedInputType_LeftMouseUp = 4,
    SynchronizedInputType_LeftMouseDown = 8,
    SynchronizedInputType_RightMouseUp = 16,
    SynchronizedInputType_RightMouseDown = 32,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.ToggleState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CToggleState
{
    ToggleState_Off = 0,
    ToggleState_On = 1,
    ToggleState_Indeterminate = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.WindowInteractionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CWindowInteractionState
{
    WindowInteractionState_Running = 0,
    WindowInteractionState_Closing = 1,
    WindowInteractionState_ReadyForUserInteraction = 2,
    WindowInteractionState_BlockedByModalWindow = 3,
    WindowInteractionState_NotResponding = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.WindowVisualState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CWindowVisualState
{
    WindowVisualState_Normal = 0,
    WindowVisualState_Maximized = 1,
    WindowVisualState_Minimized = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Xaml.Automation.ZoomUnit
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CZoomUnit
{
    ZoomUnit_NoAmount = 0,
    ZoomUnit_LargeDecrement = 1,
    ZoomUnit_SmallDecrement = 2,
    ZoomUnit_LargeIncrement = 3,
    ZoomUnit_SmallIncrement = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAnnotationPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AnnotationPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAnnotationPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IAnnotationPatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAnnotationPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AnnotationPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAnnotationPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IAnnotationPatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AnnotationTypeIdProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_AnnotationTypeNameProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_AuthorProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_DateTimeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_TargetProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_get_AnnotationTypeIdProperty(This, value) \
    ((This)->lpVtbl->get_AnnotationTypeIdProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_get_AnnotationTypeNameProperty(This, value) \
    ((This)->lpVtbl->get_AnnotationTypeNameProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_get_AuthorProperty(This, value) \
    ((This)->lpVtbl->get_AuthorProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_get_DateTimeProperty(This, value) \
    ((This)->lpVtbl->get_DateTimeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_get_TargetProperty(This, value) \
    ((This)->lpVtbl->get_TargetProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAnnotationPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationAnnotation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationAnnotation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationAnnotation[] = L"Windows.UI.Xaml.Automation.IAutomationAnnotation";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation* This,
        enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CAnnotationType* value);
    HRESULT (STDMETHODCALLTYPE* put_Type)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation* This,
        enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CAnnotationType value);
    HRESULT (STDMETHODCALLTYPE* get_Element)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement** value);
    HRESULT (STDMETHODCALLTYPE* put_Element)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_put_Type(This, value) \
    ((This)->lpVtbl->put_Type(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_get_Element(This, value) \
    ((This)->lpVtbl->get_Element(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_put_Element(This, value) \
    ((This)->lpVtbl->put_Element(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationAnnotationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationAnnotation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationAnnotationFactory[] = L"Windows.UI.Xaml.Automation.IAutomationAnnotationFactory";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory* This,
        enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CAnnotationType type,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithElementParameter)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory* This,
        enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CAnnotationType type,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* element,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotation** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactoryVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_CreateInstance(This, type, value) \
    ((This)->lpVtbl->CreateInstance(This, type, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_CreateWithElementParameter(This, type, element, value) \
    ((This)->lpVtbl->CreateWithElementParameter(This, type, element, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationAnnotationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationAnnotation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationAnnotationStatics[] = L"Windows.UI.Xaml.Automation.IAutomationAnnotationStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TypeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ElementProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_get_TypeProperty(This, value) \
    ((This)->lpVtbl->get_TypeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_get_ElementProperty(This, value) \
    ((This)->lpVtbl->get_ElementProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationAnnotationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiers[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AcceleratorKeyProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_AccessKeyProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_AutomationIdProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_BoundingRectangleProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ClassNameProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ClickablePointProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ControlTypeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_HasKeyboardFocusProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_HelpTextProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_IsContentElementProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_IsControlElementProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabledProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_IsKeyboardFocusableProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_IsOffscreenProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_IsPasswordProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_IsRequiredForFormProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ItemStatusProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ItemTypeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_LabeledByProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_LocalizedControlTypeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_NameProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_OrientationProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_LiveSettingProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_AcceleratorKeyProperty(This, value) \
    ((This)->lpVtbl->get_AcceleratorKeyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_AccessKeyProperty(This, value) \
    ((This)->lpVtbl->get_AccessKeyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_AutomationIdProperty(This, value) \
    ((This)->lpVtbl->get_AutomationIdProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_BoundingRectangleProperty(This, value) \
    ((This)->lpVtbl->get_BoundingRectangleProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_ClassNameProperty(This, value) \
    ((This)->lpVtbl->get_ClassNameProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_ClickablePointProperty(This, value) \
    ((This)->lpVtbl->get_ClickablePointProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_ControlTypeProperty(This, value) \
    ((This)->lpVtbl->get_ControlTypeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_HasKeyboardFocusProperty(This, value) \
    ((This)->lpVtbl->get_HasKeyboardFocusProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_HelpTextProperty(This, value) \
    ((This)->lpVtbl->get_HelpTextProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_IsContentElementProperty(This, value) \
    ((This)->lpVtbl->get_IsContentElementProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_IsControlElementProperty(This, value) \
    ((This)->lpVtbl->get_IsControlElementProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_IsEnabledProperty(This, value) \
    ((This)->lpVtbl->get_IsEnabledProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_IsKeyboardFocusableProperty(This, value) \
    ((This)->lpVtbl->get_IsKeyboardFocusableProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_IsOffscreenProperty(This, value) \
    ((This)->lpVtbl->get_IsOffscreenProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_IsPasswordProperty(This, value) \
    ((This)->lpVtbl->get_IsPasswordProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_IsRequiredForFormProperty(This, value) \
    ((This)->lpVtbl->get_IsRequiredForFormProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_ItemStatusProperty(This, value) \
    ((This)->lpVtbl->get_ItemStatusProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_ItemTypeProperty(This, value) \
    ((This)->lpVtbl->get_ItemTypeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_LabeledByProperty(This, value) \
    ((This)->lpVtbl->get_LabeledByProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_LocalizedControlTypeProperty(This, value) \
    ((This)->lpVtbl->get_LocalizedControlTypeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_NameProperty(This, value) \
    ((This)->lpVtbl->get_NameProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_OrientationProperty(This, value) \
    ((This)->lpVtbl->get_OrientationProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_get_LiveSettingProperty(This, value) \
    ((This)->lpVtbl->get_LiveSettingProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiersStatics2[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ControlledPeersProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_get_ControlledPeersProperty(This, value) \
    ((This)->lpVtbl->get_ControlledPeersProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiersStatics3[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics3";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PositionInSetProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_SizeOfSetProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_LevelProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_AnnotationsProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_get_PositionInSetProperty(This, value) \
    ((This)->lpVtbl->get_PositionInSetProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_get_SizeOfSetProperty(This, value) \
    ((This)->lpVtbl->get_SizeOfSetProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_get_LevelProperty(This, value) \
    ((This)->lpVtbl->get_LevelProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_get_AnnotationsProperty(This, value) \
    ((This)->lpVtbl->get_AnnotationsProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiersStatics4[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics4";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LandmarkTypeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_LocalizedLandmarkTypeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_get_LandmarkTypeProperty(This, value) \
    ((This)->lpVtbl->get_LandmarkTypeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_get_LocalizedLandmarkTypeProperty(This, value) \
    ((This)->lpVtbl->get_LocalizedLandmarkTypeProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiersStatics5[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics5";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsPeripheralProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_IsDataValidForFormProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_FullDescriptionProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_DescribedByProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_FlowsToProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_FlowsFromProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_get_IsPeripheralProperty(This, value) \
    ((This)->lpVtbl->get_IsPeripheralProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_get_IsDataValidForFormProperty(This, value) \
    ((This)->lpVtbl->get_IsDataValidForFormProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_get_FullDescriptionProperty(This, value) \
    ((This)->lpVtbl->get_FullDescriptionProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_get_DescribedByProperty(This, value) \
    ((This)->lpVtbl->get_DescribedByProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_get_FlowsToProperty(This, value) \
    ((This)->lpVtbl->get_FlowsToProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_get_FlowsFromProperty(This, value) \
    ((This)->lpVtbl->get_FlowsFromProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiersStatics6[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics6";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CultureProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_get_CultureProperty(This, value) \
    ((This)->lpVtbl->get_CultureProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics7
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiersStatics7[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics7";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HeadingLevelProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_get_HeadingLevelProperty(This, value) \
    ((This)->lpVtbl->get_HeadingLevelProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics7_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics8
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationElementIdentifiersStatics8[] = L"Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics8";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsDialogProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_get_IsDialogProperty(This, value) \
    ((This)->lpVtbl->get_IsDialogProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationElementIdentifiersStatics8_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationProperties[] = L"Windows.UI.Xaml.Automation.IAutomationProperties";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AcceleratorKeyProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetAcceleratorKey)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* SetAcceleratorKey)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_AccessKeyProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetAccessKey)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* SetAccessKey)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_AutomationIdProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetAutomationId)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* SetAutomationId)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_HelpTextProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetHelpText)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* SetHelpText)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IsRequiredForFormProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetIsRequiredForForm)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetIsRequiredForForm)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ItemStatusProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetItemStatus)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* SetItemStatus)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ItemTypeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetItemType)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* SetItemType)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_LabeledByProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetLabeledBy)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement** result);
    HRESULT (STDMETHODCALLTYPE* SetLabeledBy)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* value);
    HRESULT (STDMETHODCALLTYPE* get_NameProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* SetName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_LiveSettingProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetLiveSetting)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAutomationLiveSetting* result);
    HRESULT (STDMETHODCALLTYPE* SetLiveSetting)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAutomationLiveSetting value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_get_AcceleratorKeyProperty(This, value) \
    ((This)->lpVtbl->get_AcceleratorKeyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_GetAcceleratorKey(This, element, result) \
    ((This)->lpVtbl->GetAcceleratorKey(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_SetAcceleratorKey(This, element, value) \
    ((This)->lpVtbl->SetAcceleratorKey(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_get_AccessKeyProperty(This, value) \
    ((This)->lpVtbl->get_AccessKeyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_GetAccessKey(This, element, result) \
    ((This)->lpVtbl->GetAccessKey(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_SetAccessKey(This, element, value) \
    ((This)->lpVtbl->SetAccessKey(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_get_AutomationIdProperty(This, value) \
    ((This)->lpVtbl->get_AutomationIdProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_GetAutomationId(This, element, result) \
    ((This)->lpVtbl->GetAutomationId(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_SetAutomationId(This, element, value) \
    ((This)->lpVtbl->SetAutomationId(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_get_HelpTextProperty(This, value) \
    ((This)->lpVtbl->get_HelpTextProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_GetHelpText(This, element, result) \
    ((This)->lpVtbl->GetHelpText(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_SetHelpText(This, element, value) \
    ((This)->lpVtbl->SetHelpText(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_get_IsRequiredForFormProperty(This, value) \
    ((This)->lpVtbl->get_IsRequiredForFormProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_GetIsRequiredForForm(This, element, result) \
    ((This)->lpVtbl->GetIsRequiredForForm(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_SetIsRequiredForForm(This, element, value) \
    ((This)->lpVtbl->SetIsRequiredForForm(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_get_ItemStatusProperty(This, value) \
    ((This)->lpVtbl->get_ItemStatusProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_GetItemStatus(This, element, result) \
    ((This)->lpVtbl->GetItemStatus(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_SetItemStatus(This, element, value) \
    ((This)->lpVtbl->SetItemStatus(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_get_ItemTypeProperty(This, value) \
    ((This)->lpVtbl->get_ItemTypeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_GetItemType(This, element, result) \
    ((This)->lpVtbl->GetItemType(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_SetItemType(This, element, value) \
    ((This)->lpVtbl->SetItemType(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_get_LabeledByProperty(This, value) \
    ((This)->lpVtbl->get_LabeledByProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_GetLabeledBy(This, element, result) \
    ((This)->lpVtbl->GetLabeledBy(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_SetLabeledBy(This, element, value) \
    ((This)->lpVtbl->SetLabeledBy(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_get_NameProperty(This, value) \
    ((This)->lpVtbl->get_NameProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_GetName(This, element, result) \
    ((This)->lpVtbl->GetName(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_SetName(This, element, value) \
    ((This)->lpVtbl->SetName(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_get_LiveSettingProperty(This, value) \
    ((This)->lpVtbl->get_LiveSettingProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_GetLiveSetting(This, element, result) \
    ((This)->lpVtbl->GetLiveSetting(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_SetLiveSetting(This, element, value) \
    ((This)->lpVtbl->SetLiveSetting(This, element, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics2[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics2";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AccessibilityViewProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetAccessibilityView)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAccessibilityView* result);
    HRESULT (STDMETHODCALLTYPE* SetAccessibilityView)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAccessibilityView value);
    HRESULT (STDMETHODCALLTYPE* get_ControlledPeersProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetControlledPeers)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        __FIVector_1_Windows__CUI__CXaml__CUIElement** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_get_AccessibilityViewProperty(This, value) \
    ((This)->lpVtbl->get_AccessibilityViewProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_GetAccessibilityView(This, element, result) \
    ((This)->lpVtbl->GetAccessibilityView(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_SetAccessibilityView(This, element, value) \
    ((This)->lpVtbl->SetAccessibilityView(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_get_ControlledPeersProperty(This, value) \
    ((This)->lpVtbl->get_ControlledPeersProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_GetControlledPeers(This, element, result) \
    ((This)->lpVtbl->GetControlledPeers(This, element, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics3[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics3";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PositionInSetProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetPositionInSet)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* SetPositionInSet)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_SizeOfSetProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetSizeOfSet)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* SetSizeOfSet)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_LevelProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* SetLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_AnnotationsProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetAnnotations)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        __FIVector_1_Windows__CUI__CXaml__CAutomation__CAutomationAnnotation** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_get_PositionInSetProperty(This, value) \
    ((This)->lpVtbl->get_PositionInSetProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_GetPositionInSet(This, element, result) \
    ((This)->lpVtbl->GetPositionInSet(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_SetPositionInSet(This, element, value) \
    ((This)->lpVtbl->SetPositionInSet(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_get_SizeOfSetProperty(This, value) \
    ((This)->lpVtbl->get_SizeOfSetProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_GetSizeOfSet(This, element, result) \
    ((This)->lpVtbl->GetSizeOfSet(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_SetSizeOfSet(This, element, value) \
    ((This)->lpVtbl->SetSizeOfSet(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_get_LevelProperty(This, value) \
    ((This)->lpVtbl->get_LevelProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_GetLevel(This, element, result) \
    ((This)->lpVtbl->GetLevel(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_SetLevel(This, element, value) \
    ((This)->lpVtbl->SetLevel(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_get_AnnotationsProperty(This, value) \
    ((This)->lpVtbl->get_AnnotationsProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_GetAnnotations(This, element, result) \
    ((This)->lpVtbl->GetAnnotations(This, element, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics4[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics4";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LandmarkTypeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetLandmarkType)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAutomationLandmarkType* result);
    HRESULT (STDMETHODCALLTYPE* SetLandmarkType)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAutomationLandmarkType value);
    HRESULT (STDMETHODCALLTYPE* get_LocalizedLandmarkTypeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetLocalizedLandmarkType)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* SetLocalizedLandmarkType)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_get_LandmarkTypeProperty(This, value) \
    ((This)->lpVtbl->get_LandmarkTypeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_GetLandmarkType(This, element, result) \
    ((This)->lpVtbl->GetLandmarkType(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_SetLandmarkType(This, element, value) \
    ((This)->lpVtbl->SetLandmarkType(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_get_LocalizedLandmarkTypeProperty(This, value) \
    ((This)->lpVtbl->get_LocalizedLandmarkTypeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_GetLocalizedLandmarkType(This, element, result) \
    ((This)->lpVtbl->GetLocalizedLandmarkType(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_SetLocalizedLandmarkType(This, element, value) \
    ((This)->lpVtbl->SetLocalizedLandmarkType(This, element, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics5[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics5";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsPeripheralProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetIsPeripheral)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetIsPeripheral)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsDataValidForFormProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetIsDataValidForForm)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetIsDataValidForForm)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_FullDescriptionProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetFullDescription)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* SetFullDescription)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_LocalizedControlTypeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetLocalizedControlType)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* SetLocalizedControlType)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DescribedByProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetDescribedBy)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        __FIVector_1_Windows__CUI__CXaml__CDependencyObject** result);
    HRESULT (STDMETHODCALLTYPE* get_FlowsToProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetFlowsTo)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        __FIVector_1_Windows__CUI__CXaml__CDependencyObject** result);
    HRESULT (STDMETHODCALLTYPE* get_FlowsFromProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetFlowsFrom)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        __FIVector_1_Windows__CUI__CXaml__CDependencyObject** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_get_IsPeripheralProperty(This, value) \
    ((This)->lpVtbl->get_IsPeripheralProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_GetIsPeripheral(This, element, result) \
    ((This)->lpVtbl->GetIsPeripheral(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_SetIsPeripheral(This, element, value) \
    ((This)->lpVtbl->SetIsPeripheral(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_get_IsDataValidForFormProperty(This, value) \
    ((This)->lpVtbl->get_IsDataValidForFormProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_GetIsDataValidForForm(This, element, result) \
    ((This)->lpVtbl->GetIsDataValidForForm(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_SetIsDataValidForForm(This, element, value) \
    ((This)->lpVtbl->SetIsDataValidForForm(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_get_FullDescriptionProperty(This, value) \
    ((This)->lpVtbl->get_FullDescriptionProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_GetFullDescription(This, element, result) \
    ((This)->lpVtbl->GetFullDescription(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_SetFullDescription(This, element, value) \
    ((This)->lpVtbl->SetFullDescription(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_get_LocalizedControlTypeProperty(This, value) \
    ((This)->lpVtbl->get_LocalizedControlTypeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_GetLocalizedControlType(This, element, result) \
    ((This)->lpVtbl->GetLocalizedControlType(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_SetLocalizedControlType(This, element, value) \
    ((This)->lpVtbl->SetLocalizedControlType(This, element, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_get_DescribedByProperty(This, value) \
    ((This)->lpVtbl->get_DescribedByProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_GetDescribedBy(This, element, result) \
    ((This)->lpVtbl->GetDescribedBy(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_get_FlowsToProperty(This, value) \
    ((This)->lpVtbl->get_FlowsToProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_GetFlowsTo(This, element, result) \
    ((This)->lpVtbl->GetFlowsTo(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_get_FlowsFromProperty(This, value) \
    ((This)->lpVtbl->get_FlowsFromProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_GetFlowsFrom(This, element, result) \
    ((This)->lpVtbl->GetFlowsFrom(This, element, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics6[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics6";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CultureProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetCulture)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* SetCulture)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        INT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_get_CultureProperty(This, value) \
    ((This)->lpVtbl->get_CultureProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_GetCulture(This, element, result) \
    ((This)->lpVtbl->GetCulture(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_SetCulture(This, element, value) \
    ((This)->lpVtbl->SetCulture(This, element, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics7
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics7[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics7";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HeadingLevelProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetHeadingLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAutomationHeadingLevel* result);
    HRESULT (STDMETHODCALLTYPE* SetHeadingLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAutomationHeadingLevel value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_get_HeadingLevelProperty(This, value) \
    ((This)->lpVtbl->get_HeadingLevelProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_GetHeadingLevel(This, element, result) \
    ((This)->lpVtbl->GetHeadingLevel(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_SetHeadingLevel(This, element, value) \
    ((This)->lpVtbl->SetHeadingLevel(This, element, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics7_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics8
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics8[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics8";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsDialogProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetIsDialog)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetIsDialog)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyObject* element,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_get_IsDialogProperty(This, value) \
    ((This)->lpVtbl->get_IsDialogProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_GetIsDialog(This, element, result) \
    ((This)->lpVtbl->GetIsDialog(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_SetIsDialog(This, element, value) \
    ((This)->lpVtbl->SetIsDialog(This, element, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics8_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationPropertiesStatics9
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 14.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationPropertiesStatics9[] = L"Windows.UI.Xaml.Automation.IAutomationPropertiesStatics9";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AutomationControlTypeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9* This,
        __x_ABI_CWindows_CUI_CXaml_CIDependencyProperty** value);
    HRESULT (STDMETHODCALLTYPE* GetAutomationControlType)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* element,
        enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAutomationControlType* result);
    HRESULT (STDMETHODCALLTYPE* SetAutomationControlType)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9* This,
        __x_ABI_CWindows_CUI_CXaml_CIUIElement* element,
        enum __x_ABI_CWindows_CUI_CXaml_CAutomation_CPeers_CAutomationControlType value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9Vtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_get_AutomationControlTypeProperty(This, value) \
    ((This)->lpVtbl->get_AutomationControlTypeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_GetAutomationControlType(This, element, result) \
    ((This)->lpVtbl->GetAutomationControlType(This, element, result))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_SetAutomationControlType(This, element, value) \
    ((This)->lpVtbl->SetAutomationControlType(This, element, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertiesStatics9_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IAutomationProperty
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.AutomationProperty
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IAutomationProperty[] = L"Windows.UI.Xaml.Automation.IAutomationProperty";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertyVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertyVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationPropertyVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IDockPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.DockPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IDockPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IDockPatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IDockPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.DockPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IDockPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IDockPatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DockPositionProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_get_DockPositionProperty(This, value) \
    ((This)->lpVtbl->get_DockPositionProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDockPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IDragPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.DragPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IDragPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IDragPatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IDragPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.DragPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IDragPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IDragPatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DropEffectProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_DropEffectsProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_GrabbedItemsProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_IsGrabbedProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_get_DropEffectProperty(This, value) \
    ((This)->lpVtbl->get_DropEffectProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_get_DropEffectsProperty(This, value) \
    ((This)->lpVtbl->get_DropEffectsProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_get_GrabbedItemsProperty(This, value) \
    ((This)->lpVtbl->get_GrabbedItemsProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_get_IsGrabbedProperty(This, value) \
    ((This)->lpVtbl->get_IsGrabbedProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDragPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IDropTargetPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.DropTargetPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IDropTargetPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IDropTargetPatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IDropTargetPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.DropTargetPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IDropTargetPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IDropTargetPatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DropTargetEffectProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_DropTargetEffectsProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_get_DropTargetEffectProperty(This, value) \
    ((This)->lpVtbl->get_DropTargetEffectProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_get_DropTargetEffectsProperty(This, value) \
    ((This)->lpVtbl->get_DropTargetEffectsProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIDropTargetPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IExpandCollapsePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IExpandCollapsePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IExpandCollapsePatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IExpandCollapsePatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IExpandCollapsePatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IExpandCollapsePatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExpandCollapseStateProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_get_ExpandCollapseStateProperty(This, value) \
    ((This)->lpVtbl->get_ExpandCollapseStateProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIExpandCollapsePatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IGridItemPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.GridItemPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IGridItemPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IGridItemPatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IGridItemPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.GridItemPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IGridItemPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IGridItemPatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ColumnProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ColumnSpanProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ContainingGridProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_RowProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_RowSpanProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_get_ColumnProperty(This, value) \
    ((This)->lpVtbl->get_ColumnProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_get_ColumnSpanProperty(This, value) \
    ((This)->lpVtbl->get_ColumnSpanProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_get_ContainingGridProperty(This, value) \
    ((This)->lpVtbl->get_ContainingGridProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_get_RowProperty(This, value) \
    ((This)->lpVtbl->get_RowProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_get_RowSpanProperty(This, value) \
    ((This)->lpVtbl->get_RowSpanProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridItemPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IGridPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.GridPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IGridPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IGridPatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IGridPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.GridPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IGridPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IGridPatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ColumnCountProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_RowCountProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_get_ColumnCountProperty(This, value) \
    ((This)->lpVtbl->get_ColumnCountProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_get_RowCountProperty(This, value) \
    ((This)->lpVtbl->get_RowCountProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIGridPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IMultipleViewPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.MultipleViewPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IMultipleViewPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IMultipleViewPatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IMultipleViewPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.MultipleViewPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IMultipleViewPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IMultipleViewPatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CurrentViewProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedViewsProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_get_CurrentViewProperty(This, value) \
    ((This)->lpVtbl->get_CurrentViewProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_get_SupportedViewsProperty(This, value) \
    ((This)->lpVtbl->get_SupportedViewsProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIMultipleViewPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IRangeValuePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.RangeValuePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IRangeValuePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IRangeValuePatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IRangeValuePatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.RangeValuePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IRangeValuePatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IRangeValuePatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsReadOnlyProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_LargeChangeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_MaximumProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_MinimumProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_SmallChangeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ValueProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_get_IsReadOnlyProperty(This, value) \
    ((This)->lpVtbl->get_IsReadOnlyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_get_LargeChangeProperty(This, value) \
    ((This)->lpVtbl->get_LargeChangeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_get_MaximumProperty(This, value) \
    ((This)->lpVtbl->get_MaximumProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_get_MinimumProperty(This, value) \
    ((This)->lpVtbl->get_MinimumProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_get_SmallChangeProperty(This, value) \
    ((This)->lpVtbl->get_SmallChangeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_get_ValueProperty(This, value) \
    ((This)->lpVtbl->get_ValueProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIRangeValuePatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IScrollPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.ScrollPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IScrollPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IScrollPatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IScrollPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.ScrollPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IScrollPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IScrollPatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HorizontallyScrollableProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_HorizontalScrollPercentProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_HorizontalViewSizeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_NoScroll)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_VerticallyScrollableProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_VerticalScrollPercentProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_VerticalViewSizeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_get_HorizontallyScrollableProperty(This, value) \
    ((This)->lpVtbl->get_HorizontallyScrollableProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_get_HorizontalScrollPercentProperty(This, value) \
    ((This)->lpVtbl->get_HorizontalScrollPercentProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_get_HorizontalViewSizeProperty(This, value) \
    ((This)->lpVtbl->get_HorizontalViewSizeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_get_NoScroll(This, value) \
    ((This)->lpVtbl->get_NoScroll(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_get_VerticallyScrollableProperty(This, value) \
    ((This)->lpVtbl->get_VerticallyScrollableProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_get_VerticalScrollPercentProperty(This, value) \
    ((This)->lpVtbl->get_VerticalScrollPercentProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_get_VerticalViewSizeProperty(This, value) \
    ((This)->lpVtbl->get_VerticalViewSizeProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIScrollPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ISelectionItemPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.SelectionItemPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ISelectionItemPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ISelectionItemPatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ISelectionItemPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.SelectionItemPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ISelectionItemPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.ISelectionItemPatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsSelectedProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_SelectionContainerProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_get_IsSelectedProperty(This, value) \
    ((This)->lpVtbl->get_IsSelectedProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_get_SelectionContainerProperty(This, value) \
    ((This)->lpVtbl->get_SelectionContainerProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionItemPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ISelectionPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.SelectionPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ISelectionPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ISelectionPatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ISelectionPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.SelectionPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ISelectionPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.ISelectionPatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CanSelectMultipleProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_IsSelectionRequiredProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_SelectionProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_get_CanSelectMultipleProperty(This, value) \
    ((This)->lpVtbl->get_CanSelectMultipleProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_get_IsSelectionRequiredProperty(This, value) \
    ((This)->lpVtbl->get_IsSelectionRequiredProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_get_SelectionProperty(This, value) \
    ((This)->lpVtbl->get_SelectionProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISelectionPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ISpreadsheetItemPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ISpreadsheetItemPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ISpreadsheetItemPatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ISpreadsheetItemPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ISpreadsheetItemPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.ISpreadsheetItemPatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FormulaProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_get_FormulaProperty(This, value) \
    ((This)->lpVtbl->get_FormulaProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CISpreadsheetItemPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IStylesPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.StylesPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IStylesPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IStylesPatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IStylesPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.StylesPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IStylesPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IStylesPatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedPropertiesProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_FillColorProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_FillPatternColorProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_FillPatternStyleProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ShapeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_StyleIdProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_StyleNameProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_get_ExtendedPropertiesProperty(This, value) \
    ((This)->lpVtbl->get_ExtendedPropertiesProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_get_FillColorProperty(This, value) \
    ((This)->lpVtbl->get_FillColorProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_get_FillPatternColorProperty(This, value) \
    ((This)->lpVtbl->get_FillPatternColorProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_get_FillPatternStyleProperty(This, value) \
    ((This)->lpVtbl->get_FillPatternStyleProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_get_ShapeProperty(This, value) \
    ((This)->lpVtbl->get_ShapeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_get_StyleIdProperty(This, value) \
    ((This)->lpVtbl->get_StyleIdProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_get_StyleNameProperty(This, value) \
    ((This)->lpVtbl->get_StyleNameProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIStylesPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITableItemPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TableItemPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITableItemPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ITableItemPatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITableItemPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TableItemPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITableItemPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.ITableItemPatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ColumnHeaderItemsProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_RowHeaderItemsProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_get_ColumnHeaderItemsProperty(This, value) \
    ((This)->lpVtbl->get_ColumnHeaderItemsProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_get_RowHeaderItemsProperty(This, value) \
    ((This)->lpVtbl->get_RowHeaderItemsProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITableItemPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITablePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TablePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITablePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ITablePatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITablePatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TablePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITablePatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.ITablePatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ColumnHeadersProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_RowHeadersProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_RowOrColumnMajorProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_get_ColumnHeadersProperty(This, value) \
    ((This)->lpVtbl->get_ColumnHeadersProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_get_RowHeadersProperty(This, value) \
    ((This)->lpVtbl->get_RowHeadersProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_get_RowOrColumnMajorProperty(This, value) \
    ((This)->lpVtbl->get_RowOrColumnMajorProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITablePatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITogglePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TogglePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITogglePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ITogglePatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITogglePatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TogglePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITogglePatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.ITogglePatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ToggleStateProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_get_ToggleStateProperty(This, value) \
    ((This)->lpVtbl->get_ToggleStateProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITogglePatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITransformPattern2Identifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TransformPattern2Identifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITransformPattern2Identifiers[] = L"Windows.UI.Xaml.Automation.ITransformPattern2Identifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2Identifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITransformPattern2IdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TransformPattern2Identifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITransformPattern2IdentifiersStatics[] = L"Windows.UI.Xaml.Automation.ITransformPattern2IdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CanZoomProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ZoomLevelProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_MaxZoomProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_MinZoomProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_get_CanZoomProperty(This, value) \
    ((This)->lpVtbl->get_CanZoomProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_get_ZoomLevelProperty(This, value) \
    ((This)->lpVtbl->get_ZoomLevelProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_get_MaxZoomProperty(This, value) \
    ((This)->lpVtbl->get_MaxZoomProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_get_MinZoomProperty(This, value) \
    ((This)->lpVtbl->get_MinZoomProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPattern2IdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITransformPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TransformPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITransformPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ITransformPatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.ITransformPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.TransformPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_ITransformPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.ITransformPatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CanMoveProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_CanResizeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_CanRotateProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_get_CanMoveProperty(This, value) \
    ((This)->lpVtbl->get_CanMoveProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_get_CanResizeProperty(This, value) \
    ((This)->lpVtbl->get_CanResizeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_get_CanRotateProperty(This, value) \
    ((This)->lpVtbl->get_CanRotateProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CITransformPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IValuePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.ValuePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IValuePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IValuePatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IValuePatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.ValuePatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IValuePatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IValuePatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsReadOnlyProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_ValueProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_get_IsReadOnlyProperty(This, value) \
    ((This)->lpVtbl->get_IsReadOnlyProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_get_ValueProperty(This, value) \
    ((This)->lpVtbl->get_ValueProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIValuePatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IWindowPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.WindowPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IWindowPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.IWindowPatternIdentifiers";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Xaml.Automation.IWindowPatternIdentifiersStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Xaml.Automation.WindowPatternIdentifiers
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Xaml_Automation_IWindowPatternIdentifiersStatics[] = L"Windows.UI.Xaml.Automation.IWindowPatternIdentifiersStatics";
typedef struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CanMaximizeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_CanMinimizeProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_IsModalProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_IsTopmostProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_WindowInteractionStateProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);
    HRESULT (STDMETHODCALLTYPE* get_WindowVisualStateProperty)(__x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics* This,
        __x_ABI_CWindows_CUI_CXaml_CAutomation_CIAutomationProperty** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStaticsVtbl;

interface __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_get_CanMaximizeProperty(This, value) \
    ((This)->lpVtbl->get_CanMaximizeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_get_CanMinimizeProperty(This, value) \
    ((This)->lpVtbl->get_CanMinimizeProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_get_IsModalProperty(This, value) \
    ((This)->lpVtbl->get_IsModalProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_get_IsTopmostProperty(This, value) \
    ((This)->lpVtbl->get_IsTopmostProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_get_WindowInteractionStateProperty(This, value) \
    ((This)->lpVtbl->get_WindowInteractionStateProperty(This, value))

#define __x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_get_WindowVisualStateProperty(This, value) \
    ((This)->lpVtbl->get_WindowVisualStateProperty(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CXaml_CAutomation_CIWindowPatternIdentifiersStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.AnnotationPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAnnotationPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IAnnotationPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_AnnotationPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_AnnotationPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_AnnotationPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.AnnotationPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.AutomationAnnotation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.UI.Xaml.Automation.IAutomationAnnotationFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationAnnotationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IAutomationAnnotation ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_AutomationAnnotation_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_AutomationAnnotation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_AutomationAnnotation[] = L"Windows.UI.Xaml.Automation.AutomationAnnotation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.AutomationElementIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics6 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics5 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics7 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics4 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics3 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationElementIdentifiersStatics8 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IAutomationElementIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_AutomationElementIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_AutomationElementIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_AutomationElementIdentifiers[] = L"Windows.UI.Xaml.Automation.AutomationElementIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.AutomationProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics6 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics5 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics7 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics9 interface starting with version 14.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics8 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics4 interface starting with version 2.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics3 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IAutomationPropertiesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IAutomationProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_AutomationProperties_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_AutomationProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_AutomationProperties[] = L"Windows.UI.Xaml.Automation.AutomationProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.AutomationProperty
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IAutomationProperty ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_AutomationProperty_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_AutomationProperty_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_AutomationProperty[] = L"Windows.UI.Xaml.Automation.AutomationProperty";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.DockPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IDockPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IDockPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_DockPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_DockPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_DockPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.DockPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.DragPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IDragPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IDragPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_DragPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_DragPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_DragPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.DragPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.DropTargetPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IDropTargetPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IDropTargetPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_DropTargetPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_DropTargetPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_DropTargetPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.DropTargetPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IExpandCollapsePatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IExpandCollapsePatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_ExpandCollapsePatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_ExpandCollapsePatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_ExpandCollapsePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ExpandCollapsePatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.GridItemPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IGridItemPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IGridItemPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_GridItemPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_GridItemPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_GridItemPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.GridItemPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.GridPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IGridPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IGridPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_GridPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_GridPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_GridPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.GridPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.MultipleViewPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IMultipleViewPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IMultipleViewPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_MultipleViewPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_MultipleViewPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_MultipleViewPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.MultipleViewPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.RangeValuePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IRangeValuePatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IRangeValuePatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_RangeValuePatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_RangeValuePatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_RangeValuePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.RangeValuePatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.ScrollPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IScrollPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IScrollPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_ScrollPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_ScrollPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_ScrollPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ScrollPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.SelectionItemPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.ISelectionItemPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.ISelectionItemPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_SelectionItemPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_SelectionItemPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_SelectionItemPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.SelectionItemPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.SelectionPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.ISelectionPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.ISelectionPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_SelectionPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_SelectionPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_SelectionPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.SelectionPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.ISpreadsheetItemPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.ISpreadsheetItemPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_SpreadsheetItemPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_SpreadsheetItemPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_SpreadsheetItemPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.SpreadsheetItemPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.StylesPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IStylesPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IStylesPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_StylesPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_StylesPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_StylesPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.StylesPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.TableItemPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.ITableItemPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.ITableItemPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_TableItemPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_TableItemPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_TableItemPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.TableItemPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.TablePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.ITablePatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.ITablePatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_TablePatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_TablePatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_TablePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.TablePatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.TogglePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.ITogglePatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.ITogglePatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_TogglePatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_TogglePatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_TogglePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.TogglePatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.TransformPattern2Identifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.ITransformPattern2IdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.ITransformPattern2Identifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_TransformPattern2Identifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_TransformPattern2Identifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_TransformPattern2Identifiers[] = L"Windows.UI.Xaml.Automation.TransformPattern2Identifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.TransformPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.ITransformPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.ITransformPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_TransformPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_TransformPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_TransformPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.TransformPatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.ValuePatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IValuePatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IValuePatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_ValuePatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_ValuePatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_ValuePatternIdentifiers[] = L"Windows.UI.Xaml.Automation.ValuePatternIdentifiers";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Xaml.Automation.WindowPatternIdentifiers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Xaml.Automation.IWindowPatternIdentifiersStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Xaml.Automation.IWindowPatternIdentifiers ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Xaml_Automation_WindowPatternIdentifiers_DEFINED
#define RUNTIMECLASS_Windows_UI_Xaml_Automation_WindowPatternIdentifiers_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Xaml_Automation_WindowPatternIdentifiers[] = L"Windows.UI.Xaml.Automation.WindowPatternIdentifiers";
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
#endif // __windows2Eui2Examl2Eautomation_p_h__

#endif // __windows2Eui2Examl2Eautomation_h__
