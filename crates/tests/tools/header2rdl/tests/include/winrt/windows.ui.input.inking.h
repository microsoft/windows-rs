
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
#ifndef __windows2Eui2Einput2Einking_h__
#define __windows2Eui2Einput2Einking_h__
#ifndef __windows2Eui2Einput2Einking_p_h__
#define __windows2Eui2Einput2Einking_p_h__


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
#include "Windows.Foundation.Numerics.h"
#include "Windows.Storage.Streams.h"
#include "Windows.UI.h"
#include "Windows.UI.Core.h"
#include "Windows.UI.Input.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkDrawingAttributes;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes ABI::Windows::UI::Input::Inking::IInkDrawingAttributes

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkDrawingAttributes2;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2 ABI::Windows::UI::Input::Inking::IInkDrawingAttributes2

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkDrawingAttributes3;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3 ABI::Windows::UI::Input::Inking::IInkDrawingAttributes3

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkDrawingAttributes4;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4 ABI::Windows::UI::Input::Inking::IInkDrawingAttributes4

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkDrawingAttributes5;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5 ABI::Windows::UI::Input::Inking::IInkDrawingAttributes5

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkDrawingAttributesPencilProperties;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties ABI::Windows::UI::Input::Inking::IInkDrawingAttributesPencilProperties

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkDrawingAttributesStatics;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics ABI::Windows::UI::Input::Inking::IInkDrawingAttributesStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkInputConfiguration;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration ABI::Windows::UI::Input::Inking::IInkInputConfiguration

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkInputConfiguration2;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2 ABI::Windows::UI::Input::Inking::IInkInputConfiguration2

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkInputProcessingConfiguration;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration ABI::Windows::UI::Input::Inking::IInkInputProcessingConfiguration

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkManager;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager ABI::Windows::UI::Input::Inking::IInkManager

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkModelerAttributes;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes ABI::Windows::UI::Input::Inking::IInkModelerAttributes

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkModelerAttributes2;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2 ABI::Windows::UI::Input::Inking::IInkModelerAttributes2

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkPoint;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint ABI::Windows::UI::Input::Inking::IInkPoint

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkPoint2;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2 ABI::Windows::UI::Input::Inking::IInkPoint2

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkPointFactory;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory ABI::Windows::UI::Input::Inking::IInkPointFactory

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkPointFactory2;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2 ABI::Windows::UI::Input::Inking::IInkPointFactory2

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkPresenter;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter ABI::Windows::UI::Input::Inking::IInkPresenter

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkPresenter2;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2 ABI::Windows::UI::Input::Inking::IInkPresenter2

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkPresenter3;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3 ABI::Windows::UI::Input::Inking::IInkPresenter3

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkPresenterProtractor;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor ABI::Windows::UI::Input::Inking::IInkPresenterProtractor

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkPresenterProtractorFactory;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory ABI::Windows::UI::Input::Inking::IInkPresenterProtractorFactory

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkPresenterRuler;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler ABI::Windows::UI::Input::Inking::IInkPresenterRuler

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkPresenterRuler2;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2 ABI::Windows::UI::Input::Inking::IInkPresenterRuler2

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkPresenterRulerFactory;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory ABI::Windows::UI::Input::Inking::IInkPresenterRulerFactory

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkPresenterStencil;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil ABI::Windows::UI::Input::Inking::IInkPresenterStencil

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkRecognitionResult;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult ABI::Windows::UI::Input::Inking::IInkRecognitionResult

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkRecognizer;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer ABI::Windows::UI::Input::Inking::IInkRecognizer

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkRecognizerContainer;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer ABI::Windows::UI::Input::Inking::IInkRecognizerContainer

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkStroke;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke ABI::Windows::UI::Input::Inking::IInkStroke

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkStroke2;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2 ABI::Windows::UI::Input::Inking::IInkStroke2

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkStroke3;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3 ABI::Windows::UI::Input::Inking::IInkStroke3

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkStroke4;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4 ABI::Windows::UI::Input::Inking::IInkStroke4

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkStrokeBuilder;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder ABI::Windows::UI::Input::Inking::IInkStrokeBuilder

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkStrokeBuilder2;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2 ABI::Windows::UI::Input::Inking::IInkStrokeBuilder2

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkStrokeBuilder3;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3 ABI::Windows::UI::Input::Inking::IInkStrokeBuilder3

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkStrokeContainer;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer ABI::Windows::UI::Input::Inking::IInkStrokeContainer

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkStrokeContainer2;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2 ABI::Windows::UI::Input::Inking::IInkStrokeContainer2

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkStrokeContainer3;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3 ABI::Windows::UI::Input::Inking::IInkStrokeContainer3

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkStrokeInput;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput ABI::Windows::UI::Input::Inking::IInkStrokeInput

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkStrokeRenderingSegment;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment ABI::Windows::UI::Input::Inking::IInkStrokeRenderingSegment

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkStrokesCollectedEventArgs;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs ABI::Windows::UI::Input::Inking::IInkStrokesCollectedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkStrokesErasedEventArgs;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs ABI::Windows::UI::Input::Inking::IInkStrokesErasedEventArgs

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkSynchronizer;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer ABI::Windows::UI::Input::Inking::IInkSynchronizer

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IInkUnprocessedInput;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput ABI::Windows::UI::Input::Inking::IInkUnprocessedInput

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IPenAndInkSettings;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings ABI::Windows::UI::Input::Inking::IPenAndInkSettings

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IPenAndInkSettings2;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2 ABI::Windows::UI::Input::Inking::IPenAndInkSettings2

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    interface IPenAndInkSettingsStatics;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics ABI::Windows::UI::Input::Inking::IPenAndInkSettingsStatics

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncActionWithProgressCompletedHandler_1_UINT64_USE
#define DEF___FIAsyncActionWithProgressCompletedHandler_1_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e6ff857b-f160-571a-a934-2c61f98c862d"))
IAsyncActionWithProgressCompletedHandler<UINT64> : IAsyncActionWithProgressCompletedHandler_impl<UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncActionWithProgressCompletedHandler`1<UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncActionWithProgressCompletedHandler<UINT64> __FIAsyncActionWithProgressCompletedHandler_1_UINT64_t;
#define __FIAsyncActionWithProgressCompletedHandler_1_UINT64 ABI::Windows::Foundation::__FIAsyncActionWithProgressCompletedHandler_1_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncActionWithProgressCompletedHandler_1_UINT64_USE */



#ifndef DEF___FIAsyncActionWithProgress_1_UINT64_USE
#define DEF___FIAsyncActionWithProgress_1_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("43f713d0-c49d-5e55-aebf-af395768351e"))
IAsyncActionWithProgress<UINT64> : IAsyncActionWithProgress_impl<UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncActionWithProgress`1<UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncActionWithProgress<UINT64> __FIAsyncActionWithProgress_1_UINT64_t;
#define __FIAsyncActionWithProgress_1_UINT64 ABI::Windows::Foundation::__FIAsyncActionWithProgress_1_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncActionWithProgress_1_UINT64_USE */



