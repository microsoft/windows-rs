
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
#ifndef __windows2Egraphics2Eprinting2Eoptiondetails_h__
#define __windows2Egraphics2Eprinting2Eoptiondetails_h__
#ifndef __windows2Egraphics2Eprinting2Eoptiondetails_p_h__
#define __windows2Egraphics2Eprinting2Eoptiondetails_p_h__


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
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintBindingOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintBindingOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintBorderingOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintBorderingOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintCollationOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintCollationOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintColorModeOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintColorModeOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintCopiesOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintCopiesOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintCustomItemDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintCustomItemDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintCustomItemListOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintCustomItemListOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintCustomItemListOptionDetails2;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2 ABI::Windows::Graphics::Printing::OptionDetails::IPrintCustomItemListOptionDetails2

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintCustomItemListOptionDetails3;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3 ABI::Windows::Graphics::Printing::OptionDetails::IPrintCustomItemListOptionDetails3

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintCustomOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintCustomOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintCustomTextOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintCustomTextOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintCustomTextOptionDetails2;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2 ABI::Windows::Graphics::Printing::OptionDetails::IPrintCustomTextOptionDetails2

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintCustomToggleOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintCustomToggleOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintDuplexOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintDuplexOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintHolePunchOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintHolePunchOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintItemListOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintItemListOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintMediaSizeOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintMediaSizeOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintMediaTypeOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintMediaTypeOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintNumberOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintNumberOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintOrientationOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintOrientationOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintPageRangeOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintPageRangeOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintQualityOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintQualityOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintStapleOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintStapleOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintTaskOptionChangedEventArgs;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs ABI::Windows::Graphics::Printing::OptionDetails::IPrintTaskOptionChangedEventArgs

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintTaskOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintTaskOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintTaskOptionDetails2;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2 ABI::Windows::Graphics::Printing::OptionDetails::IPrintTaskOptionDetails2

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintTaskOptionDetailsStatic;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic ABI::Windows::Graphics::Printing::OptionDetails::IPrintTaskOptionDetailsStatic

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    interface IPrintTextOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails ABI::Windows::Graphics::Printing::OptionDetails::IPrintTextOptionDetails

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIIterator_1_IInspectable_USE
#define DEF___FIIterator_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("44a94f2d-04f8-5091-b336-be7892dd10be"))
IIterator<IInspectable*> : IIterator_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<IInspectable*> __FIIterator_1_IInspectable_t;
#define __FIIterator_1_IInspectable ABI::Windows::Foundation::Collections::__FIIterator_1_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_IInspectable_USE */



#ifndef DEF___FIIterable_1_IInspectable_USE
#define DEF___FIIterable_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("092b849b-60b1-52be-a44a-6fe8e933cbe4"))
IIterable<IInspectable*> : IIterable_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<IInspectable*> __FIIterable_1_IInspectable_t;
#define __FIIterable_1_IInspectable ABI::Windows::Foundation::Collections::__FIIterable_1_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_IInspectable_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f5d9c723-a4b1-5fc8-9f78-0b95b716720b"))
IKeyValuePair<HSTRING, ABI::Windows::Graphics::Printing::OptionDetails::IPrintOptionDetails*> : IKeyValuePair_impl<HSTRING, ABI::Windows::Graphics::Printing::OptionDetails::IPrintOptionDetails*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, ABI::Windows::Graphics::Printing::OptionDetails::IPrintOptionDetails*> __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7bef6011-58a1-5523-8e2a-309f8cb1bd39"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6770cf39-094f-59c5-8a5d-e3b5dc64db0f"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_USE
#define DEF___FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("79e5168f-702a-5030-bd44-ef23d8887aed"))
IMapView<HSTRING, ABI::Windows::Graphics::Printing::OptionDetails::IPrintOptionDetails*> : IMapView_impl<HSTRING, ABI::Windows::Graphics::Printing::OptionDetails::IPrintOptionDetails*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, ABI::Windows::Graphics::Printing::OptionDetails::IPrintOptionDetails*> __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_t;
#define __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIVectorView_1_IInspectable_USE
#define DEF___FIVectorView_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a6487363-b074-5c60-ab16-866dce4ee54d"))
IVectorView<IInspectable*> : IVectorView_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<IInspectable*> __FIVectorView_1_IInspectable_t;
#define __FIVectorView_1_IInspectable ABI::Windows::Foundation::Collections::__FIVectorView_1_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_IInspectable_USE */


namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    class PrintTaskOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5c88455c-5b59-557c-8064-5e4f3d59a8ec"))
ITypedEventHandler<ABI::Windows::Graphics::Printing::OptionDetails::PrintTaskOptionDetails*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::OptionDetails::PrintTaskOptionDetails*, ABI::Windows::Graphics::Printing::OptionDetails::IPrintTaskOptionDetails*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Printing::OptionDetails::PrintTaskOptionDetails*, IInspectable*> __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    class PrintTaskOptionChangedEventArgs;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1b1f456b-8821-592e-b4a7-9b4c3712518e"))
ITypedEventHandler<ABI::Windows::Graphics::Printing::OptionDetails::PrintTaskOptionDetails*, ABI::Windows::Graphics::Printing::OptionDetails::PrintTaskOptionChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::OptionDetails::PrintTaskOptionDetails*, ABI::Windows::Graphics::Printing::OptionDetails::IPrintTaskOptionDetails*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Graphics::Printing::OptionDetails::PrintTaskOptionChangedEventArgs*, ABI::Windows::Graphics::Printing::OptionDetails::IPrintTaskOptionChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails, Windows.Graphics.Printing.OptionDetails.PrintTaskOptionChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Graphics::Printing::OptionDetails::PrintTaskOptionDetails*, ABI::Windows::Graphics::Printing::OptionDetails::PrintTaskOptionChangedEventArgs*> __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                interface IPrintTaskOptionsCoreUIConfiguration;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration ABI::Windows::Graphics::Printing::IPrintTaskOptionsCoreUIConfiguration

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                class PrintTaskOptions;
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamWithContentType;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    typedef enum PrintOptionStates : unsigned int PrintOptionStates;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    typedef enum PrintOptionType : int PrintOptionType;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    class PrintCustomItemListOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    class PrintCustomTextOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    class PrintCustomToggleOptionDetails;
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Graphics.Printing.OptionDetails.PrintOptionStates
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    enum PrintOptionStates : unsigned int
                    {
                        PrintOptionStates_None = 0,
                        PrintOptionStates_Enabled = 0x1,
                        PrintOptionStates_Constrained = 0x2,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(PrintOptionStates)
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.OptionDetails.PrintOptionType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    enum PrintOptionType : int
                    {
                        PrintOptionType_Unknown = 0,
                        PrintOptionType_Number = 1,
                        PrintOptionType_Text = 2,
                        PrintOptionType_ItemList = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        PrintOptionType_Toggle = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    };
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintBindingOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintBindingOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintBindingOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintBindingOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("c3f4cc98-9564-4f16-a055-a98b9a49e9d3")
                    IPrintBindingOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_WarningText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WarningText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintBindingOptionDetails = __uuidof(IPrintBindingOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintBorderingOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintBorderingOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintBorderingOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintBorderingOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("4d73bc8f-fb53-4eb2-985f-1d91de0b7639")
                    IPrintBorderingOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_WarningText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WarningText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintBorderingOptionDetails = __uuidof(IPrintBorderingOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCollationOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCollationOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCollationOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCollationOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("d6abb166-a5a6-40dc-acc3-739f28f1e5d3")
                    IPrintCollationOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_WarningText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WarningText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintCollationOptionDetails = __uuidof(IPrintCollationOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintColorModeOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintColorModeOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintColorModeOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintColorModeOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("dba97704-f1d6-4843-a484-9b447cdcf3b6")
                    IPrintColorModeOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_WarningText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WarningText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintColorModeOptionDetails = __uuidof(IPrintColorModeOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCopiesOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCopiesOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCopiesOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCopiesOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("42053099-4339-4343-898d-2c47b5e0c341")
                    IPrintCopiesOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_WarningText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WarningText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintCopiesOptionDetails = __uuidof(IPrintCopiesOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCustomItemDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCustomItemDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCustomItemDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCustomItemDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("5704b637-5c3a-449a-aa36-b3291b1192fd")
                    IPrintCustomItemDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ItemId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ItemDisplayName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ItemDisplayName(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintCustomItemDetails = __uuidof(IPrintCustomItemDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCustomItemListOptionDetails
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *     Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails
 *     Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCustomItemListOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("a5fafd88-58f2-4ebd-b90f-51e4f2944c5d")
                    IPrintCustomItemListOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE AddItem(
                            HSTRING itemId,
                            HSTRING displayName
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintCustomItemListOptionDetails = __uuidof(IPrintCustomItemListOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCustomItemListOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCustomItemListOptionDetails2[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("c9d6353d-651c-4a39-906e-1091a1801bf1")
                    IPrintCustomItemListOptionDetails2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE AddItem(
                            HSTRING itemId,
                            HSTRING displayName,
                            HSTRING description,
                            ABI::Windows::Storage::Streams::IRandomAccessStreamWithContentType* icon
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintCustomItemListOptionDetails2 = __uuidof(IPrintCustomItemListOptionDetails2);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCustomItemListOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCustomItemListOptionDetails3[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails3";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("4fa1b53f-3c34-4868-a407-fc5eab259b21")
                    IPrintCustomItemListOptionDetails3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_WarningText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WarningText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintCustomItemListOptionDetails3 = __uuidof(IPrintCustomItemListOptionDetails3);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCustomOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("e32bde1c-28af-4b90-95da-a3acf320b929")
                    IPrintCustomOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintCustomOptionDetails = __uuidof(IPrintCustomOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCustomTextOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCustomTextOptionDetails
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails
 *     Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCustomTextOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCustomTextOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("2ad171f8-c8bd-4905-9192-0d75136e8b31")
                    IPrintCustomTextOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_MaxCharacters(
                            UINT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MaxCharacters(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintCustomTextOptionDetails = __uuidof(IPrintCustomTextOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCustomTextOptionDetails2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCustomTextOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCustomTextOptionDetails2[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCustomTextOptionDetails2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("cea70b54-b977-4718-8338-7ed2b0d86fe3")
                    IPrintCustomTextOptionDetails2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_WarningText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WarningText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintCustomTextOptionDetails2 = __uuidof(IPrintCustomTextOptionDetails2);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCustomToggleOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCustomToggleOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCustomToggleOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCustomToggleOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("9db4d514-e461-4608-8ee9-db6f5ed073c6")
                    IPrintCustomToggleOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_WarningText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WarningText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintCustomToggleOptionDetails = __uuidof(IPrintCustomToggleOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintDuplexOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintDuplexOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintDuplexOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintDuplexOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("fcd94591-d4a4-44fa-b3fe-42e0ba28d5ad")
                    IPrintDuplexOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_WarningText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WarningText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintDuplexOptionDetails = __uuidof(IPrintDuplexOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintHolePunchOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintHolePunchOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintHolePunchOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintHolePunchOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("a6de1f18-482c-4657-9d71-8ddddbea1e1e")
                    IPrintHolePunchOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_WarningText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WarningText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintHolePunchOptionDetails = __uuidof(IPrintHolePunchOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintItemListOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("9a2257bf-fe61-43d8-a24f-a3f6ab7320e7")
                    IPrintItemListOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Items(
                            __FIVectorView_1_IInspectable** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintItemListOptionDetails = __uuidof(IPrintItemListOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintMediaSizeOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintMediaSizeOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintMediaSizeOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintMediaSizeOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("6c8d5bcf-c0bf-47c8-b84a-628e7d0d1a1d")
                    IPrintMediaSizeOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_WarningText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WarningText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintMediaSizeOptionDetails = __uuidof(IPrintMediaSizeOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintMediaTypeOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintMediaTypeOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintMediaTypeOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintMediaTypeOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("f8c7000b-abf3-4abc-8e86-22abc5744a43")
                    IPrintMediaTypeOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_WarningText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WarningText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintMediaTypeOptionDetails = __uuidof(IPrintMediaTypeOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintNumberOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintNumberOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintNumberOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("4d01bbaf-645c-4de9-965f-6fc6bbc47cab")
                    IPrintNumberOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_MinValue(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MaxValue(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintNumberOptionDetails = __uuidof(IPrintNumberOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("390686cf-d682-495f-adfe-d7333f5c1808")
                    IPrintOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_OptionId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OptionType(
                            ABI::Windows::Graphics::Printing::OptionDetails::PrintOptionType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ErrorText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ErrorText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_State(
                            ABI::Windows::Graphics::Printing::OptionDetails::PrintOptionStates value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_State(
                            ABI::Windows::Graphics::Printing::OptionDetails::PrintOptionStates* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Value(
                            IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TrySetValue(
                            IInspectable* value,
                            boolean* succeeded
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintOptionDetails = __uuidof(IPrintOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintOrientationOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintOrientationOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintOrientationOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintOrientationOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("46c38879-66e0-4da0-87b4-d25457824eb7")
                    IPrintOrientationOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_WarningText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WarningText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintOrientationOptionDetails = __uuidof(IPrintOrientationOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintPageRangeOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintPageRangeOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintPageRangeOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintPageRangeOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("5a19e4b7-2be8-4aa7-9ea5-defbe8713b4e")
                    IPrintPageRangeOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_WarningText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WarningText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintPageRangeOptionDetails = __uuidof(IPrintPageRangeOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintQualityOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintQualityOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintQualityOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintQualityOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("2dd06ba1-ce1a-44e6-84f9-3a92ea1e3044")
                    IPrintQualityOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_WarningText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WarningText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintQualityOptionDetails = __uuidof(IPrintQualityOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintStapleOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintStapleOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintStapleOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintStapleOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("d43175bd-9c0b-44e0-84f6-ceebce653800")
                    IPrintStapleOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_WarningText(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_WarningText(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Description(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Description(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintStapleOptionDetails = __uuidof(IPrintStapleOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintTaskOptionChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintTaskOptionChangedEventArgs[] = L"Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("65197d05-a5ee-4307-9407-9acad147679c")
                    IPrintTaskOptionChangedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_OptionId(
                            IInspectable** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintTaskOptionChangedEventArgs = __uuidof(IPrintTaskOptionChangedEventArgs);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintTaskOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("f5720af1-a89e-42a6-81af-f8e010b38a68")
                    IPrintTaskOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Options(
                            __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateItemListOption(
                            HSTRING optionId,
                            HSTRING displayName,
                            ABI::Windows::Graphics::Printing::OptionDetails::IPrintOptionDetails** itemListOption
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateTextOption(
                            HSTRING optionId,
                            HSTRING displayName,
                            ABI::Windows::Graphics::Printing::OptionDetails::IPrintOptionDetails** textOption
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_OptionChanged(
                            __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs* eventHandler,
                            EventRegistrationToken* eventCookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_OptionChanged(
                            EventRegistrationToken eventCookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_BeginValidation(
                            __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable* eventHandler,
                            EventRegistrationToken* eventCookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_BeginValidation(
                            EventRegistrationToken eventCookie
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintTaskOptionDetails = __uuidof(IPrintTaskOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetails2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintTaskOptionDetails2[] = L"Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetails2";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("53730a09-f968-4692-a177-c074597186db")
                    IPrintTaskOptionDetails2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateToggleOption(
                            HSTRING optionId,
                            HSTRING displayName,
                            ABI::Windows::Graphics::Printing::OptionDetails::IPrintOptionDetails** toggleOption
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintTaskOptionDetails2 = __uuidof(IPrintTaskOptionDetails2);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetailsStatic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintTaskOptionDetailsStatic[] = L"Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetailsStatic";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("135da193-0961-4b6e-8766-f13b7fbccd58")
                    IPrintTaskOptionDetailsStatic : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetFromPrintTaskOptions(
                            ABI::Windows::Graphics::Printing::IPrintTaskOptionsCore* printTaskOptions,
                            ABI::Windows::Graphics::Printing::OptionDetails::IPrintTaskOptionDetails** printTaskOptionDetails
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintTaskOptionDetailsStatic = __uuidof(IPrintTaskOptionDetailsStatic);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintTextOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintTextOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintTextOptionDetails";
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Printing {
                namespace OptionDetails {
                    MIDL_INTERFACE("ad75e563-5ce4-46bc-9918-ab9fad144c5b")
                    IPrintTextOptionDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_MaxCharacters(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPrintTextOptionDetails = __uuidof(IPrintTextOptionDetails);
                } /* OptionDetails */
            } /* Printing */
        } /* Graphics */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintBindingOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintBindingOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintBindingOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintBindingOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintBindingOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintBindingOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintBorderingOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintBorderingOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintBorderingOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintBorderingOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintBorderingOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintBorderingOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintCollationOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintCollationOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCollationOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCollationOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintCollationOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintCollationOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintColorModeOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintColorModeOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintColorModeOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintColorModeOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintColorModeOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintColorModeOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintCopiesOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintNumberOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintCopiesOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCopiesOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCopiesOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintCopiesOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintCopiesOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintCustomItemDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomItemDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCustomItemDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCustomItemDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintCustomItemDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintCustomItemDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintCustomItemListOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails2
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCustomItemListOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCustomItemListOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintCustomItemListOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintCustomItemListOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintCustomTextOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomTextOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomTextOptionDetails2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCustomTextOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCustomTextOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintCustomTextOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintCustomTextOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintCustomToggleOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomToggleOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCustomToggleOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCustomToggleOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintCustomToggleOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintCustomToggleOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintDuplexOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintDuplexOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintDuplexOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintDuplexOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintDuplexOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintDuplexOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintHolePunchOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintHolePunchOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintHolePunchOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintHolePunchOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintHolePunchOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintHolePunchOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintMediaSizeOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintMediaSizeOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintMediaSizeOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintMediaSizeOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintMediaSizeOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintMediaSizeOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintMediaTypeOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintMediaTypeOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintMediaTypeOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintMediaTypeOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintMediaTypeOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintMediaTypeOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintOrientationOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintOrientationOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintOrientationOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintOrientationOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintOrientationOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintOrientationOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintPageRangeOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintPageRangeOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintPageRangeOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintPageRangeOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintPageRangeOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintPageRangeOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintQualityOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintQualityOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintQualityOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintQualityOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintQualityOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintQualityOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintStapleOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintStapleOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintStapleOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintStapleOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintStapleOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintStapleOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintTaskOptionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintTaskOptionChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintTaskOptionChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintTaskOptionChangedEventArgs[] = L"Windows.Graphics.Printing.OptionDetails.PrintTaskOptionChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetailsStatic interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.IPrintTaskOptionsCore
 *    Windows.Graphics.Printing.IPrintTaskOptionsCoreUIConfiguration
 *    Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetails2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintTaskOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintTaskOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintTaskOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2 __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3 __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2 __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2 __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if !defined(____FIIterator_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterator_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterator_1_IInspectable __FIIterator_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_IInspectable;

typedef struct __FIIterator_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_IInspectable* This,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_IInspectable* This,
        UINT32 itemsLength,
        IInspectable** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_IInspectableVtbl;

interface __FIIterator_1_IInspectable
{
    CONST_VTBL struct __FIIterator_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_IInspectable_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_IInspectable_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_IInspectable_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_IInspectable_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterable_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterable_1_IInspectable __FIIterable_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_IInspectable;

typedef struct __FIIterable_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_IInspectable* This,
        __FIIterator_1_IInspectable** result);

    END_INTERFACE
} __FIIterable_1_IInspectableVtbl;

interface __FIIterable_1_IInspectable
{
    CONST_VTBL struct __FIIterable_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_IInspectable_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_IInspectable_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetailsVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetailsVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetailsVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails;

typedef struct __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        HSTRING key,
        __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails* This,
        __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails** first,
        __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetailsVtbl;

interface __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIVectorView_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIVectorView_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_IInspectable __FIVectorView_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_IInspectable;

typedef struct __FIVectorView_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_IInspectable* This,
        UINT32 index,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_IInspectable* This,
        IInspectable* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_IInspectable* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        IInspectable** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_IInspectableVtbl;

interface __FIVectorView_1_IInspectable
{
    CONST_VTBL struct __FIVectorView_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_IInspectable_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_IInspectable_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_IInspectable_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_IInspectable_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable* This,
        __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs* This,
        __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails* sender,
        __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration;

#endif // ____x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCoreUIConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CPrintOptionStates __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CPrintOptionStates;

typedef enum __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CPrintOptionType __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CPrintOptionType;

/*
 *
 * Struct Windows.Graphics.Printing.OptionDetails.PrintOptionStates
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CPrintOptionStates
{
    PrintOptionStates_None = 0,
    PrintOptionStates_Enabled = 0x1,
    PrintOptionStates_Constrained = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Graphics.Printing.OptionDetails.PrintOptionType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CPrintOptionType
{
    PrintOptionType_Unknown = 0,
    PrintOptionType_Number = 1,
    PrintOptionType_Text = 2,
    PrintOptionType_ItemList = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    PrintOptionType_Toggle = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintBindingOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintBindingOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintBindingOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintBindingOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_put_WarningText(This, value) \
    ((This)->lpVtbl->put_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_get_WarningText(This, value) \
    ((This)->lpVtbl->get_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBindingOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintBorderingOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintBorderingOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintBorderingOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintBorderingOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_put_WarningText(This, value) \
    ((This)->lpVtbl->put_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_get_WarningText(This, value) \
    ((This)->lpVtbl->get_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintBorderingOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCollationOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCollationOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCollationOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCollationOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_put_WarningText(This, value) \
    ((This)->lpVtbl->put_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_get_WarningText(This, value) \
    ((This)->lpVtbl->get_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCollationOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintColorModeOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintColorModeOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintColorModeOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintColorModeOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_put_WarningText(This, value) \
    ((This)->lpVtbl->put_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_get_WarningText(This, value) \
    ((This)->lpVtbl->get_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintColorModeOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCopiesOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCopiesOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCopiesOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCopiesOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_put_WarningText(This, value) \
    ((This)->lpVtbl->put_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_get_WarningText(This, value) \
    ((This)->lpVtbl->get_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCopiesOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCustomItemDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCustomItemDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCustomItemDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCustomItemDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ItemId)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ItemDisplayName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ItemDisplayName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_get_ItemId(This, value) \
    ((This)->lpVtbl->get_ItemId(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_put_ItemDisplayName(This, value) \
    ((This)->lpVtbl->put_ItemDisplayName(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_get_ItemDisplayName(This, value) \
    ((This)->lpVtbl->get_ItemDisplayName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCustomItemListOptionDetails
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *     Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails
 *     Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCustomItemListOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddItem)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails* This,
        HSTRING itemId,
        HSTRING displayName);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_AddItem(This, itemId, displayName) \
    ((This)->lpVtbl->AddItem(This, itemId, displayName))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCustomItemListOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCustomItemListOptionDetails2[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails2";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddItem)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2* This,
        HSTRING itemId,
        HSTRING displayName,
        HSTRING description,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamWithContentType* icon);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_AddItem(This, itemId, displayName, description, icon) \
    ((This)->lpVtbl->AddItem(This, itemId, displayName, description, icon))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCustomItemListOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCustomItemListOptionDetails3[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails3";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_put_WarningText(This, value) \
    ((This)->lpVtbl->put_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_get_WarningText(This, value) \
    ((This)->lpVtbl->get_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomItemListOptionDetails3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCustomOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCustomTextOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCustomTextOptionDetails
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails
 *     Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCustomTextOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCustomTextOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_MaxCharacters)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_MaxCharacters)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_put_MaxCharacters(This, value) \
    ((This)->lpVtbl->put_MaxCharacters(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_get_MaxCharacters(This, value) \
    ((This)->lpVtbl->get_MaxCharacters(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCustomTextOptionDetails2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCustomTextOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCustomTextOptionDetails2[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCustomTextOptionDetails2";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_put_WarningText(This, value) \
    ((This)->lpVtbl->put_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_get_WarningText(This, value) \
    ((This)->lpVtbl->get_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomTextOptionDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintCustomToggleOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintCustomToggleOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintCustomToggleOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintCustomToggleOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_put_WarningText(This, value) \
    ((This)->lpVtbl->put_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_get_WarningText(This, value) \
    ((This)->lpVtbl->get_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintCustomToggleOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintDuplexOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintDuplexOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintDuplexOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintDuplexOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_put_WarningText(This, value) \
    ((This)->lpVtbl->put_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_get_WarningText(This, value) \
    ((This)->lpVtbl->get_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintDuplexOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintHolePunchOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintHolePunchOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintHolePunchOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintHolePunchOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_put_WarningText(This, value) \
    ((This)->lpVtbl->put_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_get_WarningText(This, value) \
    ((This)->lpVtbl->get_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintHolePunchOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintItemListOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Items)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails* This,
        __FIVectorView_1_IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_get_Items(This, value) \
    ((This)->lpVtbl->get_Items(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintItemListOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintMediaSizeOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintMediaSizeOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintMediaSizeOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintMediaSizeOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_put_WarningText(This, value) \
    ((This)->lpVtbl->put_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_get_WarningText(This, value) \
    ((This)->lpVtbl->get_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaSizeOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintMediaTypeOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintMediaTypeOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintMediaTypeOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintMediaTypeOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_put_WarningText(This, value) \
    ((This)->lpVtbl->put_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_get_WarningText(This, value) \
    ((This)->lpVtbl->get_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintMediaTypeOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintNumberOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintNumberOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintNumberOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MinValue)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxValue)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_get_MinValue(This, value) \
    ((This)->lpVtbl->get_MinValue(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_get_MaxValue(This, value) \
    ((This)->lpVtbl->get_MaxValue(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintNumberOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OptionId)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_OptionType)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CPrintOptionType* value);
    HRESULT (STDMETHODCALLTYPE* put_ErrorText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ErrorText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_State)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CPrintOptionStates value);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails* This,
        enum __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CPrintOptionStates* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails* This,
        IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* TrySetValue)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails* This,
        IInspectable* value,
        boolean* succeeded);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_get_OptionId(This, value) \
    ((This)->lpVtbl->get_OptionId(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_get_OptionType(This, value) \
    ((This)->lpVtbl->get_OptionType(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_put_ErrorText(This, value) \
    ((This)->lpVtbl->put_ErrorText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_get_ErrorText(This, value) \
    ((This)->lpVtbl->get_ErrorText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_put_State(This, value) \
    ((This)->lpVtbl->put_State(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_TrySetValue(This, value, succeeded) \
    ((This)->lpVtbl->TrySetValue(This, value, succeeded))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintOrientationOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintOrientationOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintOrientationOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintOrientationOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_put_WarningText(This, value) \
    ((This)->lpVtbl->put_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_get_WarningText(This, value) \
    ((This)->lpVtbl->get_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOrientationOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintPageRangeOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintPageRangeOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintPageRangeOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintPageRangeOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_put_WarningText(This, value) \
    ((This)->lpVtbl->put_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_get_WarningText(This, value) \
    ((This)->lpVtbl->get_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintPageRangeOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintQualityOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintQualityOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintQualityOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintQualityOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_put_WarningText(This, value) \
    ((This)->lpVtbl->put_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_get_WarningText(This, value) \
    ((This)->lpVtbl->get_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintQualityOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintStapleOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintStapleOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintStapleOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintStapleOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_WarningText)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_put_WarningText(This, value) \
    ((This)->lpVtbl->put_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_get_WarningText(This, value) \
    ((This)->lpVtbl->get_WarningText(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintStapleOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintTaskOptionChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintTaskOptionChangedEventArgs[] = L"Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionChangedEventArgs";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OptionId)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs* This,
        IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_get_OptionId(This, value) \
    ((This)->lpVtbl->get_OptionId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintTaskOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Options)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails* This,
        __FIMapView_2_HSTRING_Windows__CGraphics__CPrinting__COptionDetails__CIPrintOptionDetails** value);
    HRESULT (STDMETHODCALLTYPE* CreateItemListOption)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails* This,
        HSTRING optionId,
        HSTRING displayName,
        __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails** itemListOption);
    HRESULT (STDMETHODCALLTYPE* CreateTextOption)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails* This,
        HSTRING optionId,
        HSTRING displayName,
        __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails** textOption);
    HRESULT (STDMETHODCALLTYPE* add_OptionChanged)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails* This,
        __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionChangedEventArgs* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_OptionChanged)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails* This,
        EventRegistrationToken eventCookie);
    HRESULT (STDMETHODCALLTYPE* add_BeginValidation)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails* This,
        __FITypedEventHandler_2_Windows__CGraphics__CPrinting__COptionDetails__CPrintTaskOptionDetails_IInspectable* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_BeginValidation)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_get_Options(This, value) \
    ((This)->lpVtbl->get_Options(This, value))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_CreateItemListOption(This, optionId, displayName, itemListOption) \
    ((This)->lpVtbl->CreateItemListOption(This, optionId, displayName, itemListOption))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_CreateTextOption(This, optionId, displayName, textOption) \
    ((This)->lpVtbl->CreateTextOption(This, optionId, displayName, textOption))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_add_OptionChanged(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_OptionChanged(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_remove_OptionChanged(This, eventCookie) \
    ((This)->lpVtbl->remove_OptionChanged(This, eventCookie))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_add_BeginValidation(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_BeginValidation(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_remove_BeginValidation(This, eventCookie) \
    ((This)->lpVtbl->remove_BeginValidation(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetails2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintTaskOptionDetails2[] = L"Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetails2";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateToggleOption)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2* This,
        HSTRING optionId,
        HSTRING displayName,
        __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintOptionDetails** toggleOption);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2Vtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_CreateToggleOption(This, optionId, displayName, toggleOption) \
    ((This)->lpVtbl->CreateToggleOption(This, optionId, displayName, toggleOption))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetailsStatic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintTaskOptionDetailsStatic[] = L"Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetailsStatic";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStaticVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetFromPrintTaskOptions)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic* This,
        __x_ABI_CWindows_CGraphics_CPrinting_CIPrintTaskOptionsCore* printTaskOptions,
        __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetails** printTaskOptionDetails);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStaticVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStaticVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_GetFromPrintTaskOptions(This, printTaskOptions, printTaskOptionDetails) \
    ((This)->lpVtbl->GetFromPrintTaskOptions(This, printTaskOptions, printTaskOptionDetails))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTaskOptionDetailsStatic_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Graphics.Printing.OptionDetails.IPrintTextOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Graphics_Printing_OptionDetails_IPrintTextOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.IPrintTextOptionDetails";
typedef struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxCharacters)(__x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetailsVtbl;

interface __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_get_MaxCharacters(This, value) \
    ((This)->lpVtbl->get_MaxCharacters(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails;
#endif /* !defined(____x_ABI_CWindows_CGraphics_CPrinting_COptionDetails_CIPrintTextOptionDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintBindingOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintBindingOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintBindingOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintBindingOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintBindingOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintBindingOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintBorderingOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintBorderingOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintBorderingOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintBorderingOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintBorderingOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintBorderingOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintCollationOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintCollationOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCollationOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCollationOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintCollationOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintCollationOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintColorModeOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintColorModeOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintColorModeOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintColorModeOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintColorModeOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintColorModeOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintCopiesOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintNumberOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintCopiesOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCopiesOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCopiesOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintCopiesOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintCopiesOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintCustomItemDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomItemDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCustomItemDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCustomItemDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintCustomItemDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintCustomItemDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintCustomItemListOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails2
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomItemListOptionDetails3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCustomItemListOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCustomItemListOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintCustomItemListOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintCustomItemListOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintCustomTextOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomTextOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomTextOptionDetails2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCustomTextOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCustomTextOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintCustomTextOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintCustomTextOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintCustomToggleOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintCustomToggleOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCustomToggleOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintCustomToggleOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintCustomToggleOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintCustomToggleOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintDuplexOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintDuplexOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintDuplexOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintDuplexOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintDuplexOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintDuplexOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintHolePunchOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintHolePunchOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintHolePunchOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintHolePunchOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintHolePunchOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintHolePunchOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintMediaSizeOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintMediaSizeOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintMediaSizeOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintMediaSizeOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintMediaSizeOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintMediaSizeOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintMediaTypeOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintMediaTypeOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintMediaTypeOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintMediaTypeOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintMediaTypeOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintMediaTypeOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintOrientationOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintOrientationOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintOrientationOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintOrientationOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintOrientationOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintOrientationOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintPageRangeOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintPageRangeOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintPageRangeOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintPageRangeOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintPageRangeOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintPageRangeOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintQualityOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintQualityOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintQualityOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintQualityOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintQualityOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintQualityOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintStapleOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.OptionDetails.IPrintItemListOptionDetails
 *    Windows.Graphics.Printing.OptionDetails.IPrintStapleOptionDetails
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintStapleOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintStapleOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintStapleOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintStapleOptionDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintTaskOptionChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintTaskOptionChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintTaskOptionChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintTaskOptionChangedEventArgs[] = L"Windows.Graphics.Printing.OptionDetails.PrintTaskOptionChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetailsStatic interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetails ** Default Interface **
 *    Windows.Graphics.Printing.IPrintTaskOptionsCore
 *    Windows.Graphics.Printing.IPrintTaskOptionsCoreUIConfiguration
 *    Windows.Graphics.Printing.OptionDetails.IPrintTaskOptionDetails2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintTaskOptionDetails_DEFINED
#define RUNTIMECLASS_Windows_Graphics_Printing_OptionDetails_PrintTaskOptionDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Graphics_Printing_OptionDetails_PrintTaskOptionDetails[] = L"Windows.Graphics.Printing.OptionDetails.PrintTaskOptionDetails";
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
#endif // __windows2Egraphics2Eprinting2Eoptiondetails_p_h__

#endif // __windows2Egraphics2Eprinting2Eoptiondetails_h__