#ifndef DEF___FIAsyncActionProgressHandler_1_UINT64_USE
#define DEF___FIAsyncActionProgressHandler_1_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("55e233ca-f243-5ae2-853b-f9cc7c0ae0cf"))
IAsyncActionProgressHandler<UINT64> : IAsyncActionProgressHandler_impl<UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncActionProgressHandler`1<UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncActionProgressHandler<UINT64> __FIAsyncActionProgressHandler_1_UINT64_t;
#define __FIAsyncActionProgressHandler_1_UINT64 ABI::Windows::Foundation::__FIAsyncActionProgressHandler_1_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncActionProgressHandler_1_UINT64_USE */


namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkRecognitionResult;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_USE
#define DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9abc247f-0223-5f44-8fa1-0d6d691bf9af"))
IIterator<ABI::Windows::UI::Input::Inking::InkRecognitionResult*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkRecognitionResult*, ABI::Windows::UI::Input::Inking::IInkRecognitionResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Input.Inking.InkRecognitionResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Input::Inking::InkRecognitionResult*> __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_t;
#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_USE
#define DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e29b658b-7cc1-561c-9912-001dbca86651"))
IIterable<ABI::Windows::UI::Input::Inking::InkRecognitionResult*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkRecognitionResult*, ABI::Windows::UI::Input::Inking::IInkRecognitionResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Input.Inking.InkRecognitionResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Input::Inking::InkRecognitionResult*> __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_t;
#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_USE
#define DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ecfc4f0b-112c-5cd3-acf0-d746d6bdfeb5"))
IVectorView<ABI::Windows::UI::Input::Inking::InkRecognitionResult*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkRecognitionResult*, ABI::Windows::UI::Input::Inking::IInkRecognitionResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Input.Inking.InkRecognitionResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Input::Inking::InkRecognitionResult*> __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_t;
#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b1923f59-d674-5365-b99a-3f1e52268c7f"))
IAsyncOperation<__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.UI.Input.Inking.InkRecognitionResult>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult*> __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ece8567f-8080-5ced-8988-bb0364c803d4"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.UI.Input.Inking.InkRecognitionResult>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1e466dc5-840f-54f9-b877-5e3a9f4b6c74"))
IAsyncOperationWithProgressCompletedHandler<UINT32, UINT32> : IAsyncOperationWithProgressCompletedHandler_impl<UINT32, UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<UInt32, UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<UINT32, UINT32> __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32 ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_USE */



#ifndef DEF___FIAsyncOperationWithProgress_2_UINT32_UINT32_USE
#define DEF___FIAsyncOperationWithProgress_2_UINT32_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("eccb574a-c684-5572-a679-6b0842cfb57f"))
IAsyncOperationWithProgress<UINT32, UINT32> : IAsyncOperationWithProgress_impl<UINT32, UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<UInt32, UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<UINT32, UINT32> __FIAsyncOperationWithProgress_2_UINT32_UINT32_t;
#define __FIAsyncOperationWithProgress_2_UINT32_UINT32 ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_UINT32_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_UINT32_UINT32_USE */



#ifndef DEF___FIAsyncOperationProgressHandler_2_UINT32_UINT32_USE
#define DEF___FIAsyncOperationProgressHandler_2_UINT32_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ea0fe405-d432-5ac7-9ef8-5a65e1f97d7e"))
IAsyncOperationProgressHandler<UINT32, UINT32> : IAsyncOperationProgressHandler_impl<UINT32, UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<UInt32, UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<UINT32, UINT32> __FIAsyncOperationProgressHandler_2_UINT32_UINT32_t;
#define __FIAsyncOperationProgressHandler_2_UINT32_UINT32 ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_UINT32_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_UINT32_UINT32_USE */



#ifndef DEF___FIIterator_1_HSTRING_USE
#define DEF___FIIterator_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8c304ebb-6615-50a4-8829-879ecd443236"))
IIterator<HSTRING> : IIterator_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<HSTRING> __FIIterator_1_HSTRING_t;
#define __FIIterator_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_HSTRING_USE */



#ifndef DEF___FIIterable_1_HSTRING_USE
#define DEF___FIIterable_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e2fcc7c1-3bfc-5a0b-b2b0-72e769d1cb7e"))
IIterable<HSTRING> : IIterable_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<HSTRING> __FIIterable_1_HSTRING_t;
#define __FIIterable_1_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct Point Point;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CFoundation__CPoint_USE
#define DEF___FIIterator_1_Windows__CFoundation__CPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c602b59e-0a8e-5e99-b478-2b564585278d"))
IIterator<struct ABI::Windows::Foundation::Point> : IIterator_impl<struct ABI::Windows::Foundation::Point>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Point>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<struct ABI::Windows::Foundation::Point> __FIIterator_1_Windows__CFoundation__CPoint_t;
#define __FIIterator_1_Windows__CFoundation__CPoint ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CFoundation__CPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CFoundation__CPoint_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CFoundation__CPoint_USE
#define DEF___FIIterable_1_Windows__CFoundation__CPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c192280d-3a09-5423-9dc5-67b83ebde41d"))
IIterable<struct ABI::Windows::Foundation::Point> : IIterable_impl<struct ABI::Windows::Foundation::Point>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Point>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<struct ABI::Windows::Foundation::Point> __FIIterable_1_Windows__CFoundation__CPoint_t;
#define __FIIterable_1_Windows__CFoundation__CPoint ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CFoundation__CPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CFoundation__CPoint_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkPoint;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_USE
#define DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("47415452-db79-567e-84d5-e9912330f944"))
IIterator<ABI::Windows::UI::Input::Inking::InkPoint*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkPoint*, ABI::Windows::UI::Input::Inking::IInkPoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Input.Inking.InkPoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Input::Inking::InkPoint*> __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_t;
#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_USE
#define DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0630c0ef-a4e2-5af6-b2e9-8e042e294e17"))
IIterable<ABI::Windows::UI::Input::Inking::InkPoint*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkPoint*, ABI::Windows::UI::Input::Inking::IInkPoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Input.Inking.InkPoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Input::Inking::InkPoint*> __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_t;
#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkRecognizer;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_USE
#define DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f8bd3097-5262-5e7a-a19d-13c029d2d7e5"))
IIterator<ABI::Windows::UI::Input::Inking::InkRecognizer*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkRecognizer*, ABI::Windows::UI::Input::Inking::IInkRecognizer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Input.Inking.InkRecognizer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Input::Inking::InkRecognizer*> __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_t;
#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer_USE
#define DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("611b7e84-a803-5071-aaea-4f2ce0151052"))
IIterable<ABI::Windows::UI::Input::Inking::InkRecognizer*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkRecognizer*, ABI::Windows::UI::Input::Inking::IInkRecognizer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Input.Inking.InkRecognizer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Input::Inking::InkRecognizer*> __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer_t;
#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkStroke;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_USE
#define DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5608d5a9-e7e4-5a0b-941f-b7fed76b35bf"))
IIterator<ABI::Windows::UI::Input::Inking::InkStroke*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkStroke*, ABI::Windows::UI::Input::Inking::IInkStroke*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Input.Inking.InkStroke>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Input::Inking::InkStroke*> __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_t;
#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_USE
#define DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bbc11401-89d0-5305-a3b3-36c887714b9b"))
IIterable<ABI::Windows::UI::Input::Inking::InkStroke*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkStroke*, ABI::Windows::UI::Input::Inking::IInkStroke*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Input.Inking.InkStroke>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Input::Inking::InkStroke*> __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_t;
#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkStrokeRenderingSegment;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_USE
#define DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d7d8c317-6f3f-5192-9210-65a263baf8d1"))
IIterator<ABI::Windows::UI::Input::Inking::InkStrokeRenderingSegment*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkStrokeRenderingSegment*, ABI::Windows::UI::Input::Inking::IInkStrokeRenderingSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Input.Inking.InkStrokeRenderingSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Input::Inking::InkStrokeRenderingSegment*> __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_t;
#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_USE
#define DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("27000f47-2885-5da9-8923-16a3a58b7a55"))
IIterable<ABI::Windows::UI::Input::Inking::InkStrokeRenderingSegment*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkStrokeRenderingSegment*, ABI::Windows::UI::Input::Inking::IInkStrokeRenderingSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Input.Inking.InkStrokeRenderingSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Input::Inking::InkStrokeRenderingSegment*> __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_t;
#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIVectorView_1_HSTRING_USE
#define DEF___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2f13c006-a03a-5f69-b090-75a43e33423e"))
IVectorView<HSTRING> : IVectorView_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<HSTRING> __FIVectorView_1_HSTRING_t;
#define __FIVectorView_1_HSTRING ABI::Windows::Foundation::Collections::__FIVectorView_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_USE
#define DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d1ac414b-c87d-540f-8ab1-4e0d09d9d283"))
IVectorView<ABI::Windows::UI::Input::Inking::InkPoint*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkPoint*, ABI::Windows::UI::Input::Inking::IInkPoint*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Input.Inking.InkPoint>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Input::Inking::InkPoint*> __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_t;
#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_USE
#define DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8eadfa4f-27ef-5a5d-b0b8-7fd5c0ce6b39"))
IVectorView<ABI::Windows::UI::Input::Inking::InkRecognizer*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkRecognizer*, ABI::Windows::UI::Input::Inking::IInkRecognizer*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Input.Inking.InkRecognizer>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Input::Inking::InkRecognizer*> __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_t;
#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_USE
#define DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6744f458-b242-5767-a643-996e01dff0e4"))
IVectorView<ABI::Windows::UI::Input::Inking::InkStroke*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkStroke*, ABI::Windows::UI::Input::Inking::IInkStroke*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Input.Inking.InkStroke>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Input::Inking::InkStroke*> __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_t;
#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_USE
#define DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f39ea41d-4714-5d80-87de-973dd26da269"))
IVectorView<ABI::Windows::UI::Input::Inking::InkStrokeRenderingSegment*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkStrokeRenderingSegment*, ABI::Windows::UI::Input::Inking::IInkStrokeRenderingSegment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Input.Inking.InkStrokeRenderingSegment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Input::Inking::InkStrokeRenderingSegment*> __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_t;
#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CDateTime_USE
#define DEF___FIReference_1_Windows__CFoundation__CDateTime_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5541d8a7-497c-5aa4-86fc-7713adbf2a2c"))
IReference<struct ABI::Windows::Foundation::DateTime> : IReference_impl<struct ABI::Windows::Foundation::DateTime>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.DateTime>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::DateTime> __FIReference_1_Windows__CFoundation__CDateTime_t;
#define __FIReference_1_Windows__CFoundation__CDateTime ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CDateTime_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CDateTime_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CTimeSpan_USE
#define DEF___FIReference_1_Windows__CFoundation__CTimeSpan_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("604d0c4c-91de-5c2a-935f-362f13eaf800"))
IReference<struct ABI::Windows::Foundation::TimeSpan> : IReference_impl<struct ABI::Windows::Foundation::TimeSpan>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.TimeSpan>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::TimeSpan> __FIReference_1_Windows__CFoundation__CTimeSpan_t;
#define __FIReference_1_Windows__CFoundation__CTimeSpan ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CTimeSpan_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CTimeSpan_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkPresenter;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkStrokesCollectedEventArgs;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("176bfa8f-c0de-5b3a-b28c-0f3931ca52d3"))
ITypedEventHandler<ABI::Windows::UI::Input::Inking::InkPresenter*, ABI::Windows::UI::Input::Inking::InkStrokesCollectedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkPresenter*, ABI::Windows::UI::Input::Inking::IInkPresenter*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkStrokesCollectedEventArgs*, ABI::Windows::UI::Input::Inking::IInkStrokesCollectedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Inking.InkPresenter, Windows.UI.Input.Inking.InkStrokesCollectedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Inking::InkPresenter*, ABI::Windows::UI::Input::Inking::InkStrokesCollectedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkStrokesErasedEventArgs;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("30fec929-14d0-550f-84f2-137fc6a9f08f"))
ITypedEventHandler<ABI::Windows::UI::Input::Inking::InkPresenter*, ABI::Windows::UI::Input::Inking::InkStrokesErasedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkPresenter*, ABI::Windows::UI::Input::Inking::IInkPresenter*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkStrokesErasedEventArgs*, ABI::Windows::UI::Input::Inking::IInkStrokesErasedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Inking.InkPresenter, Windows.UI.Input.Inking.InkStrokesErasedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Inking::InkPresenter*, ABI::Windows::UI::Input::Inking::InkStrokesErasedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkStrokeInput;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                class PointerEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Core {
                interface IPointerEventArgs;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs ABI::Windows::UI::Core::IPointerEventArgs

#endif // ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bf66b962-702d-5c07-a2d5-15f21583c43a"))
ITypedEventHandler<ABI::Windows::UI::Input::Inking::InkStrokeInput*, ABI::Windows::UI::Core::PointerEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkStrokeInput*, ABI::Windows::UI::Input::Inking::IInkStrokeInput*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::PointerEventArgs*, ABI::Windows::UI::Core::IPointerEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Inking.InkStrokeInput, Windows.UI.Core.PointerEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Inking::InkStrokeInput*, ABI::Windows::UI::Core::PointerEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkUnprocessedInput;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4a86bd78-5bcf-5ede-8f65-a8c65235055c"))
ITypedEventHandler<ABI::Windows::UI::Input::Inking::InkUnprocessedInput*, ABI::Windows::UI::Core::PointerEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Input::Inking::InkUnprocessedInput*, ABI::Windows::UI::Input::Inking::IInkUnprocessedInput*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Core::PointerEventArgs*, ABI::Windows::UI::Core::IPointerEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Input.Inking.InkUnprocessedInput, Windows.UI.Core.PointerEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Input::Inking::InkUnprocessedInput*, ABI::Windows::UI::Core::PointerEventArgs*> __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
            namespace Numerics {
                typedef struct Matrix3x2 Matrix3x2;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
            typedef struct Size Size;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IInputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIInputStream ABI::Windows::Storage::Streams::IInputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IOutputStream;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIOutputStream ABI::Windows::Storage::Streams::IOutputStream

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

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
                typedef enum CoreInputDeviceTypes : unsigned int CoreInputDeviceTypes;
            } /* Core */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                class PointerPoint;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                interface IPointerPoint;
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CInput_CIPointerPoint ABI::Windows::UI::Input::IPointerPoint

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    typedef enum HandwritingLineHeight : int HandwritingLineHeight;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    typedef enum InkDrawingAttributesKind : int InkDrawingAttributesKind;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    typedef enum InkHighContrastAdjustment : int InkHighContrastAdjustment;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    typedef enum InkInputProcessingMode : int InkInputProcessingMode;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    typedef enum InkInputRightDragAction : int InkInputRightDragAction;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    typedef enum InkManipulationMode : int InkManipulationMode;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    typedef enum InkPersistenceFormat : int InkPersistenceFormat;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    typedef enum InkPresenterPredefinedConfiguration : int InkPresenterPredefinedConfiguration;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    typedef enum InkPresenterStencilKind : int InkPresenterStencilKind;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    typedef enum InkRecognitionTarget : int InkRecognitionTarget;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    typedef enum PenHandedness : int PenHandedness;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    typedef enum PenTipShape : int PenTipShape;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkDrawingAttributes;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkDrawingAttributesPencilProperties;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkInputConfiguration;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkInputProcessingConfiguration;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkModelerAttributes;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkPresenterProtractor;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkPresenterRuler;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkStrokeContainer;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class InkSynchronizer;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    class PenAndInkSettings;
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Input.Inking.HandwritingLineHeight
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    enum HandwritingLineHeight : int
                    {
                        HandwritingLineHeight_Small = 0,
                        HandwritingLineHeight_Medium = 1,
                        HandwritingLineHeight_Large = 2,
                    };
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.UI.Input.Inking.InkDrawingAttributesKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    enum InkDrawingAttributesKind : int
                    {
                        InkDrawingAttributesKind_Default = 0,
                        InkDrawingAttributesKind_Pencil = 1,
                    };
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Inking.InkHighContrastAdjustment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    enum InkHighContrastAdjustment : int
                    {
                        InkHighContrastAdjustment_UseSystemColorsWhenNecessary = 0,
                        InkHighContrastAdjustment_UseSystemColors = 1,
                        InkHighContrastAdjustment_UseOriginalColors = 2,
                    };
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.Input.Inking.InkInputProcessingMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    enum InkInputProcessingMode : int
                    {
                        InkInputProcessingMode_None = 0,
                        InkInputProcessingMode_Inking = 1,
                        InkInputProcessingMode_Erasing = 2,
                    };
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Inking.InkInputRightDragAction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    enum InkInputRightDragAction : int
                    {
                        InkInputRightDragAction_LeaveUnprocessed = 0,
                        InkInputRightDragAction_AllowProcessing = 1,
                    };
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Inking.InkManipulationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    enum InkManipulationMode : int
                    {
                        InkManipulationMode_Inking = 0,
                        InkManipulationMode_Erasing = 1,
                        InkManipulationMode_Selecting = 2,
                    };
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Inking.InkPersistenceFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    enum InkPersistenceFormat : int
                    {
                        InkPersistenceFormat_GifWithEmbeddedIsf = 0,
                        InkPersistenceFormat_Isf = 1,
                    };
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.Input.Inking.InkPresenterPredefinedConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    enum InkPresenterPredefinedConfiguration : int
                    {
                        InkPresenterPredefinedConfiguration_SimpleSinglePointer = 0,
                        InkPresenterPredefinedConfiguration_SimpleMultiplePointer = 1,
                    };
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Inking.InkPresenterStencilKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    enum InkPresenterStencilKind : int
                    {
                        InkPresenterStencilKind_Other = 0,
                        InkPresenterStencilKind_Ruler = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        InkPresenterStencilKind_Protractor = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    };
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Inking.InkRecognitionTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    enum InkRecognitionTarget : int
                    {
                        InkRecognitionTarget_All = 0,
                        InkRecognitionTarget_Selected = 1,
                        InkRecognitionTarget_Recent = 2,
                    };
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Inking.PenHandedness
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    enum PenHandedness : int
                    {
                        PenHandedness_Right = 0,
                        PenHandedness_Left = 1,
                    };
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.UI.Input.Inking.PenTipShape
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    enum PenTipShape : int
                    {
                        PenTipShape_Circle = 0,
                        PenTipShape_Rectangle = 1,
                    };
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkDrawingAttributes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkDrawingAttributes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkDrawingAttributes[] = L"Windows.UI.Input.Inking.IInkDrawingAttributes";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("97a2176c-6774-48ad-84f0-48f5a9be74f9")
                    IInkDrawingAttributes : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Color(
                            ABI::Windows::UI::Color* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Color(
                            ABI::Windows::UI::Color value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PenTip(
                            ABI::Windows::UI::Input::Inking::PenTipShape* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PenTip(
                            ABI::Windows::UI::Input::Inking::PenTipShape value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Size(
                            ABI::Windows::Foundation::Size* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Size(
                            ABI::Windows::Foundation::Size value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IgnorePressure(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IgnorePressure(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FitToCurve(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_FitToCurve(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkDrawingAttributes = __uuidof(IInkDrawingAttributes);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkDrawingAttributes2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkDrawingAttributes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkDrawingAttributes2[] = L"Windows.UI.Input.Inking.IInkDrawingAttributes2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("7cab6508-8ec4-42fd-a5a5-e4b7d1d5316d")
                    IInkDrawingAttributes2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PenTipTransform(
                            ABI::Windows::Foundation::Numerics::Matrix3x2* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PenTipTransform(
                            ABI::Windows::Foundation::Numerics::Matrix3x2 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DrawAsHighlighter(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DrawAsHighlighter(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkDrawingAttributes2 = __uuidof(IInkDrawingAttributes2);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkDrawingAttributes3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkDrawingAttributes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkDrawingAttributes3[] = L"Windows.UI.Input.Inking.IInkDrawingAttributes3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("72020002-7d5b-4690-8af4-e664cbe2b74f")
                    IInkDrawingAttributes3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Kind(
                            ABI::Windows::UI::Input::Inking::InkDrawingAttributesKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PencilProperties(
                            ABI::Windows::UI::Input::Inking::IInkDrawingAttributesPencilProperties** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkDrawingAttributes3 = __uuidof(IInkDrawingAttributes3);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkDrawingAttributes4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkDrawingAttributes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkDrawingAttributes4[] = L"Windows.UI.Input.Inking.IInkDrawingAttributes4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("ef65dc25-9f19-456d-91a3-bc3a3d91c5fb")
                    IInkDrawingAttributes4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IgnoreTilt(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IgnoreTilt(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkDrawingAttributes4 = __uuidof(IInkDrawingAttributes4);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkDrawingAttributes5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkDrawingAttributes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkDrawingAttributes5[] = L"Windows.UI.Input.Inking.IInkDrawingAttributes5";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("d11aa0bb-0775-4852-ae64-41143a7ae6c9")
                    IInkDrawingAttributes5 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ModelerAttributes(
                            ABI::Windows::UI::Input::Inking::IInkModelerAttributes** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkDrawingAttributes5 = __uuidof(IInkDrawingAttributes5);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkDrawingAttributesPencilProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkDrawingAttributesPencilProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkDrawingAttributesPencilProperties[] = L"Windows.UI.Input.Inking.IInkDrawingAttributesPencilProperties";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("4f2534cb-2d86-41bb-b0e8-e4c2a0253c52")
                    IInkDrawingAttributesPencilProperties : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Opacity(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Opacity(
                            DOUBLE value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkDrawingAttributesPencilProperties = __uuidof(IInkDrawingAttributesPencilProperties);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkDrawingAttributesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkDrawingAttributes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkDrawingAttributesStatics[] = L"Windows.UI.Input.Inking.IInkDrawingAttributesStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("f731e03f-1a65-4862-96cb-6e1665e17f6d")
                    IInkDrawingAttributesStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateForPencil(
                            ABI::Windows::UI::Input::Inking::IInkDrawingAttributes** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkDrawingAttributesStatics = __uuidof(IInkDrawingAttributesStatics);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkInputConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkInputConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkInputConfiguration[] = L"Windows.UI.Input.Inking.IInkInputConfiguration";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("93a68dc4-0b7b-49d7-b34f-9901e524dcf2")
                    IInkInputConfiguration : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsPrimaryBarrelButtonInputEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsPrimaryBarrelButtonInputEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsEraserInputEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsEraserInputEnabled(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkInputConfiguration = __uuidof(IInkInputConfiguration);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkInputConfiguration2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkInputConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkInputConfiguration2[] = L"Windows.UI.Input.Inking.IInkInputConfiguration2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("6ac2272e-81b4-5cc4-a36d-d057c387dfda")
                    IInkInputConfiguration2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsPenHapticFeedbackEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsPenHapticFeedbackEnabled(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkInputConfiguration2 = __uuidof(IInkInputConfiguration2);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkInputProcessingConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkInputProcessingConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkInputProcessingConfiguration[] = L"Windows.UI.Input.Inking.IInkInputProcessingConfiguration";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("2778d85e-33ca-4b06-a6d3-ac3945116d37")
                    IInkInputProcessingConfiguration : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Mode(
                            ABI::Windows::UI::Input::Inking::InkInputProcessingMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Mode(
                            ABI::Windows::UI::Input::Inking::InkInputProcessingMode value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RightDragAction(
                            ABI::Windows::UI::Input::Inking::InkInputRightDragAction* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_RightDragAction(
                            ABI::Windows::UI::Input::Inking::InkInputRightDragAction value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkInputProcessingConfiguration = __uuidof(IInkInputProcessingConfiguration);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.IInkStrokeContainer
 *     Windows.UI.Input.Inking.IInkRecognizerContainer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkManager[] = L"Windows.UI.Input.Inking.IInkManager";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("4744737d-671b-4163-9c95-4e8d7a035fe1")
                    IInkManager : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Mode(
                            ABI::Windows::UI::Input::Inking::InkManipulationMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Mode(
                            ABI::Windows::UI::Input::Inking::InkManipulationMode value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ProcessPointerDown(
                            ABI::Windows::UI::Input::IPointerPoint* pointerPoint
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ProcessPointerUpdate(
                            ABI::Windows::UI::Input::IPointerPoint* pointerPoint,
                            IInspectable** updateInformation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ProcessPointerUp(
                            ABI::Windows::UI::Input::IPointerPoint* pointerPoint,
                            ABI::Windows::Foundation::Rect* updateRectangle
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetDefaultDrawingAttributes(
                            ABI::Windows::UI::Input::Inking::IInkDrawingAttributes* drawingAttributes
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RecognizeAsync2(
                            ABI::Windows::UI::Input::Inking::InkRecognitionTarget recognitionTarget,
                            __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult** recognitionResults
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkManager = __uuidof(IInkManager);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkModelerAttributes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkModelerAttributes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkModelerAttributes[] = L"Windows.UI.Input.Inking.IInkModelerAttributes";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("bad31f27-0cd9-4bfd-b6f3-9e03ba8d7454")
                    IInkModelerAttributes : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PredictionTime(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PredictionTime(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ScalingFactor(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ScalingFactor(
                            FLOAT value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkModelerAttributes = __uuidof(IInkModelerAttributes);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkModelerAttributes2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkModelerAttributes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkModelerAttributes2[] = L"Windows.UI.Input.Inking.IInkModelerAttributes2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("86d1d09a-4ef8-5e25-b7bc-b65424f16bb3")
                    IInkModelerAttributes2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_UseVelocityBasedPressure(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_UseVelocityBasedPressure(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkModelerAttributes2 = __uuidof(IInkModelerAttributes2);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPoint[] = L"Windows.UI.Input.Inking.IInkPoint";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("9f87272b-858c-46a5-9b41-d195970459fd")
                    IInkPoint : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Position(
                            ABI::Windows::Foundation::Point* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Pressure(
                            FLOAT* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkPoint = __uuidof(IInkPoint);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPoint2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPoint2[] = L"Windows.UI.Input.Inking.IInkPoint2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("fba9c3f7-ae56-4d5c-bd2f-0ac45f5e4af9")
                    IInkPoint2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_TiltX(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TiltY(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                            UINT64* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkPoint2 = __uuidof(IInkPoint2);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPointFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPointFactory[] = L"Windows.UI.Input.Inking.IInkPointFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("29e5d51c-c98f-405d-9f3b-e53e31068d4d")
                    IInkPointFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInkPoint(
                            ABI::Windows::Foundation::Point position,
                            FLOAT pressure,
                            ABI::Windows::UI::Input::Inking::IInkPoint** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkPointFactory = __uuidof(IInkPointFactory);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPointFactory2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPointFactory2[] = L"Windows.UI.Input.Inking.IInkPointFactory2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("e0145e85-daff-45f2-ad69-050d8256a209")
                    IInkPointFactory2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInkPointWithTiltAndTimestamp(
                            ABI::Windows::Foundation::Point position,
                            FLOAT pressure,
                            FLOAT tiltX,
                            FLOAT tiltY,
                            UINT64 timestamp,
                            ABI::Windows::UI::Input::Inking::IInkPoint** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkPointFactory2 = __uuidof(IInkPointFactory2);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPresenter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenter[] = L"Windows.UI.Input.Inking.IInkPresenter";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("a69b70e2-887b-458f-b173-4fe4438930a3")
                    IInkPresenter : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsInputEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsInputEnabled(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_InputDeviceTypes(
                            ABI::Windows::UI::Core::CoreInputDeviceTypes* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_InputDeviceTypes(
                            ABI::Windows::UI::Core::CoreInputDeviceTypes value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UnprocessedInput(
                            ABI::Windows::UI::Input::Inking::IInkUnprocessedInput** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeInput(
                            ABI::Windows::UI::Input::Inking::IInkStrokeInput** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_InputProcessingConfiguration(
                            ABI::Windows::UI::Input::Inking::IInkInputProcessingConfiguration** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeContainer(
                            ABI::Windows::UI::Input::Inking::IInkStrokeContainer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StrokeContainer(
                            ABI::Windows::UI::Input::Inking::IInkStrokeContainer* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CopyDefaultDrawingAttributes(
                            ABI::Windows::UI::Input::Inking::IInkDrawingAttributes** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UpdateDefaultDrawingAttributes(
                            ABI::Windows::UI::Input::Inking::IInkDrawingAttributes* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ActivateCustomDrying(
                            ABI::Windows::UI::Input::Inking::IInkSynchronizer** inkSynchronizer
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPredefinedConfiguration(
                            ABI::Windows::UI::Input::Inking::InkPresenterPredefinedConfiguration value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_StrokesCollected(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_StrokesCollected(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_StrokesErased(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_StrokesErased(
                            EventRegistrationToken cookie
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkPresenter = __uuidof(IInkPresenter);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPresenter
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.IInkPresenter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenter2[] = L"Windows.UI.Input.Inking.IInkPresenter2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("cf53e612-9a34-11e6-9f33-a24fc0d9649c")
                    IInkPresenter2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_HighContrastAdjustment(
                            ABI::Windows::UI::Input::Inking::InkHighContrastAdjustment* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_HighContrastAdjustment(
                            ABI::Windows::UI::Input::Inking::InkHighContrastAdjustment value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkPresenter2 = __uuidof(IInkPresenter2);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenter3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPresenter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenter3[] = L"Windows.UI.Input.Inking.IInkPresenter3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("51e1ce89-d37d-4a90-83fc-7f5e9dfbf217")
                    IInkPresenter3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InputConfiguration(
                            ABI::Windows::UI::Input::Inking::IInkInputConfiguration** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkPresenter3 = __uuidof(IInkPresenter3);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenterProtractor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPresenterProtractor
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.IInkPresenterStencil
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenterProtractor[] = L"Windows.UI.Input.Inking.IInkPresenterProtractor";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("7de3f2aa-ef6c-4e91-a73b-5b70d56fbd17")
                    IInkPresenterProtractor : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AreTickMarksVisible(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AreTickMarksVisible(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AreRaysVisible(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AreRaysVisible(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsCenterMarkerVisible(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsCenterMarkerVisible(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsAngleReadoutVisible(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsAngleReadoutVisible(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsResizable(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsResizable(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Radius(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Radius(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AccentColor(
                            ABI::Windows::UI::Color* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AccentColor(
                            ABI::Windows::UI::Color value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkPresenterProtractor = __uuidof(IInkPresenterProtractor);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenterProtractorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPresenterProtractor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenterProtractorFactory[] = L"Windows.UI.Input.Inking.IInkPresenterProtractorFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("320103c9-68fa-47e9-8127-8370711fc46c")
                    IInkPresenterProtractorFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            ABI::Windows::UI::Input::Inking::IInkPresenter* inkPresenter,
                            ABI::Windows::UI::Input::Inking::IInkPresenterProtractor** inkPresenterProtractor
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkPresenterProtractorFactory = __uuidof(IInkPresenterProtractorFactory);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenterRuler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPresenterRuler
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.IInkPresenterStencil
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenterRuler[] = L"Windows.UI.Input.Inking.IInkPresenterRuler";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("6cda7d5a-dec7-4dd7-877a-2133f183d48a")
                    IInkPresenterRuler : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Length(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Length(
                            DOUBLE value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Width(
                            DOUBLE* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Width(
                            DOUBLE value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkPresenterRuler = __uuidof(IInkPresenterRuler);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenterRuler2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPresenterRuler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenterRuler2[] = L"Windows.UI.Input.Inking.IInkPresenterRuler2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("45130dc1-bc61-44d4-a423-54712ae671c4")
                    IInkPresenterRuler2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AreTickMarksVisible(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AreTickMarksVisible(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsCompassVisible(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsCompassVisible(
                            boolean value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkPresenterRuler2 = __uuidof(IInkPresenterRuler2);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenterRulerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenterRulerFactory[] = L"Windows.UI.Input.Inking.IInkPresenterRulerFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("34361beb-9001-4a4b-a690-69dbaf63e501")
                    IInkPresenterRulerFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Create(
                            ABI::Windows::UI::Input::Inking::IInkPresenter* inkPresenter,
                            ABI::Windows::UI::Input::Inking::IInkPresenterRuler** inkPresenterRuler
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkPresenterRulerFactory = __uuidof(IInkPresenterRulerFactory);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenterStencil
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenterStencil[] = L"Windows.UI.Input.Inking.IInkPresenterStencil";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("30d12d6d-3e06-4d02-b116-277fb5d8addc")
                    IInkPresenterStencil : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Kind(
                            ABI::Windows::UI::Input::Inking::InkPresenterStencilKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsVisible(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsVisible(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BackgroundColor(
                            ABI::Windows::UI::Color* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_BackgroundColor(
                            ABI::Windows::UI::Color value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ForegroundColor(
                            ABI::Windows::UI::Color* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ForegroundColor(
                            ABI::Windows::UI::Color value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Transform(
                            ABI::Windows::Foundation::Numerics::Matrix3x2* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Transform(
                            ABI::Windows::Foundation::Numerics::Matrix3x2 value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkPresenterStencil = __uuidof(IInkPresenterStencil);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkRecognitionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkRecognitionResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkRecognitionResult[] = L"Windows.UI.Input.Inking.IInkRecognitionResult";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("36461a94-5068-40ef-8a05-2c2fb60908a2")
                    IInkRecognitionResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_BoundingRect(
                            ABI::Windows::Foundation::Rect* boundingRect
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetTextCandidates(
                            __FIVectorView_1_HSTRING** textCandidates
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStrokes(
                            __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke** strokes
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkRecognitionResult = __uuidof(IInkRecognitionResult);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkRecognizer[] = L"Windows.UI.Input.Inking.IInkRecognizer";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("077ccea3-904d-442a-b151-aaca3631c43b")
                    IInkRecognizer : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkRecognizer = __uuidof(IInkRecognizer);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkRecognizerContainer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkRecognizerContainer[] = L"Windows.UI.Input.Inking.IInkRecognizerContainer";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("a74d9a31-8047-4698-a912-f82a5085012f")
                    IInkRecognizerContainer : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetDefaultRecognizer(
                            ABI::Windows::UI::Input::Inking::IInkRecognizer* recognizer
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RecognizeAsync(
                            ABI::Windows::UI::Input::Inking::IInkStrokeContainer* strokeCollection,
                            ABI::Windows::UI::Input::Inking::InkRecognitionTarget recognitionTarget,
                            __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult** recognitionResults
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetRecognizers(
                            __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer** recognizerView
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkRecognizerContainer = __uuidof(IInkRecognizerContainer);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStroke
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStroke
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStroke[] = L"Windows.UI.Input.Inking.IInkStroke";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("15144d60-cce3-4fcf-9d52-11518ab6afd4")
                    IInkStroke : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DrawingAttributes(
                            ABI::Windows::UI::Input::Inking::IInkDrawingAttributes** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DrawingAttributes(
                            ABI::Windows::UI::Input::Inking::IInkDrawingAttributes* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BoundingRect(
                            ABI::Windows::Foundation::Rect* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Selected(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Selected(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Recognized(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetRenderingSegments(
                            __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment** renderingSegments
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Clone(
                            ABI::Windows::UI::Input::Inking::IInkStroke** clonedStroke
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkStroke = __uuidof(IInkStroke);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStroke2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStroke
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStroke2[] = L"Windows.UI.Input.Inking.IInkStroke2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("5db9e4f4-bafa-4de1-89d3-201b1ed7d89b")
                    IInkStroke2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PointTransform(
                            ABI::Windows::Foundation::Numerics::Matrix3x2* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_PointTransform(
                            ABI::Windows::Foundation::Numerics::Matrix3x2 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetInkPoints(
                            __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint** inkPoints
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkStroke2 = __uuidof(IInkStroke2);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStroke3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStroke
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStroke3[] = L"Windows.UI.Input.Inking.IInkStroke3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("4a807374-9499-411d-a1c4-68855d03d65f")
                    IInkStroke3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeStartedTime(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StrokeStartedTime(
                            __FIReference_1_Windows__CFoundation__CDateTime* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StrokeDuration(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_StrokeDuration(
                            __FIReference_1_Windows__CFoundation__CTimeSpan* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkStroke3 = __uuidof(IInkStroke3);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStroke4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStroke
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStroke4[] = L"Windows.UI.Input.Inking.IInkStroke4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("cd5b62e5-b6e9-5b91-a577-1921d2348690")
                    IInkStroke4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PointerId(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkStroke4 = __uuidof(IInkStroke4);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokeBuilder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokeBuilder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokeBuilder[] = L"Windows.UI.Input.Inking.IInkStrokeBuilder";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("82bbd1dc-1c63-41dc-9e07-4b4a70ced801")
                    IInkStrokeBuilder : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE BeginStroke(
                            ABI::Windows::UI::Input::IPointerPoint* pointerPoint
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AppendToStroke(
                            ABI::Windows::UI::Input::IPointerPoint* pointerPoint,
                            ABI::Windows::UI::Input::IPointerPoint** previousPointerPoint
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE EndStroke(
                            ABI::Windows::UI::Input::IPointerPoint* pointerPoint,
                            ABI::Windows::UI::Input::Inking::IInkStroke** stroke
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateStroke(
                            __FIIterable_1_Windows__CFoundation__CPoint* points,
                            ABI::Windows::UI::Input::Inking::IInkStroke** stroke
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetDefaultDrawingAttributes(
                            ABI::Windows::UI::Input::Inking::IInkDrawingAttributes* drawingAttributes
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkStrokeBuilder = __uuidof(IInkStrokeBuilder);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokeBuilder2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokeBuilder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokeBuilder2[] = L"Windows.UI.Input.Inking.IInkStrokeBuilder2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("bd82bc27-731f-4cbc-bbbf-6d468044f1e5")
                    IInkStrokeBuilder2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateStrokeFromInkPoints(
                            __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* inkPoints,
                            ABI::Windows::Foundation::Numerics::Matrix3x2 transform,
                            ABI::Windows::UI::Input::Inking::IInkStroke** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkStrokeBuilder2 = __uuidof(IInkStrokeBuilder2);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokeBuilder3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokeBuilder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokeBuilder3[] = L"Windows.UI.Input.Inking.IInkStrokeBuilder3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("b2c71fcd-5472-46b1-a81d-c37a3d169441")
                    IInkStrokeBuilder3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateStrokeFromInkPoints(
                            __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* inkPoints,
                            ABI::Windows::Foundation::Numerics::Matrix3x2 transform,
                            __FIReference_1_Windows__CFoundation__CDateTime* strokeStartedTime,
                            __FIReference_1_Windows__CFoundation__CTimeSpan* strokeDuration,
                            ABI::Windows::UI::Input::Inking::IInkStroke** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkStrokeBuilder3 = __uuidof(IInkStrokeBuilder3);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokeContainer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokeContainer[] = L"Windows.UI.Input.Inking.IInkStrokeContainer";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("22accbc6-faa9-4f14-b68c-f6cee670ae16")
                    IInkStrokeContainer : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_BoundingRect(
                            ABI::Windows::Foundation::Rect* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE AddStroke(
                            ABI::Windows::UI::Input::Inking::IInkStroke* stroke
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE DeleteSelected(
                            ABI::Windows::Foundation::Rect* invalidatedRect
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE MoveSelected(
                            ABI::Windows::Foundation::Point translation,
                            ABI::Windows::Foundation::Rect* invalidatedRectangle
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SelectWithPolyLine(
                            __FIIterable_1_Windows__CFoundation__CPoint* polyline,
                            ABI::Windows::Foundation::Rect* invalidatedRectangle
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SelectWithLine(
                            ABI::Windows::Foundation::Point from,
                            ABI::Windows::Foundation::Point to,
                            ABI::Windows::Foundation::Rect* invalidatedRectangle
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CopySelectedToClipboard(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE PasteFromClipboard(
                            ABI::Windows::Foundation::Point position,
                            ABI::Windows::Foundation::Rect* invalidatedRectangle
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CanPasteFromClipboard(
                            boolean* canPaste
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE LoadAsync(
                            ABI::Windows::Storage::Streams::IInputStream* inputStream,
                            __FIAsyncActionWithProgress_1_UINT64** loadAction
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SaveAsync(
                            ABI::Windows::Storage::Streams::IOutputStream* outputStream,
                            __FIAsyncOperationWithProgress_2_UINT32_UINT32** outputStreamOperation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE UpdateRecognitionResults(
                            __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* recognitionResults
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStrokes(
                            __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke** strokeView
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetRecognitionResults(
                            __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult** recognitionResults
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkStrokeContainer = __uuidof(IInkStrokeContainer);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokeContainer2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokeContainer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokeContainer2[] = L"Windows.UI.Input.Inking.IInkStrokeContainer2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("8901d364-da36-4bcf-9e5c-d195825995b4")
                    IInkStrokeContainer2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE AddStrokes(
                            __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* strokes
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Clear(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkStrokeContainer2 = __uuidof(IInkStrokeContainer2);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokeContainer3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokeContainer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokeContainer3[] = L"Windows.UI.Input.Inking.IInkStrokeContainer3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("3d07bea5-baea-4c82-a719-7b83da1067d2")
                    IInkStrokeContainer3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SaveWithFormatAsync(
                            ABI::Windows::Storage::Streams::IOutputStream* outputStream,
                            ABI::Windows::UI::Input::Inking::InkPersistenceFormat inkPersistenceFormat,
                            __FIAsyncOperationWithProgress_2_UINT32_UINT32** outputStreamOperation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetStrokeById(
                            UINT32 id,
                            ABI::Windows::UI::Input::Inking::IInkStroke** stroke
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkStrokeContainer3 = __uuidof(IInkStrokeContainer3);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokeInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokeInput
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokeInput[] = L"Windows.UI.Input.Inking.IInkStrokeInput";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("cf2ffe7b-5e10-43c6-a080-88f26e1dc67d")
                    IInkStrokeInput : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_StrokeStarted(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_StrokeStarted(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_StrokeContinued(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_StrokeContinued(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_StrokeEnded(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_StrokeEnded(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_StrokeCanceled(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_StrokeCanceled(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_InkPresenter(
                            ABI::Windows::UI::Input::Inking::IInkPresenter** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkStrokeInput = __uuidof(IInkStrokeInput);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokeRenderingSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokeRenderingSegment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokeRenderingSegment[] = L"Windows.UI.Input.Inking.IInkStrokeRenderingSegment";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("68510f1f-88e3-477a-a2fa-569f5f1f9bd5")
                    IInkStrokeRenderingSegment : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Position(
                            ABI::Windows::Foundation::Point* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BezierControlPoint1(
                            ABI::Windows::Foundation::Point* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BezierControlPoint2(
                            ABI::Windows::Foundation::Point* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Pressure(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TiltX(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TiltY(
                            FLOAT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Twist(
                            FLOAT* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkStrokeRenderingSegment = __uuidof(IInkStrokeRenderingSegment);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokesCollectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokesCollectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokesCollectedEventArgs[] = L"Windows.UI.Input.Inking.IInkStrokesCollectedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("c4f3f229-1938-495c-b4d9-6de4b08d4811")
                    IInkStrokesCollectedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Strokes(
                            __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkStrokesCollectedEventArgs = __uuidof(IInkStrokesCollectedEventArgs);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokesErasedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokesErasedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokesErasedEventArgs[] = L"Windows.UI.Input.Inking.IInkStrokesErasedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("a4216a22-1503-4ebf-8ff5-2de84584a8aa")
                    IInkStrokesErasedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Strokes(
                            __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkStrokesErasedEventArgs = __uuidof(IInkStrokesErasedEventArgs);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkSynchronizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkSynchronizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkSynchronizer[] = L"Windows.UI.Input.Inking.IInkSynchronizer";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("9b9ea160-ae9b-45f9-8407-4b493b163661")
                    IInkSynchronizer : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE BeginDry(
                            __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke** inkStrokes
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE EndDry(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkSynchronizer = __uuidof(IInkSynchronizer);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkUnprocessedInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkUnprocessedInput
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkUnprocessedInput[] = L"Windows.UI.Input.Inking.IInkUnprocessedInput";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("db4445e0-8398-4921-ac3b-ab978c5ba256")
                    IInkUnprocessedInput : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_PointerEntered(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PointerEntered(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_PointerHovered(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PointerHovered(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_PointerExited(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PointerExited(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_PointerPressed(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PointerPressed(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_PointerMoved(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PointerMoved(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_PointerReleased(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PointerReleased(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_PointerLost(
                            __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PointerLost(
                            EventRegistrationToken cookie
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_InkPresenter(
                            ABI::Windows::UI::Input::Inking::IInkPresenter** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInkUnprocessedInput = __uuidof(IInkUnprocessedInput);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IPenAndInkSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.PenAndInkSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IPenAndInkSettings[] = L"Windows.UI.Input.Inking.IPenAndInkSettings";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("bc2ceb8f-0066-44a8-bb7a-b839b3deb8f5")
                    IPenAndInkSettings : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsHandwritingDirectlyIntoTextFieldEnabled(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PenHandedness(
                            ABI::Windows::UI::Input::Inking::PenHandedness* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HandwritingLineHeight(
                            ABI::Windows::UI::Input::Inking::HandwritingLineHeight* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FontFamilyName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UserConsentsToHandwritingTelemetryCollection(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsTouchHandwritingEnabled(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPenAndInkSettings = __uuidof(IPenAndInkSettings);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Input.Inking.IPenAndInkSettings2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.PenAndInkSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IPenAndInkSettings2[] = L"Windows.UI.Input.Inking.IPenAndInkSettings2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("3262da53-1f44-55e2-9929-ebf77e5481b8")
                    IPenAndInkSettings2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE SetPenHandedness(
                            ABI::Windows::UI::Input::Inking::PenHandedness value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPenAndInkSettings2 = __uuidof(IPenAndInkSettings2);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.UI.Input.Inking.IPenAndInkSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.PenAndInkSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IPenAndInkSettingsStatics[] = L"Windows.UI.Input.Inking.IPenAndInkSettingsStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Input {
                namespace Inking {
                    MIDL_INTERFACE("ed6dd036-5708-5c3c-96db-f2f552eab641")
                    IPenAndInkSettingsStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetDefault(
                            ABI::Windows::UI::Input::Inking::IPenAndInkSettings** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPenAndInkSettingsStatics = __uuidof(IPenAndInkSettingsStatics);
                } /* Inking */
            } /* Input */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.Input.Inking.InkDrawingAttributes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Inking.IInkDrawingAttributesStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkDrawingAttributes ** Default Interface **
 *    Windows.UI.Input.Inking.IInkDrawingAttributes2
 *    Windows.UI.Input.Inking.IInkDrawingAttributes3
 *    Windows.UI.Input.Inking.IInkDrawingAttributes4
 *    Windows.UI.Input.Inking.IInkDrawingAttributes5
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkDrawingAttributes_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkDrawingAttributes_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkDrawingAttributes[] = L"Windows.UI.Input.Inking.InkDrawingAttributes";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkDrawingAttributesPencilProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkDrawingAttributesPencilProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkDrawingAttributesPencilProperties_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkDrawingAttributesPencilProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkDrawingAttributesPencilProperties[] = L"Windows.UI.Input.Inking.InkDrawingAttributesPencilProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.Inking.InkInputConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkInputConfiguration ** Default Interface **
 *    Windows.UI.Input.Inking.IInkInputConfiguration2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkInputConfiguration_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkInputConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkInputConfiguration[] = L"Windows.UI.Input.Inking.InkInputConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Input.Inking.InkInputProcessingConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkInputProcessingConfiguration ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkInputProcessingConfiguration_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkInputProcessingConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkInputProcessingConfiguration[] = L"Windows.UI.Input.Inking.InkInputProcessingConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkManager ** Default Interface **
 *    Windows.UI.Input.Inking.IInkRecognizerContainer
 *    Windows.UI.Input.Inking.IInkStrokeContainer
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkManager[] = L"Windows.UI.Input.Inking.InkManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkModelerAttributes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkModelerAttributes ** Default Interface **
 *    Windows.UI.Input.Inking.IInkModelerAttributes2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkModelerAttributes_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkModelerAttributes_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkModelerAttributes[] = L"Windows.UI.Input.Inking.InkModelerAttributes";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Input.Inking.InkPoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Input.Inking.IInkPointFactory2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.UI.Input.Inking.IInkPointFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkPoint ** Default Interface **
 *    Windows.UI.Input.Inking.IInkPoint2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkPoint_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkPoint_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkPoint[] = L"Windows.UI.Input.Inking.InkPoint";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkPresenter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkPresenter ** Default Interface **
 *    Windows.UI.Input.Inking.IInkPresenter2
 *    Windows.UI.Input.Inking.IInkPresenter3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkPresenter_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkPresenter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkPresenter[] = L"Windows.UI.Input.Inking.InkPresenter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkPresenterProtractor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Input.Inking.IInkPresenterProtractorFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkPresenterProtractor ** Default Interface **
 *    Windows.UI.Input.Inking.IInkPresenterStencil
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkPresenterProtractor_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkPresenterProtractor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkPresenterProtractor[] = L"Windows.UI.Input.Inking.InkPresenterProtractor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.InkPresenterRuler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Input.Inking.IInkPresenterRulerFactory interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkPresenterRuler ** Default Interface **
 *    Windows.UI.Input.Inking.IInkPresenterStencil
 *    Windows.UI.Input.Inking.IInkPresenterRuler2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkPresenterRuler_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkPresenterRuler_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkPresenterRuler[] = L"Windows.UI.Input.Inking.InkPresenterRuler";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.Inking.InkRecognitionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkRecognitionResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkRecognitionResult_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkRecognitionResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkRecognitionResult[] = L"Windows.UI.Input.Inking.InkRecognitionResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkRecognizer ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkRecognizer_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkRecognizer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkRecognizer[] = L"Windows.UI.Input.Inking.InkRecognizer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkRecognizerContainer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkRecognizerContainer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkRecognizerContainer_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkRecognizerContainer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkRecognizerContainer[] = L"Windows.UI.Input.Inking.InkRecognizerContainer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkStroke
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkStroke ** Default Interface **
 *    Windows.UI.Input.Inking.IInkStroke2
 *    Windows.UI.Input.Inking.IInkStroke3
 *    Windows.UI.Input.Inking.IInkStroke4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkStroke_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkStroke_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkStroke[] = L"Windows.UI.Input.Inking.InkStroke";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkStrokeBuilder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkStrokeBuilder ** Default Interface **
 *    Windows.UI.Input.Inking.IInkStrokeBuilder2
 *    Windows.UI.Input.Inking.IInkStrokeBuilder3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokeBuilder_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokeBuilder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkStrokeBuilder[] = L"Windows.UI.Input.Inking.InkStrokeBuilder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkStrokeContainer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkStrokeContainer ** Default Interface **
 *    Windows.UI.Input.Inking.IInkStrokeContainer2
 *    Windows.UI.Input.Inking.IInkStrokeContainer3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokeContainer_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokeContainer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkStrokeContainer[] = L"Windows.UI.Input.Inking.InkStrokeContainer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkStrokeInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkStrokeInput ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokeInput_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokeInput_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkStrokeInput[] = L"Windows.UI.Input.Inking.InkStrokeInput";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkStrokeRenderingSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkStrokeRenderingSegment ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokeRenderingSegment_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokeRenderingSegment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkStrokeRenderingSegment[] = L"Windows.UI.Input.Inking.InkStrokeRenderingSegment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkStrokesCollectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkStrokesCollectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokesCollectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokesCollectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkStrokesCollectedEventArgs[] = L"Windows.UI.Input.Inking.InkStrokesCollectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkStrokesErasedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkStrokesErasedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokesErasedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokesErasedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkStrokesErasedEventArgs[] = L"Windows.UI.Input.Inking.InkStrokesErasedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkSynchronizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkSynchronizer ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkSynchronizer_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkSynchronizer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkSynchronizer[] = L"Windows.UI.Input.Inking.InkSynchronizer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkUnprocessedInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkUnprocessedInput ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkUnprocessedInput_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkUnprocessedInput_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkUnprocessedInput[] = L"Windows.UI.Input.Inking.InkUnprocessedInput";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.PenAndInkSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Inking.IPenAndInkSettingsStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IPenAndInkSettings ** Default Interface **
 *    Windows.UI.Input.Inking.IPenAndInkSettings2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_PenAndInkSettings_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_PenAndInkSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_PenAndInkSettings[] = L"Windows.UI.Input.Inking.PenAndInkSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2 __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3 __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4 __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5 __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2 __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2 __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2 __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2 __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2 __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3 __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2 __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2 __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3 __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4 __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2 __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3 __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2 __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3 __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer __x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2 __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics;

#endif // ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncActionProgressHandler_1_UINT64 __FIAsyncActionProgressHandler_1_UINT64;

typedef interface __FIAsyncActionWithProgress_1_UINT64 __FIAsyncActionWithProgress_1_UINT64;

#if !defined(____FIAsyncActionWithProgressCompletedHandler_1_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncActionWithProgressCompletedHandler_1_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncActionWithProgressCompletedHandler_1_UINT64 __FIAsyncActionWithProgressCompletedHandler_1_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncActionWithProgressCompletedHandler_1_UINT64;

typedef struct __FIAsyncActionWithProgressCompletedHandler_1_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncActionWithProgressCompletedHandler_1_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncActionWithProgressCompletedHandler_1_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncActionWithProgressCompletedHandler_1_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncActionWithProgressCompletedHandler_1_UINT64* This,
        __FIAsyncActionWithProgress_1_UINT64* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncActionWithProgressCompletedHandler_1_UINT64Vtbl;

interface __FIAsyncActionWithProgressCompletedHandler_1_UINT64
{
    CONST_VTBL struct __FIAsyncActionWithProgressCompletedHandler_1_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncActionWithProgressCompletedHandler_1_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncActionWithProgressCompletedHandler_1_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncActionWithProgressCompletedHandler_1_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncActionWithProgressCompletedHandler_1_UINT64_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncActionWithProgressCompletedHandler_1_UINT64_INTERFACE_DEFINED__

#if !defined(____FIAsyncActionWithProgress_1_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncActionWithProgress_1_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncActionWithProgress_1_UINT64 __FIAsyncActionWithProgress_1_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncActionWithProgress_1_UINT64;

typedef struct __FIAsyncActionWithProgress_1_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncActionWithProgress_1_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncActionWithProgress_1_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncActionWithProgress_1_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncActionWithProgress_1_UINT64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncActionWithProgress_1_UINT64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncActionWithProgress_1_UINT64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncActionWithProgress_1_UINT64* This,
        __FIAsyncActionProgressHandler_1_UINT64* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncActionWithProgress_1_UINT64* This,
        __FIAsyncActionProgressHandler_1_UINT64** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncActionWithProgress_1_UINT64* This,
        __FIAsyncActionWithProgressCompletedHandler_1_UINT64* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncActionWithProgress_1_UINT64* This,
        __FIAsyncActionWithProgressCompletedHandler_1_UINT64** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncActionWithProgress_1_UINT64* This);

    END_INTERFACE
} __FIAsyncActionWithProgress_1_UINT64Vtbl;

interface __FIAsyncActionWithProgress_1_UINT64
{
    CONST_VTBL struct __FIAsyncActionWithProgress_1_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncActionWithProgress_1_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncActionWithProgress_1_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncActionWithProgress_1_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncActionWithProgress_1_UINT64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncActionWithProgress_1_UINT64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncActionWithProgress_1_UINT64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncActionWithProgress_1_UINT64_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncActionWithProgress_1_UINT64_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncActionWithProgress_1_UINT64_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncActionWithProgress_1_UINT64_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncActionWithProgress_1_UINT64_GetResults(This) \
    ((This)->lpVtbl->GetResults(This))

#endif /* COBJMACROS */

#endif // ____FIAsyncActionWithProgress_1_UINT64_INTERFACE_DEFINED__

#if !defined(____FIAsyncActionProgressHandler_1_UINT64_INTERFACE_DEFINED__)
#define ____FIAsyncActionProgressHandler_1_UINT64_INTERFACE_DEFINED__

typedef interface __FIAsyncActionProgressHandler_1_UINT64 __FIAsyncActionProgressHandler_1_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncActionProgressHandler_1_UINT64;

typedef struct __FIAsyncActionProgressHandler_1_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncActionProgressHandler_1_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncActionProgressHandler_1_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncActionProgressHandler_1_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncActionProgressHandler_1_UINT64* This,
        __FIAsyncActionWithProgress_1_UINT64* asyncInfo,
        UINT64 progressInfo);

    END_INTERFACE
} __FIAsyncActionProgressHandler_1_UINT64Vtbl;

interface __FIAsyncActionProgressHandler_1_UINT64
{
    CONST_VTBL struct __FIAsyncActionProgressHandler_1_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncActionProgressHandler_1_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncActionProgressHandler_1_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncActionProgressHandler_1_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncActionProgressHandler_1_UINT64_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncActionProgressHandler_1_UINT64_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult;

typedef struct __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResultVtbl;

interface __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult;

typedef struct __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognitionResult** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResultVtbl;

interface __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult;

typedef struct __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResultVtbl;

interface __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResultVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResultVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationProgressHandler_2_UINT32_UINT32 __FIAsyncOperationProgressHandler_2_UINT32_UINT32;

typedef interface __FIAsyncOperationWithProgress_2_UINT32_UINT32 __FIAsyncOperationWithProgress_2_UINT32_UINT32;

#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32 __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32* This,
        __FIAsyncOperationWithProgress_2_UINT32_UINT32* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32Vtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationWithProgress_2_UINT32_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_UINT32_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_UINT32_UINT32 __FIAsyncOperationWithProgress_2_UINT32_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_UINT32_UINT32;

typedef struct __FIAsyncOperationWithProgress_2_UINT32_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        __FIAsyncOperationProgressHandler_2_UINT32_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        __FIAsyncOperationProgressHandler_2_UINT32_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_UINT32_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_UINT32_UINT32* This,
        UINT32* result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_UINT32_UINT32Vtbl;

interface __FIAsyncOperationWithProgress_2_UINT32_UINT32
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_UINT32_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_UINT32_UINT32_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_UINT32_UINT32_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationProgressHandler_2_UINT32_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_UINT32_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_UINT32_UINT32 __FIAsyncOperationProgressHandler_2_UINT32_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_UINT32_UINT32;

typedef struct __FIAsyncOperationProgressHandler_2_UINT32_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_UINT32_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_UINT32_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_UINT32_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_UINT32_UINT32* This,
        __FIAsyncOperationWithProgress_2_UINT32_UINT32* asyncInfo,
        UINT32 progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_UINT32_UINT32Vtbl;

interface __FIAsyncOperationProgressHandler_2_UINT32_UINT32
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_UINT32_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_UINT32_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_UINT32_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_UINT32_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_UINT32_UINT32_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_UINT32_UINT32_INTERFACE_DEFINED__

#if !defined(____FIIterator_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1_HSTRING __FIIterator_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_HSTRING;

typedef struct __FIIterator_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_HSTRINGVtbl;

interface __FIIterator_1_HSTRING
{
    CONST_VTBL struct __FIIterator_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1_HSTRING __FIIterable_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_HSTRING;

typedef struct __FIIterable_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_HSTRING* This,
        __FIIterator_1_HSTRING** result);

    END_INTERFACE
} __FIIterable_1_HSTRINGVtbl;

interface __FIIterable_1_HSTRING
{
    CONST_VTBL struct __FIIterable_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_HSTRING_INTERFACE_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CPoint __x_ABI_CWindows_CFoundation_CPoint;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CFoundation__CPoint __FIIterator_1_Windows__CFoundation__CPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CFoundation__CPoint;

typedef struct __FIIterator_1_Windows__CFoundation__CPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CFoundation__CPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CFoundation__CPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CFoundation__CPoint* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CFoundation_CPoint* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CFoundation__CPointVtbl;

interface __FIIterator_1_Windows__CFoundation__CPoint
{
    CONST_VTBL struct __FIIterator_1_Windows__CFoundation__CPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CFoundation__CPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CFoundation__CPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CFoundation__CPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CFoundation__CPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CFoundation__CPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CFoundation__CPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CFoundation__CPoint_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CFoundation__CPoint_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CFoundation__CPoint_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CFoundation__CPoint_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CFoundation__CPoint __FIIterable_1_Windows__CFoundation__CPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CFoundation__CPoint;

typedef struct __FIIterable_1_Windows__CFoundation__CPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CFoundation__CPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CFoundation__CPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CFoundation__CPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CFoundation__CPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CFoundation__CPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CFoundation__CPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CFoundation__CPoint* This,
        __FIIterator_1_Windows__CFoundation__CPoint** result);

    END_INTERFACE
} __FIIterable_1_Windows__CFoundation__CPointVtbl;

interface __FIIterable_1_Windows__CFoundation__CPoint
{
    CONST_VTBL struct __FIIterable_1_Windows__CFoundation__CPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CFoundation__CPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CFoundation__CPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CFoundation__CPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CFoundation__CPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CFoundation__CPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CFoundation__CPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CFoundation__CPoint_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CFoundation__CPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint;

typedef struct __FIIterator_1_Windows__CUI__CInput__CInking__CInkPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CInput__CInking__CInkPointVtbl;

interface __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CInput__CInking__CInkPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint;

typedef struct __FIIterable_1_Windows__CUI__CInput__CInking__CInkPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        __FIIterator_1_Windows__CUI__CInput__CInking__CInkPoint** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CInput__CInking__CInkPointVtbl;

interface __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CInput__CInking__CInkPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer;

typedef struct __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizerVtbl;

interface __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer;

typedef struct __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        __FIIterator_1_Windows__CUI__CInput__CInking__CInkRecognizer** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizerVtbl;

interface __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CInput__CInking__CInkRecognizer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke;

typedef struct __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeVtbl;

interface __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke;

typedef struct __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        __FIIterator_1_Windows__CUI__CInput__CInking__CInkStroke** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeVtbl;

interface __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment;

typedef struct __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegmentVtbl;

interface __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment;

typedef struct __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        __FIIterator_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegmentVtbl;

interface __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_HSTRING __FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_HSTRING;

typedef struct __FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_HSTRINGVtbl;

interface __FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint;

typedef struct __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPointVtbl;

interface __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer;

typedef struct __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizerVtbl;

interface __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke;

typedef struct __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeVtbl;

interface __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment;

typedef struct __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegmentVtbl;

interface __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CDateTime __FIReference_1_Windows__CFoundation__CDateTime;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CDateTime;

typedef struct __FIReference_1_Windows__CFoundation__CDateTimeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CDateTime* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CDateTime* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CDateTimeVtbl;

interface __FIReference_1_Windows__CFoundation__CDateTime
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CDateTimeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CDateTime_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CDateTime_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CDateTime_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CDateTime_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CTimeSpan __FIReference_1_Windows__CFoundation__CTimeSpan;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CTimeSpan;

typedef struct __FIReference_1_Windows__CFoundation__CTimeSpanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CTimeSpan* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CTimeSpan* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CTimeSpan* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CTimeSpanVtbl;

interface __FIReference_1_Windows__CFoundation__CTimeSpan
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CTimeSpanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CTimeSpan_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CTimeSpan_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CTimeSpan_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* sender,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* sender,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs;

#endif // ____x_ABI_CWindows_CUI_CCore_CIPointerEventArgs_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput* sender,
        __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* sender,
        __x_ABI_CWindows_CUI_CCore_CIPointerEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix3x2 __x_ABI_CWindows_CFoundation_CNumerics_CMatrix3x2;

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIOutputStream __x_ABI_CWindows_CStorage_CStreams_CIOutputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIOutputStream_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

typedef enum __x_ABI_CWindows_CUI_CCore_CCoreInputDeviceTypes __x_ABI_CWindows_CUI_CCore_CCoreInputDeviceTypes;

#ifndef ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CInput_CIPointerPoint __x_ABI_CWindows_CUI_CInput_CIPointerPoint;

#endif // ____x_ABI_CWindows_CUI_CInput_CIPointerPoint_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CHandwritingLineHeight __x_ABI_CWindows_CUI_CInput_CInking_CHandwritingLineHeight;

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CInkDrawingAttributesKind __x_ABI_CWindows_CUI_CInput_CInking_CInkDrawingAttributesKind;

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CInkHighContrastAdjustment __x_ABI_CWindows_CUI_CInput_CInking_CInkHighContrastAdjustment;

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CInkInputProcessingMode __x_ABI_CWindows_CUI_CInput_CInking_CInkInputProcessingMode;

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CInkInputRightDragAction __x_ABI_CWindows_CUI_CInput_CInking_CInkInputRightDragAction;

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CInkManipulationMode __x_ABI_CWindows_CUI_CInput_CInking_CInkManipulationMode;

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CInkPersistenceFormat __x_ABI_CWindows_CUI_CInput_CInking_CInkPersistenceFormat;

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CInkPresenterPredefinedConfiguration __x_ABI_CWindows_CUI_CInput_CInking_CInkPresenterPredefinedConfiguration;

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CInkPresenterStencilKind __x_ABI_CWindows_CUI_CInput_CInking_CInkPresenterStencilKind;

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CInkRecognitionTarget __x_ABI_CWindows_CUI_CInput_CInking_CInkRecognitionTarget;

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CPenHandedness __x_ABI_CWindows_CUI_CInput_CInking_CPenHandedness;

typedef enum __x_ABI_CWindows_CUI_CInput_CInking_CPenTipShape __x_ABI_CWindows_CUI_CInput_CInking_CPenTipShape;

/*
 *
 * Struct Windows.UI.Input.Inking.HandwritingLineHeight
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CUI_CInput_CInking_CHandwritingLineHeight
{
    HandwritingLineHeight_Small = 0,
    HandwritingLineHeight_Medium = 1,
    HandwritingLineHeight_Large = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.UI.Input.Inking.InkDrawingAttributesKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CInput_CInking_CInkDrawingAttributesKind
{
    InkDrawingAttributesKind_Default = 0,
    InkDrawingAttributesKind_Pencil = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Inking.InkHighContrastAdjustment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CUI_CInput_CInking_CInkHighContrastAdjustment
{
    InkHighContrastAdjustment_UseSystemColorsWhenNecessary = 0,
    InkHighContrastAdjustment_UseSystemColors = 1,
    InkHighContrastAdjustment_UseOriginalColors = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.Input.Inking.InkInputProcessingMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CInking_CInkInputProcessingMode
{
    InkInputProcessingMode_None = 0,
    InkInputProcessingMode_Inking = 1,
    InkInputProcessingMode_Erasing = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Inking.InkInputRightDragAction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CInking_CInkInputRightDragAction
{
    InkInputRightDragAction_LeaveUnprocessed = 0,
    InkInputRightDragAction_AllowProcessing = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Inking.InkManipulationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CInking_CInkManipulationMode
{
    InkManipulationMode_Inking = 0,
    InkManipulationMode_Erasing = 1,
    InkManipulationMode_Selecting = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Inking.InkPersistenceFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CUI_CInput_CInking_CInkPersistenceFormat
{
    InkPersistenceFormat_GifWithEmbeddedIsf = 0,
    InkPersistenceFormat_Isf = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.Input.Inking.InkPresenterPredefinedConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CInking_CInkPresenterPredefinedConfiguration
{
    InkPresenterPredefinedConfiguration_SimpleSinglePointer = 0,
    InkPresenterPredefinedConfiguration_SimpleMultiplePointer = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Inking.InkPresenterStencilKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CInput_CInking_CInkPresenterStencilKind
{
    InkPresenterStencilKind_Other = 0,
    InkPresenterStencilKind_Ruler = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    InkPresenterStencilKind_Protractor = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Input.Inking.InkRecognitionTarget
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CInking_CInkRecognitionTarget
{
    InkRecognitionTarget_All = 0,
    InkRecognitionTarget_Selected = 1,
    InkRecognitionTarget_Recent = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Input.Inking.PenHandedness
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CUI_CInput_CInking_CPenHandedness
{
    PenHandedness_Right = 0,
    PenHandedness_Left = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.UI.Input.Inking.PenTipShape
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CInput_CInking_CPenTipShape
{
    PenTipShape_Circle = 0,
    PenTipShape_Rectangle = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkDrawingAttributes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkDrawingAttributes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkDrawingAttributes[] = L"Windows.UI.Input.Inking.IInkDrawingAttributes";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Color)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_Color)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* get_PenTip)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CPenTipShape* value);
    HRESULT (STDMETHODCALLTYPE* put_PenTip)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CPenTipShape value);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* This,
        struct __x_ABI_CWindows_CFoundation_CSize* value);
    HRESULT (STDMETHODCALLTYPE* put_Size)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* This,
        struct __x_ABI_CWindows_CFoundation_CSize value);
    HRESULT (STDMETHODCALLTYPE* get_IgnorePressure)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IgnorePressure)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_FitToCurve)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_FitToCurve)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_get_Color(This, value) \
    ((This)->lpVtbl->get_Color(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_put_Color(This, value) \
    ((This)->lpVtbl->put_Color(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_get_PenTip(This, value) \
    ((This)->lpVtbl->get_PenTip(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_put_PenTip(This, value) \
    ((This)->lpVtbl->put_PenTip(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_get_Size(This, value) \
    ((This)->lpVtbl->get_Size(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_put_Size(This, value) \
    ((This)->lpVtbl->put_Size(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_get_IgnorePressure(This, value) \
    ((This)->lpVtbl->get_IgnorePressure(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_put_IgnorePressure(This, value) \
    ((This)->lpVtbl->put_IgnorePressure(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_get_FitToCurve(This, value) \
    ((This)->lpVtbl->get_FitToCurve(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_put_FitToCurve(This, value) \
    ((This)->lpVtbl->put_FitToCurve(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkDrawingAttributes2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkDrawingAttributes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkDrawingAttributes2[] = L"Windows.UI.Input.Inking.IInkDrawingAttributes2";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PenTipTransform)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix3x2* value);
    HRESULT (STDMETHODCALLTYPE* put_PenTipTransform)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix3x2 value);
    HRESULT (STDMETHODCALLTYPE* get_DrawAsHighlighter)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_DrawAsHighlighter)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_get_PenTipTransform(This, value) \
    ((This)->lpVtbl->get_PenTipTransform(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_put_PenTipTransform(This, value) \
    ((This)->lpVtbl->put_PenTipTransform(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_get_DrawAsHighlighter(This, value) \
    ((This)->lpVtbl->get_DrawAsHighlighter(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_put_DrawAsHighlighter(This, value) \
    ((This)->lpVtbl->put_DrawAsHighlighter(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkDrawingAttributes3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkDrawingAttributes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkDrawingAttributes3[] = L"Windows.UI.Input.Inking.IInkDrawingAttributes3";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CInkDrawingAttributesKind* value);
    HRESULT (STDMETHODCALLTYPE* get_PencilProperties)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_get_PencilProperties(This, value) \
    ((This)->lpVtbl->get_PencilProperties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkDrawingAttributes4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkDrawingAttributes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkDrawingAttributes4[] = L"Windows.UI.Input.Inking.IInkDrawingAttributes4";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IgnoreTilt)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IgnoreTilt)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_get_IgnoreTilt(This, value) \
    ((This)->lpVtbl->get_IgnoreTilt(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_put_IgnoreTilt(This, value) \
    ((This)->lpVtbl->put_IgnoreTilt(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkDrawingAttributes5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkDrawingAttributes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkDrawingAttributes5[] = L"Windows.UI.Input.Inking.IInkDrawingAttributes5";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ModelerAttributes)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_get_ModelerAttributes(This, value) \
    ((This)->lpVtbl->get_ModelerAttributes(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkDrawingAttributesPencilProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkDrawingAttributesPencilProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkDrawingAttributesPencilProperties[] = L"Windows.UI.Input.Inking.IInkDrawingAttributesPencilProperties";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Opacity)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Opacity)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties* This,
        DOUBLE value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilPropertiesVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_get_Opacity(This, value) \
    ((This)->lpVtbl->get_Opacity(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_put_Opacity(This, value) \
    ((This)->lpVtbl->put_Opacity(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesPencilProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkDrawingAttributesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkDrawingAttributes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkDrawingAttributesStatics[] = L"Windows.UI.Input.Inking.IInkDrawingAttributesStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateForPencil)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_CreateForPencil(This, result) \
    ((This)->lpVtbl->CreateForPencil(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkInputConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkInputConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkInputConfiguration[] = L"Windows.UI.Input.Inking.IInkInputConfiguration";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsPrimaryBarrelButtonInputEnabled)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsPrimaryBarrelButtonInputEnabled)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsEraserInputEnabled)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsEraserInputEnabled)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfigurationVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_get_IsPrimaryBarrelButtonInputEnabled(This, value) \
    ((This)->lpVtbl->get_IsPrimaryBarrelButtonInputEnabled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_put_IsPrimaryBarrelButtonInputEnabled(This, value) \
    ((This)->lpVtbl->put_IsPrimaryBarrelButtonInputEnabled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_get_IsEraserInputEnabled(This, value) \
    ((This)->lpVtbl->get_IsEraserInputEnabled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_put_IsEraserInputEnabled(This, value) \
    ((This)->lpVtbl->put_IsEraserInputEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkInputConfiguration2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkInputConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkInputConfiguration2[] = L"Windows.UI.Input.Inking.IInkInputConfiguration2";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsPenHapticFeedbackEnabled)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsPenHapticFeedbackEnabled)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_get_IsPenHapticFeedbackEnabled(This, value) \
    ((This)->lpVtbl->get_IsPenHapticFeedbackEnabled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_put_IsPenHapticFeedbackEnabled(This, value) \
    ((This)->lpVtbl->put_IsPenHapticFeedbackEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkInputProcessingConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkInputProcessingConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkInputProcessingConfiguration[] = L"Windows.UI.Input.Inking.IInkInputProcessingConfiguration";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CInkInputProcessingMode* value);
    HRESULT (STDMETHODCALLTYPE* put_Mode)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CInkInputProcessingMode value);
    HRESULT (STDMETHODCALLTYPE* get_RightDragAction)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CInkInputRightDragAction* value);
    HRESULT (STDMETHODCALLTYPE* put_RightDragAction)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CInkInputRightDragAction value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfigurationVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_put_Mode(This, value) \
    ((This)->lpVtbl->put_Mode(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_get_RightDragAction(This, value) \
    ((This)->lpVtbl->get_RightDragAction(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_put_RightDragAction(This, value) \
    ((This)->lpVtbl->put_RightDragAction(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.IInkStrokeContainer
 *     Windows.UI.Input.Inking.IInkRecognizerContainer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkManager[] = L"Windows.UI.Input.Inking.IInkManager";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Mode)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkManager* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CInkManipulationMode* value);
    HRESULT (STDMETHODCALLTYPE* put_Mode)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkManager* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CInkManipulationMode value);
    HRESULT (STDMETHODCALLTYPE* ProcessPointerDown)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkManager* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* pointerPoint);
    HRESULT (STDMETHODCALLTYPE* ProcessPointerUpdate)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkManager* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* pointerPoint,
        IInspectable** updateInformation);
    HRESULT (STDMETHODCALLTYPE* ProcessPointerUp)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkManager* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* pointerPoint,
        struct __x_ABI_CWindows_CFoundation_CRect* updateRectangle);
    HRESULT (STDMETHODCALLTYPE* SetDefaultDrawingAttributes)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkManager* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* drawingAttributes);
    HRESULT (STDMETHODCALLTYPE* RecognizeAsync2)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkManager* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CInkRecognitionTarget recognitionTarget,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult** recognitionResults);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkManagerVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_get_Mode(This, value) \
    ((This)->lpVtbl->get_Mode(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_put_Mode(This, value) \
    ((This)->lpVtbl->put_Mode(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_ProcessPointerDown(This, pointerPoint) \
    ((This)->lpVtbl->ProcessPointerDown(This, pointerPoint))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_ProcessPointerUpdate(This, pointerPoint, updateInformation) \
    ((This)->lpVtbl->ProcessPointerUpdate(This, pointerPoint, updateInformation))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_ProcessPointerUp(This, pointerPoint, updateRectangle) \
    ((This)->lpVtbl->ProcessPointerUp(This, pointerPoint, updateRectangle))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_SetDefaultDrawingAttributes(This, drawingAttributes) \
    ((This)->lpVtbl->SetDefaultDrawingAttributes(This, drawingAttributes))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_RecognizeAsync2(This, recognitionTarget, recognitionResults) \
    ((This)->lpVtbl->RecognizeAsync2(This, recognitionTarget, recognitionResults))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkModelerAttributes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkModelerAttributes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkModelerAttributes[] = L"Windows.UI.Input.Inking.IInkModelerAttributes";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PredictionTime)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_PredictionTime)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_ScalingFactor)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* put_ScalingFactor)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes* This,
        FLOAT value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributesVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_get_PredictionTime(This, value) \
    ((This)->lpVtbl->get_PredictionTime(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_put_PredictionTime(This, value) \
    ((This)->lpVtbl->put_PredictionTime(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_get_ScalingFactor(This, value) \
    ((This)->lpVtbl->get_ScalingFactor(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_put_ScalingFactor(This, value) \
    ((This)->lpVtbl->put_ScalingFactor(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkModelerAttributes2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkModelerAttributes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkModelerAttributes2[] = L"Windows.UI.Input.Inking.IInkModelerAttributes2";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UseVelocityBasedPressure)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_UseVelocityBasedPressure)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_get_UseVelocityBasedPressure(This, value) \
    ((This)->lpVtbl->get_UseVelocityBasedPressure(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_put_UseVelocityBasedPressure(This, value) \
    ((This)->lpVtbl->put_UseVelocityBasedPressure(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkModelerAttributes2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPoint[] = L"Windows.UI.Input.Inking.IInkPoint";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_Pressure)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint* This,
        FLOAT* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_get_Pressure(This, value) \
    ((This)->lpVtbl->get_Pressure(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPoint2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPoint2[] = L"Windows.UI.Input.Inking.IInkPoint2";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TiltX)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_TiltY)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2* This,
        UINT64* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_get_TiltX(This, value) \
    ((This)->lpVtbl->get_TiltX(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_get_TiltY(This, value) \
    ((This)->lpVtbl->get_TiltY(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPointFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPointFactory[] = L"Windows.UI.Input.Inking.IInkPointFactory";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInkPoint)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory* This,
        struct __x_ABI_CWindows_CFoundation_CPoint position,
        FLOAT pressure,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactoryVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_CreateInkPoint(This, position, pressure, result) \
    ((This)->lpVtbl->CreateInkPoint(This, position, pressure, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPointFactory2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPoint
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPointFactory2[] = L"Windows.UI.Input.Inking.IInkPointFactory2";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInkPointWithTiltAndTimestamp)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2* This,
        struct __x_ABI_CWindows_CFoundation_CPoint position,
        FLOAT pressure,
        FLOAT tiltX,
        FLOAT tiltY,
        UINT64 timestamp,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPoint** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_CreateInkPointWithTiltAndTimestamp(This, position, pressure, tiltX, tiltY, timestamp, result) \
    ((This)->lpVtbl->CreateInkPointWithTiltAndTimestamp(This, position, pressure, tiltX, tiltY, timestamp, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPointFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPresenter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenter[] = L"Windows.UI.Input.Inking.IInkPresenter";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsInputEnabled)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsInputEnabled)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_InputDeviceTypes)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreInputDeviceTypes* value);
    HRESULT (STDMETHODCALLTYPE* put_InputDeviceTypes)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        enum __x_ABI_CWindows_CUI_CCore_CCoreInputDeviceTypes value);
    HRESULT (STDMETHODCALLTYPE* get_UnprocessedInput)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput** value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeInput)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput** value);
    HRESULT (STDMETHODCALLTYPE* get_InputProcessingConfiguration)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputProcessingConfiguration** value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeContainer)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer** value);
    HRESULT (STDMETHODCALLTYPE* put_StrokeContainer)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* value);
    HRESULT (STDMETHODCALLTYPE* CopyDefaultDrawingAttributes)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes** value);
    HRESULT (STDMETHODCALLTYPE* UpdateDefaultDrawingAttributes)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* value);
    HRESULT (STDMETHODCALLTYPE* ActivateCustomDrying)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer** inkSynchronizer);
    HRESULT (STDMETHODCALLTYPE* SetPredefinedConfiguration)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CInkPresenterPredefinedConfiguration value);
    HRESULT (STDMETHODCALLTYPE* add_StrokesCollected)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesCollectedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_StrokesCollected)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_StrokesErased)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkPresenter_Windows__CUI__CInput__CInking__CInkStrokesErasedEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_StrokesErased)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_get_IsInputEnabled(This, value) \
    ((This)->lpVtbl->get_IsInputEnabled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_put_IsInputEnabled(This, value) \
    ((This)->lpVtbl->put_IsInputEnabled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_get_InputDeviceTypes(This, value) \
    ((This)->lpVtbl->get_InputDeviceTypes(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_put_InputDeviceTypes(This, value) \
    ((This)->lpVtbl->put_InputDeviceTypes(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_get_UnprocessedInput(This, value) \
    ((This)->lpVtbl->get_UnprocessedInput(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_get_StrokeInput(This, value) \
    ((This)->lpVtbl->get_StrokeInput(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_get_InputProcessingConfiguration(This, value) \
    ((This)->lpVtbl->get_InputProcessingConfiguration(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_get_StrokeContainer(This, value) \
    ((This)->lpVtbl->get_StrokeContainer(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_put_StrokeContainer(This, value) \
    ((This)->lpVtbl->put_StrokeContainer(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_CopyDefaultDrawingAttributes(This, value) \
    ((This)->lpVtbl->CopyDefaultDrawingAttributes(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_UpdateDefaultDrawingAttributes(This, value) \
    ((This)->lpVtbl->UpdateDefaultDrawingAttributes(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_ActivateCustomDrying(This, inkSynchronizer) \
    ((This)->lpVtbl->ActivateCustomDrying(This, inkSynchronizer))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_SetPredefinedConfiguration(This, value) \
    ((This)->lpVtbl->SetPredefinedConfiguration(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_add_StrokesCollected(This, handler, cookie) \
    ((This)->lpVtbl->add_StrokesCollected(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_remove_StrokesCollected(This, cookie) \
    ((This)->lpVtbl->remove_StrokesCollected(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_add_StrokesErased(This, handler, cookie) \
    ((This)->lpVtbl->add_StrokesErased(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_remove_StrokesErased(This, cookie) \
    ((This)->lpVtbl->remove_StrokesErased(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPresenter
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.IInkPresenter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenter2[] = L"Windows.UI.Input.Inking.IInkPresenter2";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HighContrastAdjustment)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CInkHighContrastAdjustment* value);
    HRESULT (STDMETHODCALLTYPE* put_HighContrastAdjustment)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CInkHighContrastAdjustment value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_get_HighContrastAdjustment(This, value) \
    ((This)->lpVtbl->get_HighContrastAdjustment(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_put_HighContrastAdjustment(This, value) \
    ((This)->lpVtbl->put_HighContrastAdjustment(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenter3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPresenter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenter3[] = L"Windows.UI.Input.Inking.IInkPresenter3";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InputConfiguration)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkInputConfiguration** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_get_InputConfiguration(This, value) \
    ((This)->lpVtbl->get_InputConfiguration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenterProtractor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPresenterProtractor
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.IInkPresenterStencil
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenterProtractor[] = L"Windows.UI.Input.Inking.IInkPresenterProtractor";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AreTickMarksVisible)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AreTickMarksVisible)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AreRaysVisible)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AreRaysVisible)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsCenterMarkerVisible)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsCenterMarkerVisible)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsAngleReadoutVisible)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsAngleReadoutVisible)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsResizable)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsResizable)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Radius)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Radius)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_AccentColor)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_AccentColor)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor* This,
        struct __x_ABI_CWindows_CUI_CColor value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_get_AreTickMarksVisible(This, value) \
    ((This)->lpVtbl->get_AreTickMarksVisible(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_put_AreTickMarksVisible(This, value) \
    ((This)->lpVtbl->put_AreTickMarksVisible(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_get_AreRaysVisible(This, value) \
    ((This)->lpVtbl->get_AreRaysVisible(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_put_AreRaysVisible(This, value) \
    ((This)->lpVtbl->put_AreRaysVisible(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_get_IsCenterMarkerVisible(This, value) \
    ((This)->lpVtbl->get_IsCenterMarkerVisible(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_put_IsCenterMarkerVisible(This, value) \
    ((This)->lpVtbl->put_IsCenterMarkerVisible(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_get_IsAngleReadoutVisible(This, value) \
    ((This)->lpVtbl->get_IsAngleReadoutVisible(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_put_IsAngleReadoutVisible(This, value) \
    ((This)->lpVtbl->put_IsAngleReadoutVisible(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_get_IsResizable(This, value) \
    ((This)->lpVtbl->get_IsResizable(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_put_IsResizable(This, value) \
    ((This)->lpVtbl->put_IsResizable(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_get_Radius(This, value) \
    ((This)->lpVtbl->get_Radius(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_put_Radius(This, value) \
    ((This)->lpVtbl->put_Radius(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_get_AccentColor(This, value) \
    ((This)->lpVtbl->get_AccentColor(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_put_AccentColor(This, value) \
    ((This)->lpVtbl->put_AccentColor(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenterProtractorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPresenterProtractor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenterProtractorFactory[] = L"Windows.UI.Input.Inking.IInkPresenterProtractorFactory";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* inkPresenter,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractor** inkPresenterProtractor);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactoryVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_Create(This, inkPresenter, inkPresenterProtractor) \
    ((This)->lpVtbl->Create(This, inkPresenter, inkPresenterProtractor))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterProtractorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenterRuler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPresenterRuler
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.UI.Input.Inking.IInkPresenterStencil
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenterRuler[] = L"Windows.UI.Input.Inking.IInkPresenterRuler";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Length)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Length)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_Width)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Width)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler* This,
        DOUBLE value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_get_Length(This, value) \
    ((This)->lpVtbl->get_Length(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_put_Length(This, value) \
    ((This)->lpVtbl->put_Length(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_get_Width(This, value) \
    ((This)->lpVtbl->get_Width(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_put_Width(This, value) \
    ((This)->lpVtbl->put_Width(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenterRuler2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkPresenterRuler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenterRuler2[] = L"Windows.UI.Input.Inking.IInkPresenterRuler2";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AreTickMarksVisible)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AreTickMarksVisible)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsCompassVisible)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsCompassVisible)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_get_AreTickMarksVisible(This, value) \
    ((This)->lpVtbl->get_AreTickMarksVisible(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_put_AreTickMarksVisible(This, value) \
    ((This)->lpVtbl->put_AreTickMarksVisible(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_get_IsCompassVisible(This, value) \
    ((This)->lpVtbl->get_IsCompassVisible(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_put_IsCompassVisible(This, value) \
    ((This)->lpVtbl->put_IsCompassVisible(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenterRulerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenterRulerFactory[] = L"Windows.UI.Input.Inking.IInkPresenterRulerFactory";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter* inkPresenter,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRuler** inkPresenterRuler);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactoryVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_Create(This, inkPresenter, inkPresenterRuler) \
    ((This)->lpVtbl->Create(This, inkPresenter, inkPresenterRuler))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterRulerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkPresenterStencil
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkPresenterStencil[] = L"Windows.UI.Input.Inking.IInkPresenterStencil";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencilVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CInkPresenterStencilKind* value);
    HRESULT (STDMETHODCALLTYPE* get_IsVisible)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsVisible)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_BackgroundColor)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_BackgroundColor)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* get_ForegroundColor)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* put_ForegroundColor)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* get_Transform)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix3x2* value);
    HRESULT (STDMETHODCALLTYPE* put_Transform)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix3x2 value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencilVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencilVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_get_IsVisible(This, value) \
    ((This)->lpVtbl->get_IsVisible(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_put_IsVisible(This, value) \
    ((This)->lpVtbl->put_IsVisible(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_get_BackgroundColor(This, value) \
    ((This)->lpVtbl->get_BackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_put_BackgroundColor(This, value) \
    ((This)->lpVtbl->put_BackgroundColor(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_get_ForegroundColor(This, value) \
    ((This)->lpVtbl->get_ForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_put_ForegroundColor(This, value) \
    ((This)->lpVtbl->put_ForegroundColor(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_get_Transform(This, value) \
    ((This)->lpVtbl->get_Transform(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_put_Transform(This, value) \
    ((This)->lpVtbl->put_Transform(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenterStencil_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkRecognitionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkRecognitionResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkRecognitionResult[] = L"Windows.UI.Input.Inking.IInkRecognitionResult";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BoundingRect)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult* This,
        struct __x_ABI_CWindows_CFoundation_CRect* boundingRect);
    HRESULT (STDMETHODCALLTYPE* GetTextCandidates)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult* This,
        __FIVectorView_1_HSTRING** textCandidates);
    HRESULT (STDMETHODCALLTYPE* GetStrokes)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult* This,
        __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke** strokes);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResultVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_get_BoundingRect(This, boundingRect) \
    ((This)->lpVtbl->get_BoundingRect(This, boundingRect))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_GetTextCandidates(This, textCandidates) \
    ((This)->lpVtbl->GetTextCandidates(This, textCandidates))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_GetStrokes(This, strokes) \
    ((This)->lpVtbl->GetStrokes(This, strokes))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognitionResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkRecognizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkRecognizer[] = L"Windows.UI.Input.Inking.IInkRecognizer";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkRecognizerContainer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkRecognizerContainer[] = L"Windows.UI.Input.Inking.IInkRecognizerContainer";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetDefaultRecognizer)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizer* recognizer);
    HRESULT (STDMETHODCALLTYPE* RecognizeAsync)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* strokeCollection,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CInkRecognitionTarget recognitionTarget,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult** recognitionResults);
    HRESULT (STDMETHODCALLTYPE* GetRecognizers)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer* This,
        __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognizer** recognizerView);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainerVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_SetDefaultRecognizer(This, recognizer) \
    ((This)->lpVtbl->SetDefaultRecognizer(This, recognizer))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_RecognizeAsync(This, strokeCollection, recognitionTarget, recognitionResults) \
    ((This)->lpVtbl->RecognizeAsync(This, strokeCollection, recognitionTarget, recognitionResults))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_GetRecognizers(This, recognizerView) \
    ((This)->lpVtbl->GetRecognizers(This, recognizerView))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkRecognizerContainer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStroke
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStroke
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStroke[] = L"Windows.UI.Input.Inking.IInkStroke";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DrawingAttributes)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes** value);
    HRESULT (STDMETHODCALLTYPE* put_DrawingAttributes)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* value);
    HRESULT (STDMETHODCALLTYPE* get_BoundingRect)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* get_Selected)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Selected)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Recognized)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetRenderingSegments)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* This,
        __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStrokeRenderingSegment** renderingSegments);
    HRESULT (STDMETHODCALLTYPE* Clone)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke** clonedStroke);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_get_DrawingAttributes(This, value) \
    ((This)->lpVtbl->get_DrawingAttributes(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_put_DrawingAttributes(This, value) \
    ((This)->lpVtbl->put_DrawingAttributes(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_get_BoundingRect(This, value) \
    ((This)->lpVtbl->get_BoundingRect(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_get_Selected(This, value) \
    ((This)->lpVtbl->get_Selected(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_put_Selected(This, value) \
    ((This)->lpVtbl->put_Selected(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_get_Recognized(This, value) \
    ((This)->lpVtbl->get_Recognized(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_GetRenderingSegments(This, renderingSegments) \
    ((This)->lpVtbl->GetRenderingSegments(This, renderingSegments))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_Clone(This, clonedStroke) \
    ((This)->lpVtbl->Clone(This, clonedStroke))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStroke2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStroke
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStroke2[] = L"Windows.UI.Input.Inking.IInkStroke2";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PointTransform)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix3x2* value);
    HRESULT (STDMETHODCALLTYPE* put_PointTransform)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix3x2 value);
    HRESULT (STDMETHODCALLTYPE* GetInkPoints)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2* This,
        __FIVectorView_1_Windows__CUI__CInput__CInking__CInkPoint** inkPoints);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_get_PointTransform(This, value) \
    ((This)->lpVtbl->get_PointTransform(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_put_PointTransform(This, value) \
    ((This)->lpVtbl->put_PointTransform(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_GetInkPoints(This, inkPoints) \
    ((This)->lpVtbl->GetInkPoints(This, inkPoints))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStroke3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStroke
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStroke3[] = L"Windows.UI.Input.Inking.IInkStroke3";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeStartedTime)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_StrokeStartedTime)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_StrokeDuration)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* put_StrokeDuration)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_get_StrokeStartedTime(This, value) \
    ((This)->lpVtbl->get_StrokeStartedTime(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_put_StrokeStartedTime(This, value) \
    ((This)->lpVtbl->put_StrokeStartedTime(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_get_StrokeDuration(This, value) \
    ((This)->lpVtbl->get_StrokeDuration(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_put_StrokeDuration(This, value) \
    ((This)->lpVtbl->put_StrokeDuration(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStroke4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStroke
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStroke4[] = L"Windows.UI.Input.Inking.IInkStroke4";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PointerId)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_get_PointerId(This, value) \
    ((This)->lpVtbl->get_PointerId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokeBuilder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokeBuilder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokeBuilder[] = L"Windows.UI.Input.Inking.IInkStrokeBuilder";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* BeginStroke)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* pointerPoint);
    HRESULT (STDMETHODCALLTYPE* AppendToStroke)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* pointerPoint,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint** previousPointerPoint);
    HRESULT (STDMETHODCALLTYPE* EndStroke)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder* This,
        __x_ABI_CWindows_CUI_CInput_CIPointerPoint* pointerPoint,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke** stroke);
    HRESULT (STDMETHODCALLTYPE* CreateStroke)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder* This,
        __FIIterable_1_Windows__CFoundation__CPoint* points,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke** stroke);
    HRESULT (STDMETHODCALLTYPE* SetDefaultDrawingAttributes)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkDrawingAttributes* drawingAttributes);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilderVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_BeginStroke(This, pointerPoint) \
    ((This)->lpVtbl->BeginStroke(This, pointerPoint))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_AppendToStroke(This, pointerPoint, previousPointerPoint) \
    ((This)->lpVtbl->AppendToStroke(This, pointerPoint, previousPointerPoint))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_EndStroke(This, pointerPoint, stroke) \
    ((This)->lpVtbl->EndStroke(This, pointerPoint, stroke))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_CreateStroke(This, points, stroke) \
    ((This)->lpVtbl->CreateStroke(This, points, stroke))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_SetDefaultDrawingAttributes(This, drawingAttributes) \
    ((This)->lpVtbl->SetDefaultDrawingAttributes(This, drawingAttributes))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokeBuilder2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokeBuilder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokeBuilder2[] = L"Windows.UI.Input.Inking.IInkStrokeBuilder2";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateStrokeFromInkPoints)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2* This,
        __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* inkPoints,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix3x2 transform,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_CreateStrokeFromInkPoints(This, inkPoints, transform, result) \
    ((This)->lpVtbl->CreateStrokeFromInkPoints(This, inkPoints, transform, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokeBuilder3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokeBuilder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokeBuilder3[] = L"Windows.UI.Input.Inking.IInkStrokeBuilder3";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateStrokeFromInkPoints)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3* This,
        __FIIterable_1_Windows__CUI__CInput__CInking__CInkPoint* inkPoints,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CMatrix3x2 transform,
        __FIReference_1_Windows__CFoundation__CDateTime* strokeStartedTime,
        __FIReference_1_Windows__CFoundation__CTimeSpan* strokeDuration,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_CreateStrokeFromInkPoints(This, inkPoints, transform, strokeStartedTime, strokeDuration, result) \
    ((This)->lpVtbl->CreateStrokeFromInkPoints(This, inkPoints, transform, strokeStartedTime, strokeDuration, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeBuilder3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokeContainer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokeContainer[] = L"Windows.UI.Input.Inking.IInkStrokeContainer";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BoundingRect)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        struct __x_ABI_CWindows_CFoundation_CRect* value);
    HRESULT (STDMETHODCALLTYPE* AddStroke)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke* stroke);
    HRESULT (STDMETHODCALLTYPE* DeleteSelected)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        struct __x_ABI_CWindows_CFoundation_CRect* invalidatedRect);
    HRESULT (STDMETHODCALLTYPE* MoveSelected)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        struct __x_ABI_CWindows_CFoundation_CPoint translation,
        struct __x_ABI_CWindows_CFoundation_CRect* invalidatedRectangle);
    HRESULT (STDMETHODCALLTYPE* SelectWithPolyLine)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        __FIIterable_1_Windows__CFoundation__CPoint* polyline,
        struct __x_ABI_CWindows_CFoundation_CRect* invalidatedRectangle);
    HRESULT (STDMETHODCALLTYPE* SelectWithLine)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        struct __x_ABI_CWindows_CFoundation_CPoint from,
        struct __x_ABI_CWindows_CFoundation_CPoint to,
        struct __x_ABI_CWindows_CFoundation_CRect* invalidatedRectangle);
    HRESULT (STDMETHODCALLTYPE* CopySelectedToClipboard)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This);
    HRESULT (STDMETHODCALLTYPE* PasteFromClipboard)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        struct __x_ABI_CWindows_CFoundation_CPoint position,
        struct __x_ABI_CWindows_CFoundation_CRect* invalidatedRectangle);
    HRESULT (STDMETHODCALLTYPE* CanPasteFromClipboard)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        boolean* canPaste);
    HRESULT (STDMETHODCALLTYPE* LoadAsync)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* inputStream,
        __FIAsyncActionWithProgress_1_UINT64** loadAction);
    HRESULT (STDMETHODCALLTYPE* SaveAsync)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream* outputStream,
        __FIAsyncOperationWithProgress_2_UINT32_UINT32** outputStreamOperation);
    HRESULT (STDMETHODCALLTYPE* UpdateRecognitionResults)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult* recognitionResults);
    HRESULT (STDMETHODCALLTYPE* GetStrokes)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke** strokeView);
    HRESULT (STDMETHODCALLTYPE* GetRecognitionResults)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer* This,
        __FIVectorView_1_Windows__CUI__CInput__CInking__CInkRecognitionResult** recognitionResults);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainerVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_get_BoundingRect(This, value) \
    ((This)->lpVtbl->get_BoundingRect(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_AddStroke(This, stroke) \
    ((This)->lpVtbl->AddStroke(This, stroke))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_DeleteSelected(This, invalidatedRect) \
    ((This)->lpVtbl->DeleteSelected(This, invalidatedRect))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_MoveSelected(This, translation, invalidatedRectangle) \
    ((This)->lpVtbl->MoveSelected(This, translation, invalidatedRectangle))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_SelectWithPolyLine(This, polyline, invalidatedRectangle) \
    ((This)->lpVtbl->SelectWithPolyLine(This, polyline, invalidatedRectangle))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_SelectWithLine(This, from, to, invalidatedRectangle) \
    ((This)->lpVtbl->SelectWithLine(This, from, to, invalidatedRectangle))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_CopySelectedToClipboard(This) \
    ((This)->lpVtbl->CopySelectedToClipboard(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_PasteFromClipboard(This, position, invalidatedRectangle) \
    ((This)->lpVtbl->PasteFromClipboard(This, position, invalidatedRectangle))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_CanPasteFromClipboard(This, canPaste) \
    ((This)->lpVtbl->CanPasteFromClipboard(This, canPaste))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_LoadAsync(This, inputStream, loadAction) \
    ((This)->lpVtbl->LoadAsync(This, inputStream, loadAction))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_SaveAsync(This, outputStream, outputStreamOperation) \
    ((This)->lpVtbl->SaveAsync(This, outputStream, outputStreamOperation))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_UpdateRecognitionResults(This, recognitionResults) \
    ((This)->lpVtbl->UpdateRecognitionResults(This, recognitionResults))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_GetStrokes(This, strokeView) \
    ((This)->lpVtbl->GetStrokes(This, strokeView))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_GetRecognitionResults(This, recognitionResults) \
    ((This)->lpVtbl->GetRecognitionResults(This, recognitionResults))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokeContainer2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokeContainer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokeContainer2[] = L"Windows.UI.Input.Inking.IInkStrokeContainer2";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddStrokes)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2* This,
        __FIIterable_1_Windows__CUI__CInput__CInking__CInkStroke* strokes);
    HRESULT (STDMETHODCALLTYPE* Clear)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_AddStrokes(This, strokes) \
    ((This)->lpVtbl->AddStrokes(This, strokes))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokeContainer3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokeContainer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokeContainer3[] = L"Windows.UI.Input.Inking.IInkStrokeContainer3";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SaveWithFormatAsync)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3* This,
        __x_ABI_CWindows_CStorage_CStreams_CIOutputStream* outputStream,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CInkPersistenceFormat inkPersistenceFormat,
        __FIAsyncOperationWithProgress_2_UINT32_UINT32** outputStreamOperation);
    HRESULT (STDMETHODCALLTYPE* GetStrokeById)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3* This,
        UINT32 id,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkStroke** stroke);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_SaveWithFormatAsync(This, outputStream, inkPersistenceFormat, outputStreamOperation) \
    ((This)->lpVtbl->SaveWithFormatAsync(This, outputStream, inkPersistenceFormat, outputStreamOperation))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_GetStrokeById(This, id, stroke) \
    ((This)->lpVtbl->GetStrokeById(This, id, stroke))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeContainer3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokeInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokeInput
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokeInput[] = L"Windows.UI.Input.Inking.IInkStrokeInput";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInputVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_StrokeStarted)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_StrokeStarted)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_StrokeContinued)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_StrokeContinued)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_StrokeEnded)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_StrokeEnded)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_StrokeCanceled)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkStrokeInput_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_StrokeCanceled)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* get_InkPresenter)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInputVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInputVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_add_StrokeStarted(This, handler, cookie) \
    ((This)->lpVtbl->add_StrokeStarted(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_remove_StrokeStarted(This, cookie) \
    ((This)->lpVtbl->remove_StrokeStarted(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_add_StrokeContinued(This, handler, cookie) \
    ((This)->lpVtbl->add_StrokeContinued(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_remove_StrokeContinued(This, cookie) \
    ((This)->lpVtbl->remove_StrokeContinued(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_add_StrokeEnded(This, handler, cookie) \
    ((This)->lpVtbl->add_StrokeEnded(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_remove_StrokeEnded(This, cookie) \
    ((This)->lpVtbl->remove_StrokeEnded(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_add_StrokeCanceled(This, handler, cookie) \
    ((This)->lpVtbl->add_StrokeCanceled(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_remove_StrokeCanceled(This, cookie) \
    ((This)->lpVtbl->remove_StrokeCanceled(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_get_InkPresenter(This, value) \
    ((This)->lpVtbl->get_InkPresenter(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeInput_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokeRenderingSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokeRenderingSegment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokeRenderingSegment[] = L"Windows.UI.Input.Inking.IInkStrokeRenderingSegment";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_BezierControlPoint1)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_BezierControlPoint2)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment* This,
        struct __x_ABI_CWindows_CFoundation_CPoint* value);
    HRESULT (STDMETHODCALLTYPE* get_Pressure)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_TiltX)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_TiltY)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment* This,
        FLOAT* value);
    HRESULT (STDMETHODCALLTYPE* get_Twist)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment* This,
        FLOAT* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegmentVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_get_BezierControlPoint1(This, value) \
    ((This)->lpVtbl->get_BezierControlPoint1(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_get_BezierControlPoint2(This, value) \
    ((This)->lpVtbl->get_BezierControlPoint2(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_get_Pressure(This, value) \
    ((This)->lpVtbl->get_Pressure(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_get_TiltX(This, value) \
    ((This)->lpVtbl->get_TiltX(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_get_TiltY(This, value) \
    ((This)->lpVtbl->get_TiltY(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_get_Twist(This, value) \
    ((This)->lpVtbl->get_Twist(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokeRenderingSegment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokesCollectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokesCollectedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokesCollectedEventArgs[] = L"Windows.UI.Input.Inking.IInkStrokesCollectedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Strokes)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs* This,
        __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_get_Strokes(This, value) \
    ((This)->lpVtbl->get_Strokes(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesCollectedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkStrokesErasedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkStrokesErasedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkStrokesErasedEventArgs[] = L"Windows.UI.Input.Inking.IInkStrokesErasedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Strokes)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs* This,
        __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_get_Strokes(This, value) \
    ((This)->lpVtbl->get_Strokes(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkStrokesErasedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkSynchronizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkSynchronizer
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkSynchronizer[] = L"Windows.UI.Input.Inking.IInkSynchronizer";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* BeginDry)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer* This,
        __FIVectorView_1_Windows__CUI__CInput__CInking__CInkStroke** inkStrokes);
    HRESULT (STDMETHODCALLTYPE* EndDry)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizerVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_BeginDry(This, inkStrokes) \
    ((This)->lpVtbl->BeginDry(This, inkStrokes))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_EndDry(This) \
    ((This)->lpVtbl->EndDry(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkSynchronizer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IInkUnprocessedInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.InkUnprocessedInput
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IInkUnprocessedInput[] = L"Windows.UI.Input.Inking.IInkUnprocessedInput";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInputVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_PointerEntered)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerEntered)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerHovered)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerHovered)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerExited)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerExited)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerPressed)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerPressed)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerMoved)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerMoved)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerReleased)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerReleased)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* add_PointerLost)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        __FITypedEventHandler_2_Windows__CUI__CInput__CInking__CInkUnprocessedInput_Windows__CUI__CCore__CPointerEventArgs* handler,
        EventRegistrationToken* cookie);
    HRESULT (STDMETHODCALLTYPE* remove_PointerLost)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        EventRegistrationToken cookie);
    HRESULT (STDMETHODCALLTYPE* get_InkPresenter)(__x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIInkPresenter** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInputVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInputVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_add_PointerEntered(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerEntered(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_remove_PointerEntered(This, cookie) \
    ((This)->lpVtbl->remove_PointerEntered(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_add_PointerHovered(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerHovered(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_remove_PointerHovered(This, cookie) \
    ((This)->lpVtbl->remove_PointerHovered(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_add_PointerExited(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerExited(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_remove_PointerExited(This, cookie) \
    ((This)->lpVtbl->remove_PointerExited(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_add_PointerPressed(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerPressed(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_remove_PointerPressed(This, cookie) \
    ((This)->lpVtbl->remove_PointerPressed(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_add_PointerMoved(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerMoved(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_remove_PointerMoved(This, cookie) \
    ((This)->lpVtbl->remove_PointerMoved(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_add_PointerReleased(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerReleased(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_remove_PointerReleased(This, cookie) \
    ((This)->lpVtbl->remove_PointerReleased(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_add_PointerLost(This, handler, cookie) \
    ((This)->lpVtbl->add_PointerLost(This, handler, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_remove_PointerLost(This, cookie) \
    ((This)->lpVtbl->remove_PointerLost(This, cookie))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_get_InkPresenter(This, value) \
    ((This)->lpVtbl->get_InkPresenter(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIInkUnprocessedInput_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Input.Inking.IPenAndInkSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.PenAndInkSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IPenAndInkSettings[] = L"Windows.UI.Input.Inking.IPenAndInkSettings";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsHandwritingDirectlyIntoTextFieldEnabled)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_PenHandedness)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CPenHandedness* value);
    HRESULT (STDMETHODCALLTYPE* get_HandwritingLineHeight)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CHandwritingLineHeight* value);
    HRESULT (STDMETHODCALLTYPE* get_FontFamilyName)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_UserConsentsToHandwritingTelemetryCollection)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsTouchHandwritingEnabled)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_get_IsHandwritingDirectlyIntoTextFieldEnabled(This, value) \
    ((This)->lpVtbl->get_IsHandwritingDirectlyIntoTextFieldEnabled(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_get_PenHandedness(This, value) \
    ((This)->lpVtbl->get_PenHandedness(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_get_HandwritingLineHeight(This, value) \
    ((This)->lpVtbl->get_HandwritingLineHeight(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_get_FontFamilyName(This, value) \
    ((This)->lpVtbl->get_FontFamilyName(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_get_UserConsentsToHandwritingTelemetryCollection(This, value) \
    ((This)->lpVtbl->get_UserConsentsToHandwritingTelemetryCollection(This, value))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_get_IsTouchHandwritingEnabled(This, value) \
    ((This)->lpVtbl->get_IsTouchHandwritingEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Input.Inking.IPenAndInkSettings2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.PenAndInkSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IPenAndInkSettings2[] = L"Windows.UI.Input.Inking.IPenAndInkSettings2";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetPenHandedness)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2* This,
        enum __x_ABI_CWindows_CUI_CInput_CInking_CPenHandedness value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2Vtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_SetPenHandedness(This, value) \
    ((This)->lpVtbl->SetPenHandedness(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Interface Windows.UI.Input.Inking.IPenAndInkSettingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Input.Inking.PenAndInkSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Input_Inking_IPenAndInkSettingsStatics[] = L"Windows.UI.Input.Inking.IPenAndInkSettingsStatics";
typedef struct __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics* This,
        __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettings** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStaticsVtbl;

interface __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CInput_CInking_CIPenAndInkSettingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.Input.Inking.InkDrawingAttributes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Inking.IInkDrawingAttributesStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkDrawingAttributes ** Default Interface **
 *    Windows.UI.Input.Inking.IInkDrawingAttributes2
 *    Windows.UI.Input.Inking.IInkDrawingAttributes3
 *    Windows.UI.Input.Inking.IInkDrawingAttributes4
 *    Windows.UI.Input.Inking.IInkDrawingAttributes5
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkDrawingAttributes_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkDrawingAttributes_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkDrawingAttributes[] = L"Windows.UI.Input.Inking.InkDrawingAttributes";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkDrawingAttributesPencilProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkDrawingAttributesPencilProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkDrawingAttributesPencilProperties_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkDrawingAttributesPencilProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkDrawingAttributesPencilProperties[] = L"Windows.UI.Input.Inking.InkDrawingAttributesPencilProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.Inking.InkInputConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkInputConfiguration ** Default Interface **
 *    Windows.UI.Input.Inking.IInkInputConfiguration2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkInputConfiguration_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkInputConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkInputConfiguration[] = L"Windows.UI.Input.Inking.InkInputConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Input.Inking.InkInputProcessingConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkInputProcessingConfiguration ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkInputProcessingConfiguration_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkInputProcessingConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkInputProcessingConfiguration[] = L"Windows.UI.Input.Inking.InkInputProcessingConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkManager ** Default Interface **
 *    Windows.UI.Input.Inking.IInkRecognizerContainer
 *    Windows.UI.Input.Inking.IInkStrokeContainer
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkManager[] = L"Windows.UI.Input.Inking.InkManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkModelerAttributes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkModelerAttributes ** Default Interface **
 *    Windows.UI.Input.Inking.IInkModelerAttributes2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkModelerAttributes_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkModelerAttributes_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkModelerAttributes[] = L"Windows.UI.Input.Inking.InkModelerAttributes";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.UI.Input.Inking.InkPoint
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Input.Inking.IInkPointFactory2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.UI.Input.Inking.IInkPointFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkPoint ** Default Interface **
 *    Windows.UI.Input.Inking.IInkPoint2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkPoint_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkPoint_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkPoint[] = L"Windows.UI.Input.Inking.InkPoint";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkPresenter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkPresenter ** Default Interface **
 *    Windows.UI.Input.Inking.IInkPresenter2
 *    Windows.UI.Input.Inking.IInkPresenter3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkPresenter_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkPresenter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkPresenter[] = L"Windows.UI.Input.Inking.InkPresenter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkPresenterProtractor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Input.Inking.IInkPresenterProtractorFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkPresenterProtractor ** Default Interface **
 *    Windows.UI.Input.Inking.IInkPresenterStencil
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkPresenterProtractor_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkPresenterProtractor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkPresenterProtractor[] = L"Windows.UI.Input.Inking.InkPresenterProtractor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Input.Inking.InkPresenterRuler
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Input.Inking.IInkPresenterRulerFactory interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkPresenterRuler ** Default Interface **
 *    Windows.UI.Input.Inking.IInkPresenterStencil
 *    Windows.UI.Input.Inking.IInkPresenterRuler2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkPresenterRuler_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkPresenterRuler_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkPresenterRuler[] = L"Windows.UI.Input.Inking.InkPresenterRuler";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Input.Inking.InkRecognitionResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkRecognitionResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkRecognitionResult_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkRecognitionResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkRecognitionResult[] = L"Windows.UI.Input.Inking.InkRecognitionResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkRecognizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkRecognizer ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkRecognizer_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkRecognizer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkRecognizer[] = L"Windows.UI.Input.Inking.InkRecognizer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkRecognizerContainer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkRecognizerContainer ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkRecognizerContainer_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkRecognizerContainer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkRecognizerContainer[] = L"Windows.UI.Input.Inking.InkRecognizerContainer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkStroke
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkStroke ** Default Interface **
 *    Windows.UI.Input.Inking.IInkStroke2
 *    Windows.UI.Input.Inking.IInkStroke3
 *    Windows.UI.Input.Inking.IInkStroke4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkStroke_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkStroke_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkStroke[] = L"Windows.UI.Input.Inking.InkStroke";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkStrokeBuilder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkStrokeBuilder ** Default Interface **
 *    Windows.UI.Input.Inking.IInkStrokeBuilder2
 *    Windows.UI.Input.Inking.IInkStrokeBuilder3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokeBuilder_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokeBuilder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkStrokeBuilder[] = L"Windows.UI.Input.Inking.InkStrokeBuilder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkStrokeContainer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkStrokeContainer ** Default Interface **
 *    Windows.UI.Input.Inking.IInkStrokeContainer2
 *    Windows.UI.Input.Inking.IInkStrokeContainer3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokeContainer_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokeContainer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkStrokeContainer[] = L"Windows.UI.Input.Inking.InkStrokeContainer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkStrokeInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkStrokeInput ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokeInput_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokeInput_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkStrokeInput[] = L"Windows.UI.Input.Inking.InkStrokeInput";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkStrokeRenderingSegment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkStrokeRenderingSegment ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokeRenderingSegment_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokeRenderingSegment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkStrokeRenderingSegment[] = L"Windows.UI.Input.Inking.InkStrokeRenderingSegment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkStrokesCollectedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkStrokesCollectedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokesCollectedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokesCollectedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkStrokesCollectedEventArgs[] = L"Windows.UI.Input.Inking.InkStrokesCollectedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkStrokesErasedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkStrokesErasedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokesErasedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkStrokesErasedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkStrokesErasedEventArgs[] = L"Windows.UI.Input.Inking.InkStrokesErasedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkSynchronizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkSynchronizer ** Default Interface **
 *
 * Class Marshaling Behavior:  None - Class cannot be marshaled
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkSynchronizer_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkSynchronizer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkSynchronizer[] = L"Windows.UI.Input.Inking.InkSynchronizer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.InkUnprocessedInput
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IInkUnprocessedInput ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_InkUnprocessedInput_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_InkUnprocessedInput_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_InkUnprocessedInput[] = L"Windows.UI.Input.Inking.InkUnprocessedInput";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Input.Inking.PenAndInkSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Input.Inking.IPenAndInkSettingsStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Input.Inking.IPenAndInkSettings ** Default Interface **
 *    Windows.UI.Input.Inking.IPenAndInkSettings2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_Input_Inking_PenAndInkSettings_DEFINED
#define RUNTIMECLASS_Windows_UI_Input_Inking_PenAndInkSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Input_Inking_PenAndInkSettings[] = L"Windows.UI.Input.Inking.PenAndInkSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eui2Einput2Einking_p_h__

#endif // __windows2Eui2Einput2Einking_h__
