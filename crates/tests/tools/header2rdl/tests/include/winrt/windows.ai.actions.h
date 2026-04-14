
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
#ifndef __windows2Eai2Eactions_h__
#define __windows2Eai2Eactions_h__
#ifndef __windows2Eai2Eactions_p_h__
#define __windows2Eai2Eactions_p_h__


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
#if !defined(WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION)
#define WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION 0x80000
#endif // defined(WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION)

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
#include "Windows.AI.Actions.Hosting.h"
#include "Windows.ApplicationModel.Appointments.h"
#include "Windows.ApplicationModel.Contacts.h"
#include "Windows.UI.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionEntity ABI::Windows::AI::Actions::IActionEntity

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntity2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntity2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionEntity2;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionEntity2 ABI::Windows::AI::Actions::IActionEntity2

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntity2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionEntityDisplayInfo;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo ABI::Windows::AI::Actions::IActionEntityDisplayInfo

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionEntityFactory;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory ABI::Windows::AI::Actions::IActionEntityFactory

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionEntityFactory2;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2 ABI::Windows::AI::Actions::IActionEntityFactory2

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionEntityFactory3;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3 ABI::Windows::AI::Actions::IActionEntityFactory3

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionEntityFactory4;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4 ABI::Windows::AI::Actions::IActionEntityFactory4

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionEntityFactory5;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5 ABI::Windows::AI::Actions::IActionEntityFactory5

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionEntityFactory6;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6 ABI::Windows::AI::Actions::IActionEntityFactory6

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionEntityFactory7;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7 ABI::Windows::AI::Actions::IActionEntityFactory7

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionEntityFactoryFactory;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory ABI::Windows::AI::Actions::IActionEntityFactoryFactory

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionFeedback_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionFeedback_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionFeedback;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionFeedback ABI::Windows::AI::Actions::IActionFeedback

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionFeedback_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionInvocationContext;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext ABI::Windows::AI::Actions::IActionInvocationContext

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionInvocationContext2;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2 ABI::Windows::AI::Actions::IActionInvocationContext2

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionInvocationHelpDetails;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails ABI::Windows::AI::Actions::IActionInvocationHelpDetails

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionInvocationHelpDetails2;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2 ABI::Windows::AI::Actions::IActionInvocationHelpDetails2

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionRuntime_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionRuntime;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime ABI::Windows::AI::Actions::IActionRuntime

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionRuntime_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionRuntime2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionRuntime2;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime2 ABI::Windows::AI::Actions::IActionRuntime2

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionRuntime2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionRuntime3_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionRuntime3;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime3 ABI::Windows::AI::Actions::IActionRuntime3

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionRuntime3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionRuntime4_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionRuntime4;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime4 ABI::Windows::AI::Actions::IActionRuntime4

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionRuntime4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionRuntime5_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionRuntime5;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime5 ABI::Windows::AI::Actions::IActionRuntime5

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionRuntime5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionRuntimeFactory;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory ABI::Windows::AI::Actions::IActionRuntimeFactory

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IActionRuntimeStatics;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics ABI::Windows::AI::Actions::IActionRuntimeStatics

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IAppointmentActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity ABI::Windows::AI::Actions::IAppointmentActionEntity

#endif // ____x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IArrayActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity ABI::Windows::AI::Actions::IArrayActionEntity

#endif // ____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IArrayActionEntity2;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2 ABI::Windows::AI::Actions::IArrayActionEntity2

#endif // ____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIContactActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIContactActionEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IContactActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIContactActionEntity ABI::Windows::AI::Actions::IContactActionEntity

#endif // ____x_ABI_CWindows_CAI_CActions_CIContactActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface ICustomActionEntityStore;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore ABI::Windows::AI::Actions::ICustomActionEntityStore

#endif // ____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface ICustomActionEntityStoreFactory;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory ABI::Windows::AI::Actions::ICustomActionEntityStoreFactory

#endif // ____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface ICustomTextActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity ABI::Windows::AI::Actions::ICustomTextActionEntity

#endif // ____x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IDateTimeActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity ABI::Windows::AI::Actions::IDateTimeActionEntity

#endif // ____x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IDocumentActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity ABI::Windows::AI::Actions::IDocumentActionEntity

#endif // ____x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIFileActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIFileActionEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IFileActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIFileActionEntity ABI::Windows::AI::Actions::IFileActionEntity

#endif // ____x_ABI_CWindows_CAI_CActions_CIFileActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CINamedActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CINamedActionEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface INamedActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CINamedActionEntity ABI::Windows::AI::Actions::INamedActionEntity

#endif // ____x_ABI_CWindows_CAI_CActions_CINamedActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IPhotoActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity ABI::Windows::AI::Actions::IPhotoActionEntity

#endif // ____x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IRemoteFileActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity ABI::Windows::AI::Actions::IRemoteFileActionEntity

#endif // ____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IRemoteFileActionEntity2;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2 ABI::Windows::AI::Actions::IRemoteFileActionEntity2

#endif // ____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IStreamingTextActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity ABI::Windows::AI::Actions::IStreamingTextActionEntity

#endif // ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IStreamingTextActionEntityTextChangedArgs;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs ABI::Windows::AI::Actions::IStreamingTextActionEntityTextChangedArgs

#endif // ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IStreamingTextActionEntityWriter;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter ABI::Windows::AI::Actions::IStreamingTextActionEntityWriter

#endif // ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CITableActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CITableActionEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface ITableActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CITableActionEntity ABI::Windows::AI::Actions::ITableActionEntity

#endif // ____x_ABI_CWindows_CAI_CActions_CITableActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CITextActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CITextActionEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface ITextActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CITextActionEntity ABI::Windows::AI::Actions::ITextActionEntity

#endif // ____x_ABI_CWindows_CAI_CActions_CITextActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CITextActionEntity2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CITextActionEntity2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface ITextActionEntity2;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CITextActionEntity2 ABI::Windows::AI::Actions::ITextActionEntity2

#endif // ____x_ABI_CWindows_CAI_CActions_CITextActionEntity2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIUriActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIUriActionEntity_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                interface IUriActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CIUriActionEntity ABI::Windows::AI::Actions::IUriActionEntity

#endif // ____x_ABI_CWindows_CAI_CActions_CIUriActionEntity_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("09335560-6c6b-5a26-9348-97b781132b20"))
IKeyValuePair<HSTRING, IInspectable*> : IKeyValuePair_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, IInspectable*> __FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5db5fa32-707c-5849-a06b-91c8eb9d10e8"))
IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fe2f3d47-5d47-5499-8374-430c7cda0204"))
IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Object>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_IInspectable*> __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_USE */



#ifndef DEF___FIMapView_2_HSTRING_IInspectable_USE
#define DEF___FIMapView_2_HSTRING_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bb78502a-f79d-54fa-92c9-90c5039fdf7e"))
IMapView<HSTRING, IInspectable*> : IMapView_impl<HSTRING, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, IInspectable*> __FIMapView_2_HSTRING_IInspectable_t;
#define __FIMapView_2_HSTRING_IInspectable ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_IInspectable_USE */


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
        namespace AI {
            namespace Actions {
                class ActionInvocationHelpDetails;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b40f316f-ed8e-58ba-9274-7ceec66746b2"))
ITypedEventHandler<ABI::Windows::AI::Actions::ActionInvocationHelpDetails*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::AI::Actions::ActionInvocationHelpDetails*, ABI::Windows::AI::Actions::IActionInvocationHelpDetails*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.AI.Actions.ActionInvocationHelpDetails, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::AI::Actions::ActionInvocationHelpDetails*, IInspectable*> __FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable_USE */

#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class StreamingTextActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class StreamingTextActionEntityTextChangedArgs;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9154a3ed-c383-5bdd-a8ae-fab2c13869d5"))
ITypedEventHandler<ABI::Windows::AI::Actions::StreamingTextActionEntity*, ABI::Windows::AI::Actions::StreamingTextActionEntityTextChangedArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::AI::Actions::StreamingTextActionEntity*, ABI::Windows::AI::Actions::IStreamingTextActionEntity*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::AI::Actions::StreamingTextActionEntityTextChangedArgs*, ABI::Windows::AI::Actions::IStreamingTextActionEntityTextChangedArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.AI.Actions.StreamingTextActionEntity, Windows.AI.Actions.StreamingTextActionEntityTextChangedArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::AI::Actions::StreamingTextActionEntity*, ABI::Windows::AI::Actions::StreamingTextActionEntityTextChangedArgs*> __FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs_t;
#define __FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs_USE */

#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                namespace Hosting {
                    class ActionCatalog;
                } /* Hosting */
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CAI_CActions_CHosting_CIActionCatalog_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CHosting_CIActionCatalog_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                namespace Hosting {
                    interface IActionCatalog;
                } /* Hosting */
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CAI_CActions_CHosting_CIActionCatalog ABI::Windows::AI::Actions::Hosting::IActionCatalog

#endif // ____x_ABI_CWindows_CAI_CActions_CHosting_CIActionCatalog_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class Appointment;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointment;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment ABI::Windows::ApplicationModel::Appointments::IAppointment

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                class Contact;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                interface IContact;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CIContact ABI::Windows::ApplicationModel::Contacts::IContact

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IClosable;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIClosable ABI::Windows::Foundation::IClosable

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

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
            typedef struct WindowId WindowId;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                typedef enum ActionEntityKind : int ActionEntityKind;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                typedef enum ActionEntityTextFormat : int ActionEntityTextFormat;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                typedef enum ActionFeedbackKind : int ActionFeedbackKind;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                typedef enum ActionInvocationHelpKind : int ActionInvocationHelpKind;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                typedef enum ActionInvocationResult : int ActionInvocationResult;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                typedef enum RemoteFileKind : int RemoteFileKind;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class ActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class ActionEntityDisplayInfo;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class ActionEntityFactory;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class ActionFeedback;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class ActionInvocationContext;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class ActionRuntime;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class AppointmentActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class ArrayActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class ContactActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class CustomActionEntityStore;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class CustomTextActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class DateTimeActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class DocumentActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class FileActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class NamedActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class PhotoActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class RemoteFileActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class StreamingTextActionEntityWriter;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class TableActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class TextActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                class UriActionEntity;
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.AI.Actions.ActionEntityKind
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                enum ActionEntityKind : int
                {
                    ActionEntityKind_None = 0,
                    ActionEntityKind_Document = 1,
                    ActionEntityKind_File = 2,
                    ActionEntityKind_Photo = 3,
                    ActionEntityKind_Text = 4,
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
                    ActionEntityKind_StreamingText = 5,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
                    ActionEntityKind_RemoteFile = 6,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
                    ActionEntityKind_Table = 7,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
                    ActionEntityKind_Contact = 8,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
                    ActionEntityKind_Uri = 9,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
                    ActionEntityKind_Array = 10,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
                    ActionEntityKind_Appointment = 11,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
                    ActionEntityKind_Date = 12,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
                    ActionEntityKind_CustomText = 13,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
                };
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.AI.Actions.ActionEntityTextFormat
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                enum ActionEntityTextFormat : int
                {
                    ActionEntityTextFormat_Plain = 0,
                    ActionEntityTextFormat_Markdown = 1,
                };
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.AI.Actions.ActionFeedbackKind
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                enum ActionFeedbackKind : int
                {
                    ActionFeedbackKind_Positive = 0,
                    ActionFeedbackKind_Negative = 1,
                };
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.AI.Actions.ActionInvocationHelpKind
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                enum ActionInvocationHelpKind : int
                {
                    ActionInvocationHelpKind_None = 0,
                    ActionInvocationHelpKind_Error = 1,
                    ActionInvocationHelpKind_Warning = 2,
                };
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.AI.Actions.ActionInvocationResult
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                enum ActionInvocationResult : int
                {
                    ActionInvocationResult_Success = 0,
                    ActionInvocationResult_UserCanceled = 1,
                    ActionInvocationResult_Unsupported = 2,
                    ActionInvocationResult_Unavailable = 3,
                };
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.AI.Actions.RemoteFileKind
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                enum RemoteFileKind : int
                {
                    RemoteFileKind_Document = 0,
                    RemoteFileKind_Photo = 1,
                    RemoteFileKind_File = 2,
                };
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntity[] = L"Windows.AI.Actions.IActionEntity";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("445e700f-2122-5668-9a16-4cab2982c5f4")
                IActionEntity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::AI::Actions::ActionEntityKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayInfo(
                        ABI::Windows::AI::Actions::IActionEntityDisplayInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionEntity = __uuidof(IActionEntity);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IActionEntity2
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntity2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntity2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntity2[] = L"Windows.AI.Actions.IActionEntity2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("98fe136d-dd3a-58c1-af76-feb4e19dce9e")
                IActionEntity2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionEntity2 = __uuidof(IActionEntity2);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntity2;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntity2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IActionEntityDisplayInfo
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntityDisplayInfo
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityDisplayInfo[] = L"Windows.AI.Actions.IActionEntityDisplayInfo";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("057a9ede-03e1-55c6-acba-c7056216735a")
                IActionEntityDisplayInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionEntityDisplayInfo = __uuidof(IActionEntityDisplayInfo);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IActionEntityFactory
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityFactory[] = L"Windows.AI.Actions.IActionEntityFactory";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("9cb752a0-5bf8-5be2-916e-b00eff80088d")
                IActionEntityFactory : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IActionEntityFactory = __uuidof(IActionEntityFactory);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityFactory;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IActionEntityFactory2
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntityFactory
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityFactory2[] = L"Windows.AI.Actions.IActionEntityFactory2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("ea2fb6a5-ec6d-5180-9d30-bc663b84e7b8")
                IActionEntityFactory2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFileEntity(
                        HSTRING path,
                        ABI::Windows::AI::Actions::IFileActionEntity** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateDocumentEntity(
                        HSTRING path,
                        ABI::Windows::AI::Actions::IDocumentActionEntity** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreatePhotoEntity(
                        HSTRING path,
                        ABI::Windows::AI::Actions::IPhotoActionEntity** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateTextEntity(
                        HSTRING text,
                        ABI::Windows::AI::Actions::ITextActionEntity** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionEntityFactory2 = __uuidof(IActionEntityFactory2);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IActionEntityFactory3
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntityFactory
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityFactory3[] = L"Windows.AI.Actions.IActionEntityFactory3";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("4910e689-00b5-56bb-9c65-0fcc76215283")
                IActionEntityFactory3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateRemoteFileEntity(
                        HSTRING sourceId,
                        ABI::Windows::AI::Actions::RemoteFileKind fileKind,
                        ABI::Windows::Foundation::IUriRuntimeClass* sourceUri,
                        HSTRING fileId,
                        HSTRING contentType,
                        HSTRING driveId,
                        HSTRING accountId,
                        HSTRING extension,
                        ABI::Windows::AI::Actions::IRemoteFileActionEntity** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateTextEntityWithTextFormat(
                        HSTRING text,
                        ABI::Windows::AI::Actions::ActionEntityTextFormat textFormat,
                        ABI::Windows::AI::Actions::ITextActionEntity** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateStreamingTextActionEntityWriter(
                        ABI::Windows::AI::Actions::ActionEntityTextFormat textFormat,
                        ABI::Windows::AI::Actions::IStreamingTextActionEntityWriter** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionEntityFactory3 = __uuidof(IActionEntityFactory3);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IActionEntityFactory4
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntityFactory
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityFactory4[] = L"Windows.AI.Actions.IActionEntityFactory4";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("332eda05-de0e-5a58-b318-a2ad771f013d")
                IActionEntityFactory4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateTableEntity(
                        UINT32 dataLength,
                        HSTRING* data,
                        UINT32 columnCount,
                        ABI::Windows::AI::Actions::ITableActionEntity** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateContactEntity(
                        ABI::Windows::ApplicationModel::Contacts::IContact* contact,
                        ABI::Windows::AI::Actions::IContactActionEntity** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionEntityFactory4 = __uuidof(IActionEntityFactory4);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.AI.Actions.IActionEntityFactory5
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 6.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntityFactory
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityFactory5[] = L"Windows.AI.Actions.IActionEntityFactory5";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("b59faab1-cfe4-564a-a5ba-53ad7ff6f924")
                IActionEntityFactory5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateUriEntity(
                        ABI::Windows::Foundation::IUriRuntimeClass* Uri,
                        ABI::Windows::AI::Actions::IUriActionEntity** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateArrayEntity(
                        ABI::Windows::AI::Actions::ActionEntityKind kind,
                        UINT32 entitiesLength,
                        ABI::Windows::AI::Actions::IActionEntity** entities,
                        ABI::Windows::AI::Actions::IArrayActionEntity** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionEntityFactory5 = __uuidof(IActionEntityFactory5);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IActionEntityFactory6
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 7.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntityFactory
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityFactory6[] = L"Windows.AI.Actions.IActionEntityFactory6";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("da7123da-5639-590f-a2db-c3b5e221f3b6")
                IActionEntityFactory6 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateDateTimeEntity(
                        ABI::Windows::Foundation::DateTime dateTime,
                        ABI::Windows::AI::Actions::IDateTimeActionEntity** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAppointmentEntity(
                        HSTRING sourceId,
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        UINT32 attendeesLength,
                        ABI::Windows::AI::Actions::IContactActionEntity** attendees,
                        ABI::Windows::AI::Actions::IAppointmentActionEntity** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionEntityFactory6 = __uuidof(IActionEntityFactory6);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IActionEntityFactory7
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntityFactory
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityFactory7[] = L"Windows.AI.Actions.IActionEntityFactory7";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("b814b8d5-c9b2-51b5-a342-9fe054d8a1eb")
                IActionEntityFactory7 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateCustomTextEntity(
                        HSTRING kind,
                        HSTRING keyPhrase,
                        __FIMapView_2_HSTRING_IInspectable* props,
                        ABI::Windows::AI::Actions::ICustomTextActionEntity** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateArrayEntityWithCustomKind(
                        ABI::Windows::AI::Actions::ActionEntityKind elementKind,
                        HSTRING customKind,
                        UINT32 entitiesLength,
                        ABI::Windows::AI::Actions::IActionEntity** entities,
                        ABI::Windows::AI::Actions::IArrayActionEntity** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionEntityFactory7 = __uuidof(IActionEntityFactory7);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IActionEntityFactoryFactory
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntityFactory
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityFactoryFactory[] = L"Windows.AI.Actions.IActionEntityFactoryFactory";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("c9147d8f-88a0-5ec0-a564-47e2a1081412")
                IActionEntityFactoryFactory : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IActionEntityFactoryFactory = __uuidof(IActionEntityFactoryFactory);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IActionFeedback
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionFeedback
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionFeedback_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionFeedback_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionFeedback[] = L"Windows.AI.Actions.IActionFeedback";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("a12ee7ab-2454-56c9-bbdf-c089457fbc5e")
                IActionFeedback : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FeedbackKind(
                        ABI::Windows::AI::Actions::ActionFeedbackKind* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionFeedback = __uuidof(IActionFeedback);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionFeedback;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionFeedback_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IActionInvocationContext
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionInvocationContext
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionInvocationContext[] = L"Windows.AI.Actions.IActionInvocationContext";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("c32b622e-86e1-5eba-9661-605910104978")
                IActionInvocationContext : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_EntityFactory(
                        ABI::Windows::AI::Actions::IActionEntityFactory2** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetInputEntity(
                        HSTRING inputName,
                        ABI::Windows::AI::Actions::IActionEntity* inputValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetInputEntities(
                        UINT32* resultLength,
                        ABI::Windows::AI::Actions::INamedActionEntity*** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetOutputEntity(
                        HSTRING outputName,
                        ABI::Windows::AI::Actions::IActionEntity* outputValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetOutputEntities(
                        UINT32* resultLength,
                        ABI::Windows::AI::Actions::INamedActionEntity*** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Result(
                        ABI::Windows::AI::Actions::ActionInvocationResult* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Result(
                        ABI::Windows::AI::Actions::ActionInvocationResult value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ExtendedError(
                        HRESULT value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionInvocationContext = __uuidof(IActionInvocationContext);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionInvocationContext;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IActionInvocationContext2
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionInvocationContext
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionInvocationContext2[] = L"Windows.AI.Actions.IActionInvocationContext2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("7c843086-9279-5bcd-8f2e-d15121e7a827")
                IActionInvocationContext2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_InvokerWindowId(
                        ABI::Windows::UI::WindowId* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HelpDetails(
                        ABI::Windows::AI::Actions::IActionInvocationHelpDetails** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActionId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InvokerAppUserModelId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionInvocationContext2 = __uuidof(IActionInvocationContext2);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IActionInvocationHelpDetails
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionInvocationHelpDetails
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionInvocationHelpDetails[] = L"Windows.AI.Actions.IActionInvocationHelpDetails";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("5430f272-078f-5722-8f7d-90cf8ddd595e")
                IActionInvocationHelpDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::AI::Actions::ActionInvocationHelpKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Kind(
                        ABI::Windows::AI::Actions::ActionInvocationHelpKind value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Description(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HelpUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HelpUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HelpUriDescription(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HelpUriDescription(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionInvocationHelpDetails = __uuidof(IActionInvocationHelpDetails);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IActionInvocationHelpDetails2
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionInvocationHelpDetails
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionInvocationHelpDetails2[] = L"Windows.AI.Actions.IActionInvocationHelpDetails2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("307f6ba5-5fda-59f1-9722-1859801ad550")
                IActionInvocationHelpDetails2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_Changed(
                        __FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Changed(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionInvocationHelpDetails2 = __uuidof(IActionInvocationHelpDetails2);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IActionRuntime
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionRuntime
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionRuntime[] = L"Windows.AI.Actions.IActionRuntime";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("206efa2c-c909-508a-b4b0-9482be96db9c")
                IActionRuntime : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ActionCatalog(
                        ABI::Windows::AI::Actions::Hosting::IActionCatalog** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EntityFactory(
                        ABI::Windows::AI::Actions::IActionEntityFactory2** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateInvocationContext(
                        HSTRING actionId,
                        ABI::Windows::AI::Actions::IActionInvocationContext** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionRuntime = __uuidof(IActionRuntime);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionRuntime;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IActionRuntime2
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionRuntime
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionRuntime2[] = L"Windows.AI.Actions.IActionRuntime2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("2da4d2c0-e593-5350-8143-15bb24f63411")
                IActionRuntime2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateActionFeedback(
                        ABI::Windows::AI::Actions::ActionFeedbackKind feedbackKind,
                        ABI::Windows::AI::Actions::IActionFeedback** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetActionAvailability(
                        HSTRING actionId,
                        boolean isAvailable
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetActionAvailability(
                        HSTRING actionId,
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionRuntime2 = __uuidof(IActionRuntime2);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionRuntime2;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IActionRuntime3
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionRuntime
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionRuntime3[] = L"Windows.AI.Actions.IActionRuntime3";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("f020c3c0-caec-5928-ad00-81069b80fbc1")
                IActionRuntime3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInvocationContextWithWindowId(
                        HSTRING actionId,
                        ABI::Windows::UI::WindowId invokerWindowId,
                        ABI::Windows::AI::Actions::IActionInvocationContext** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetActionEntityById(
                        HSTRING entityId,
                        ABI::Windows::AI::Actions::IActionEntity** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LatestSupportedSchemaVersion(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionRuntime3 = __uuidof(IActionRuntime3);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionRuntime3;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime3_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.AI.Actions.IActionRuntime4
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionRuntime
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionRuntime4[] = L"Windows.AI.Actions.IActionRuntime4";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("06851dcd-c743-5c7f-88a1-bbaeb02f5e28")
                IActionRuntime4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetActionInvocationContextFromToken(
                        HSTRING token,
                        ABI::Windows::AI::Actions::IActionInvocationContext** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionRuntime4 = __uuidof(IActionRuntime4);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionRuntime4;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime4_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.AI.Actions.IActionRuntime5
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionRuntime
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionRuntime5[] = L"Windows.AI.Actions.IActionRuntime5";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("c2e995b1-52a9-5f3a-bebb-a04655e96218")
                IActionRuntime5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CustomEntityStore(
                        ABI::Windows::AI::Actions::ICustomActionEntityStore** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionRuntime5 = __uuidof(IActionRuntime5);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionRuntime5;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime5_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IActionRuntimeFactory
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionRuntime
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionRuntimeFactory[] = L"Windows.AI.Actions.IActionRuntimeFactory";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("d3f366e9-8dc9-50a0-8040-e5c14fa609d6")
                IActionRuntimeFactory : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IActionRuntimeFactory = __uuidof(IActionRuntimeFactory);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IActionRuntimeStatics
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionRuntime
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionRuntimeStatics[] = L"Windows.AI.Actions.IActionRuntimeStatics";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("2c697aab-55f2-55aa-9d63-a73ec190cecd")
                IActionRuntimeStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::AI::Actions::IActionRuntime** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IActionRuntimeStatics = __uuidof(IActionRuntimeStatics);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.AI.Actions.IAppointmentActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 7.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.AppointmentActionEntity
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IAppointmentActionEntity[] = L"Windows.AI.Actions.IAppointmentActionEntity";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("29daa00e-b474-581c-b555-6187d1aa8231")
                IAppointmentActionEntity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SourceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Appointment(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAttendees(
                        UINT32* resultLength,
                        ABI::Windows::AI::Actions::IContactActionEntity*** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPresentedFiles(
                        UINT32* resultLength,
                        ABI::Windows::AI::Actions::IRemoteFileActionEntity*** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetPresentedFiles(
                        UINT32 filesLength,
                        ABI::Windows::AI::Actions::IRemoteFileActionEntity** files
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSharedFiles(
                        UINT32* resultLength,
                        ABI::Windows::AI::Actions::IRemoteFileActionEntity*** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetSharedFiles(
                        UINT32 filesLength,
                        ABI::Windows::AI::Actions::IRemoteFileActionEntity** files
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentActionEntity = __uuidof(IAppointmentActionEntity);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IArrayActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 6.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ArrayActionEntity
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IArrayActionEntity[] = L"Windows.AI.Actions.IArrayActionEntity";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("45798e78-1059-5311-8a1b-de0081a4ca3b")
                IArrayActionEntity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ElementKind(
                        ABI::Windows::AI::Actions::ActionEntityKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAll(
                        UINT32* resultLength,
                        ABI::Windows::AI::Actions::IActionEntity*** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IArrayActionEntity = __uuidof(IArrayActionEntity);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIArrayActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IArrayActionEntity2
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ArrayActionEntity
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IArrayActionEntity2[] = L"Windows.AI.Actions.IArrayActionEntity2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("7366e049-7fe8-5df9-bbca-cea5c0f3d316")
                IArrayActionEntity2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CustomElementKind(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IArrayActionEntity2 = __uuidof(IArrayActionEntity2);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IContactActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ContactActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIContactActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIContactActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IContactActionEntity[] = L"Windows.AI.Actions.IContactActionEntity";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("458c3e07-5892-5485-bd9b-8f7a540c9501")
                IContactActionEntity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        ABI::Windows::ApplicationModel::Contacts::IContact** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IContactActionEntity = __uuidof(IContactActionEntity);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIContactActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIContactActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.AI.Actions.ICustomActionEntityStore
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.CustomActionEntityStore
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_ICustomActionEntityStore[] = L"Windows.AI.Actions.ICustomActionEntityStore";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("fa7b44d0-1762-5828-9938-e7cae5199e01")
                ICustomActionEntityStore : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetLastModifiedTime(
                        HSTRING kind,
                        ABI::Windows::Foundation::DateTime* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Insert(
                        ABI::Windows::AI::Actions::ICustomTextActionEntity* entity
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE InsertMany(
                        UINT32 entitiesLength,
                        ABI::Windows::AI::Actions::ICustomTextActionEntity** entities
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Delete(
                        HSTRING kind
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICustomActionEntityStore = __uuidof(ICustomActionEntityStore);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.ICustomActionEntityStoreFactory
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.CustomActionEntityStore
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_ICustomActionEntityStoreFactory[] = L"Windows.AI.Actions.ICustomActionEntityStoreFactory";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("d8b46bdb-68a5-5e07-9113-abb9241aaab1")
                ICustomActionEntityStoreFactory : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_ICustomActionEntityStoreFactory = __uuidof(ICustomActionEntityStoreFactory);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.ICustomTextActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.CustomTextActionEntity
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_ICustomTextActionEntity[] = L"Windows.AI.Actions.ICustomTextActionEntity";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("50eaac95-1d6c-54b0-8963-e38dea3f6aec")
                ICustomTextActionEntity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CustomTextKind(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_KeyPhrase(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        __FIMapView_2_HSTRING_IInspectable** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICustomTextActionEntity = __uuidof(ICustomTextActionEntity);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IDateTimeActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 7.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.DateTimeActionEntity
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IDateTimeActionEntity[] = L"Windows.AI.Actions.IDateTimeActionEntity";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("fd5a0880-eeae-553a-bfed-a9229d57447d")
                IDateTimeActionEntity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DateTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDateTimeActionEntity = __uuidof(IDateTimeActionEntity);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IDocumentActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.DocumentActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IDocumentActionEntity[] = L"Windows.AI.Actions.IDocumentActionEntity";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("56715297-960b-59ff-af4b-ece1098b2e36")
                IDocumentActionEntity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FullPath(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDocumentActionEntity = __uuidof(IDocumentActionEntity);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IFileActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.FileActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIFileActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIFileActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IFileActionEntity[] = L"Windows.AI.Actions.IFileActionEntity";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("f20ab43f-4c80-5904-bd42-3e6248babfcf")
                IFileActionEntity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FullPath(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFileActionEntity = __uuidof(IFileActionEntity);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIFileActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIFileActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.INamedActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.NamedActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CINamedActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CINamedActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_INamedActionEntity[] = L"Windows.AI.Actions.INamedActionEntity";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("1aaebeef-435b-5a0d-8182-05fe4dd47712")
                INamedActionEntity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Name(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Entity(
                        ABI::Windows::AI::Actions::IActionEntity** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Entity(
                        ABI::Windows::AI::Actions::IActionEntity* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INamedActionEntity = __uuidof(INamedActionEntity);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CINamedActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CINamedActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IPhotoActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.PhotoActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IPhotoActionEntity[] = L"Windows.AI.Actions.IPhotoActionEntity";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("425123b3-20ef-51a6-b35f-8414384765c5")
                IPhotoActionEntity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FullPath(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhotoActionEntity = __uuidof(IPhotoActionEntity);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IRemoteFileActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.RemoteFileActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IRemoteFileActionEntity[] = L"Windows.AI.Actions.IRemoteFileActionEntity";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("a5d8ec21-a2bd-545a-abfc-d7aa79fd0b81")
                IRemoteFileActionEntity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SourceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FileKind(
                        ABI::Windows::AI::Actions::RemoteFileKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SourceUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FileId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContentType(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DriveId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AccountId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Extension(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteFileActionEntity = __uuidof(IRemoteFileActionEntity);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IRemoteFileActionEntity2
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 7.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.RemoteFileActionEntity
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IRemoteFileActionEntity2[] = L"Windows.AI.Actions.IRemoteFileActionEntity2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("9cc8cc54-77d8-5537-83c4-6f18c1bc9f67")
                IRemoteFileActionEntity2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Filename(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Filename(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Creator(
                        ABI::Windows::AI::Actions::IContactActionEntity** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Creator(
                        ABI::Windows::AI::Actions::IContactActionEntity* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LastUpdatedTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LastUpdatedTime(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetContributors(
                        UINT32 contributorsLength,
                        ABI::Windows::AI::Actions::IContactActionEntity** contributors
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetContributors(
                        UINT32* resultLength,
                        ABI::Windows::AI::Actions::IContactActionEntity*** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteFileActionEntity2 = __uuidof(IRemoteFileActionEntity2);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IStreamingTextActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.StreamingTextActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IStreamingTextActionEntity[] = L"Windows.AI.Actions.IStreamingTextActionEntity";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("44cd8a16-abc9-5703-b4bf-6fe8b7a802fd")
                IStreamingTextActionEntity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsComplete(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetText(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TextFormat(
                        ABI::Windows::AI::Actions::ActionEntityTextFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_TextChanged(
                        __FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TextChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamingTextActionEntity = __uuidof(IStreamingTextActionEntity);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IStreamingTextActionEntityTextChangedArgs
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.StreamingTextActionEntityTextChangedArgs
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IStreamingTextActionEntityTextChangedArgs[] = L"Windows.AI.Actions.IStreamingTextActionEntityTextChangedArgs";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("2c62011f-3e06-588b-a3bd-d726bd82fb13")
                IStreamingTextActionEntityTextChangedArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsComplete(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamingTextActionEntityTextChangedArgs = __uuidof(IStreamingTextActionEntityTextChangedArgs);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IStreamingTextActionEntityWriter
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.StreamingTextActionEntityWriter
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IStreamingTextActionEntityWriter[] = L"Windows.AI.Actions.IStreamingTextActionEntityWriter";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("6bce2f76-a8af-5ff2-833c-108737ba0f42")
                IStreamingTextActionEntityWriter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ReaderEntity(
                        ABI::Windows::AI::Actions::IStreamingTextActionEntity** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TextFormat(
                        ABI::Windows::AI::Actions::ActionEntityTextFormat* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetText(
                        HSTRING text
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IStreamingTextActionEntityWriter = __uuidof(IStreamingTextActionEntityWriter);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.ITableActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.TableActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CAI_CActions_CITableActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CITableActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_ITableActionEntity[] = L"Windows.AI.Actions.ITableActionEntity";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("0f252cdb-ba24-5dbb-9d17-1b300773d141")
                ITableActionEntity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetTextContent(
                        UINT32* resultLength,
                        HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RowCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ColumnCount(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITableActionEntity = __uuidof(ITableActionEntity);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CITableActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CITableActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.AI.Actions.ITextActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.TextActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CITextActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CITextActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_ITextActionEntity[] = L"Windows.AI.Actions.ITextActionEntity";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("3c4ec25f-5adb-5f73-b8f3-080fbeadd612")
                ITextActionEntity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITextActionEntity = __uuidof(ITextActionEntity);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CITextActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CITextActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.ITextActionEntity2
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.TextActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CITextActionEntity2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CITextActionEntity2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_ITextActionEntity2[] = L"Windows.AI.Actions.ITextActionEntity2";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("7c500889-cf08-51e7-beca-f0bbc7a7486c")
                ITextActionEntity2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TextFormat(
                        ABI::Windows::AI::Actions::ActionEntityTextFormat* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITextActionEntity2 = __uuidof(ITextActionEntity2);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CITextActionEntity2;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CITextActionEntity2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IUriActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 6.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.UriActionEntity
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIUriActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIUriActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IUriActionEntity[] = L"Windows.AI.Actions.IUriActionEntity";
namespace ABI {
    namespace Windows {
        namespace AI {
            namespace Actions {
                MIDL_INTERFACE("a81cde77-bc25-532d-905e-b0725c5bcd4e")
                IUriActionEntity : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUriActionEntity = __uuidof(IUriActionEntity);
            } /* Actions */
        } /* AI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIUriActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIUriActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.AI.Actions.ActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IActionEntity ** Default Interface **
 *    Windows.AI.Actions.IActionEntity2
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ActionEntity[] = L"Windows.AI.Actions.ActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.ActionEntityDisplayInfo
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IActionEntityDisplayInfo ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ActionEntityDisplayInfo_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ActionEntityDisplayInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ActionEntityDisplayInfo[] = L"Windows.AI.Actions.ActionEntityDisplayInfo";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.ActionEntityFactory
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IActionEntityFactory2 ** Default Interface **
 *    Windows.AI.Actions.IActionEntityFactory3
 *    Windows.AI.Actions.IActionEntityFactory4
 *    Windows.AI.Actions.IActionEntityFactory5
 *    Windows.AI.Actions.IActionEntityFactory6
 *    Windows.AI.Actions.IActionEntityFactory7
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ActionEntityFactory_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ActionEntityFactory_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ActionEntityFactory[] = L"Windows.AI.Actions.ActionEntityFactory";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.ActionFeedback
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IActionFeedback ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ActionFeedback_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ActionFeedback_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ActionFeedback[] = L"Windows.AI.Actions.ActionFeedback";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.AI.Actions.ActionInvocationContext
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IActionInvocationContext ** Default Interface **
 *    Windows.AI.Actions.IActionInvocationContext2
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ActionInvocationContext_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ActionInvocationContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ActionInvocationContext[] = L"Windows.AI.Actions.ActionInvocationContext";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.ActionInvocationHelpDetails
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IActionInvocationHelpDetails ** Default Interface **
 *    Windows.AI.Actions.IActionInvocationHelpDetails2
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ActionInvocationHelpDetails_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ActionInvocationHelpDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ActionInvocationHelpDetails[] = L"Windows.AI.Actions.ActionInvocationHelpDetails";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.AI.Actions.ActionRuntime
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.Actions.IActionRuntimeStatics interface starting with version 8.0 of the Windows.AI.Actions.ActionsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IActionRuntime ** Default Interface **
 *    Windows.AI.Actions.IActionRuntime2
 *    Windows.AI.Actions.IActionRuntime3
 *    Windows.AI.Actions.IActionRuntime4
 *    Windows.AI.Actions.IActionRuntime5
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ActionRuntime_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ActionRuntime_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ActionRuntime[] = L"Windows.AI.Actions.ActionRuntime";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.AppointmentActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 7.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IAppointmentActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_AI_Actions_AppointmentActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_AppointmentActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_AppointmentActionEntity[] = L"Windows.AI.Actions.AppointmentActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.AI.Actions.ArrayActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 6.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IArrayActionEntity ** Default Interface **
 *    Windows.AI.Actions.IArrayActionEntity2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ArrayActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ArrayActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ArrayActionEntity[] = L"Windows.AI.Actions.ArrayActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.AI.Actions.ContactActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IContactActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ContactActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ContactActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ContactActionEntity[] = L"Windows.AI.Actions.ContactActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.AI.Actions.CustomActionEntityStore
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.ICustomActionEntityStore ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_AI_Actions_CustomActionEntityStore_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_CustomActionEntityStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_CustomActionEntityStore[] = L"Windows.AI.Actions.CustomActionEntityStore";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.AI.Actions.CustomTextActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.ICustomTextActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_AI_Actions_CustomTextActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_CustomTextActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_CustomTextActionEntity[] = L"Windows.AI.Actions.CustomTextActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.AI.Actions.DateTimeActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 7.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IDateTimeActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_AI_Actions_DateTimeActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_DateTimeActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_DateTimeActionEntity[] = L"Windows.AI.Actions.DateTimeActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.AI.Actions.DocumentActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IDocumentActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_DocumentActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_DocumentActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_DocumentActionEntity[] = L"Windows.AI.Actions.DocumentActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.FileActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IFileActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_FileActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_FileActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_FileActionEntity[] = L"Windows.AI.Actions.FileActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.NamedActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.INamedActionEntity ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_NamedActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_NamedActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_NamedActionEntity[] = L"Windows.AI.Actions.NamedActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.PhotoActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IPhotoActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_PhotoActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_PhotoActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_PhotoActionEntity[] = L"Windows.AI.Actions.PhotoActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.RemoteFileActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IRemoteFileActionEntity ** Default Interface **
 *    Windows.AI.Actions.IRemoteFileActionEntity2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_AI_Actions_RemoteFileActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_RemoteFileActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_RemoteFileActionEntity[] = L"Windows.AI.Actions.RemoteFileActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.AI.Actions.StreamingTextActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IStreamingTextActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_AI_Actions_StreamingTextActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_StreamingTextActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_StreamingTextActionEntity[] = L"Windows.AI.Actions.StreamingTextActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.AI.Actions.StreamingTextActionEntityTextChangedArgs
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IStreamingTextActionEntityTextChangedArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_AI_Actions_StreamingTextActionEntityTextChangedArgs_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_StreamingTextActionEntityTextChangedArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_StreamingTextActionEntityTextChangedArgs[] = L"Windows.AI.Actions.StreamingTextActionEntityTextChangedArgs";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.AI.Actions.StreamingTextActionEntityWriter
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IStreamingTextActionEntityWriter ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_AI_Actions_StreamingTextActionEntityWriter_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_StreamingTextActionEntityWriter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_StreamingTextActionEntityWriter[] = L"Windows.AI.Actions.StreamingTextActionEntityWriter";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.AI.Actions.TableActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.ITableActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_AI_Actions_TableActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_TableActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_TableActionEntity[] = L"Windows.AI.Actions.TableActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.AI.Actions.TextActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.ITextActionEntity ** Default Interface **
 *    Windows.AI.Actions.ITextActionEntity2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_TextActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_TextActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_TextActionEntity[] = L"Windows.AI.Actions.TextActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.UriActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 6.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IUriActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_AI_Actions_UriActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_UriActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_UriActionEntity[] = L"Windows.AI.Actions.UriActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionEntity __x_ABI_CWindows_CAI_CActions_CIActionEntity;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntity2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntity2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionEntity2 __x_ABI_CWindows_CAI_CActions_CIActionEntity2;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntity2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo __x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2 __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3 __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4 __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5 __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6 __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7 __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory __x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionFeedback_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionFeedback_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionFeedback __x_ABI_CWindows_CAI_CActions_CIActionFeedback;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionFeedback_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2 __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2 __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionRuntime_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionRuntime __x_ABI_CWindows_CAI_CActions_CIActionRuntime;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionRuntime_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionRuntime2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionRuntime2 __x_ABI_CWindows_CAI_CActions_CIActionRuntime2;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionRuntime2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionRuntime3_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionRuntime3 __x_ABI_CWindows_CAI_CActions_CIActionRuntime3;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionRuntime3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionRuntime4_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionRuntime4 __x_ABI_CWindows_CAI_CActions_CIActionRuntime4;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionRuntime4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionRuntime5_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionRuntime5 __x_ABI_CWindows_CAI_CActions_CIActionRuntime5;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionRuntime5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory __x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics __x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics;

#endif // ____x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity;

#endif // ____x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity;

#endif // ____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2 __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2;

#endif // ____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIContactActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIContactActionEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIContactActionEntity __x_ABI_CWindows_CAI_CActions_CIContactActionEntity;

#endif // ____x_ABI_CWindows_CAI_CActions_CIContactActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore;

#endif // ____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory;

#endif // ____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity;

#endif // ____x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity __x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity;

#endif // ____x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity __x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity;

#endif // ____x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIFileActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIFileActionEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIFileActionEntity __x_ABI_CWindows_CAI_CActions_CIFileActionEntity;

#endif // ____x_ABI_CWindows_CAI_CActions_CIFileActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CINamedActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CINamedActionEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CINamedActionEntity __x_ABI_CWindows_CAI_CActions_CINamedActionEntity;

#endif // ____x_ABI_CWindows_CAI_CActions_CINamedActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity __x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity;

#endif // ____x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity;

#endif // ____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2 __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2;

#endif // ____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity;

#endif // ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs;

#endif // ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter;

#endif // ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CITableActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CITableActionEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CITableActionEntity __x_ABI_CWindows_CAI_CActions_CITableActionEntity;

#endif // ____x_ABI_CWindows_CAI_CActions_CITableActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CITextActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CITextActionEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CITextActionEntity __x_ABI_CWindows_CAI_CActions_CITextActionEntity;

#endif // ____x_ABI_CWindows_CAI_CActions_CITextActionEntity_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CITextActionEntity2_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CITextActionEntity2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CITextActionEntity2 __x_ABI_CWindows_CAI_CActions_CITextActionEntity2;

#endif // ____x_ABI_CWindows_CAI_CActions_CITextActionEntity2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CAI_CActions_CIUriActionEntity_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CIUriActionEntity_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CIUriActionEntity __x_ABI_CWindows_CAI_CActions_CIUriActionEntity;

#endif // ____x_ABI_CWindows_CAI_CActions_CIUriActionEntity_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if !defined(____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_IInspectable __FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_IInspectable* This,
        IInspectable** result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_IInspectable_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIKeyValuePair_2_HSTRING_IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_IInspectable** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_IInspectable** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

#if !defined(____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_IInspectable __FIMapView_2_HSTRING_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_IInspectable;

typedef struct __FIMapView_2_HSTRING_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_IInspectable* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_IInspectable* This,
        __FIMapView_2_HSTRING_IInspectable** first,
        __FIMapView_2_HSTRING_IInspectable** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_IInspectableVtbl;

interface __FIMapView_2_HSTRING_IInspectable
{
    CONST_VTBL struct __FIMapView_2_HSTRING_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_IInspectable_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_IInspectable_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_IInspectable_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_IInspectable_INTERFACE_DEFINED__

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

#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable __FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable* This,
        __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs __FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs;

typedef struct __FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs* This,
        __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity* sender,
        __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgsVtbl;

interface __FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs_INTERFACE_DEFINED__
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

#ifndef ____x_ABI_CWindows_CAI_CActions_CHosting_CIActionCatalog_FWD_DEFINED__
#define ____x_ABI_CWindows_CAI_CActions_CHosting_CIActionCatalog_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CAI_CActions_CHosting_CIActionCatalog __x_ABI_CWindows_CAI_CActions_CHosting_CIActionCatalog;

#endif // ____x_ABI_CWindows_CAI_CActions_CHosting_CIActionCatalog_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CIContact __x_ABI_CWindows_CApplicationModel_CContacts_CIContact;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CWindowId __x_ABI_CWindows_CUI_CWindowId;

typedef enum __x_ABI_CWindows_CAI_CActions_CActionEntityKind __x_ABI_CWindows_CAI_CActions_CActionEntityKind;

typedef enum __x_ABI_CWindows_CAI_CActions_CActionEntityTextFormat __x_ABI_CWindows_CAI_CActions_CActionEntityTextFormat;

typedef enum __x_ABI_CWindows_CAI_CActions_CActionFeedbackKind __x_ABI_CWindows_CAI_CActions_CActionFeedbackKind;

typedef enum __x_ABI_CWindows_CAI_CActions_CActionInvocationHelpKind __x_ABI_CWindows_CAI_CActions_CActionInvocationHelpKind;

typedef enum __x_ABI_CWindows_CAI_CActions_CActionInvocationResult __x_ABI_CWindows_CAI_CActions_CActionInvocationResult;

typedef enum __x_ABI_CWindows_CAI_CActions_CRemoteFileKind __x_ABI_CWindows_CAI_CActions_CRemoteFileKind;

/*
 *
 * Struct Windows.AI.Actions.ActionEntityKind
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CAI_CActions_CActionEntityKind
{
    ActionEntityKind_None = 0,
    ActionEntityKind_Document = 1,
    ActionEntityKind_File = 2,
    ActionEntityKind_Photo = 3,
    ActionEntityKind_Text = 4,
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
    ActionEntityKind_StreamingText = 5,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
    ActionEntityKind_RemoteFile = 6,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
    ActionEntityKind_Table = 7,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
    ActionEntityKind_Contact = 8,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
    ActionEntityKind_Uri = 9,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
    ActionEntityKind_Array = 10,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
    ActionEntityKind_Appointment = 11,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
    ActionEntityKind_Date = 12,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
    ActionEntityKind_CustomText = 13,
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
};
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.AI.Actions.ActionEntityTextFormat
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CAI_CActions_CActionEntityTextFormat
{
    ActionEntityTextFormat_Plain = 0,
    ActionEntityTextFormat_Markdown = 1,
};
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.AI.Actions.ActionFeedbackKind
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CAI_CActions_CActionFeedbackKind
{
    ActionFeedbackKind_Positive = 0,
    ActionFeedbackKind_Negative = 1,
};
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.AI.Actions.ActionInvocationHelpKind
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CAI_CActions_CActionInvocationHelpKind
{
    ActionInvocationHelpKind_None = 0,
    ActionInvocationHelpKind_Error = 1,
    ActionInvocationHelpKind_Warning = 2,
};
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.AI.Actions.ActionInvocationResult
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CAI_CActions_CActionInvocationResult
{
    ActionInvocationResult_Success = 0,
    ActionInvocationResult_UserCanceled = 1,
    ActionInvocationResult_Unsupported = 2,
    ActionInvocationResult_Unavailable = 3,
};
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.AI.Actions.RemoteFileKind
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CAI_CActions_CRemoteFileKind
{
    RemoteFileKind_Document = 0,
    RemoteFileKind_Photo = 1,
    RemoteFileKind_File = 2,
};
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntity[] = L"Windows.AI.Actions.IActionEntity";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CAI_CActions_CIActionEntity* This,
        enum __x_ABI_CWindows_CAI_CActions_CActionEntityKind* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayInfo)(__x_ABI_CWindows_CAI_CActions_CIActionEntity* This,
        __x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionEntityVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntity_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntity_get_DisplayInfo(This, value) \
    ((This)->lpVtbl->get_DisplayInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IActionEntity2
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntity2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntity2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntity2[] = L"Windows.AI.Actions.IActionEntity2";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionEntity2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionEntity2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionEntity2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionEntity2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionEntity2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionEntity2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionEntity2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CAI_CActions_CIActionEntity2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionEntity2Vtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionEntity2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionEntity2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionEntity2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntity2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntity2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntity2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntity2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntity2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntity2_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntity2;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntity2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IActionEntityDisplayInfo
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntityDisplayInfo
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityDisplayInfo[] = L"Windows.AI.Actions.IActionEntityDisplayInfo";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfoVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityDisplayInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IActionEntityFactory
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityFactory[] = L"Windows.AI.Actions.IActionEntityFactory";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityFactory;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IActionEntityFactory2
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntityFactory
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityFactory2[] = L"Windows.AI.Actions.IActionEntityFactory2";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFileEntity)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2* This,
        HSTRING path,
        __x_ABI_CWindows_CAI_CActions_CIFileActionEntity** result);
    HRESULT (STDMETHODCALLTYPE* CreateDocumentEntity)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2* This,
        HSTRING path,
        __x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity** result);
    HRESULT (STDMETHODCALLTYPE* CreatePhotoEntity)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2* This,
        HSTRING path,
        __x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity** result);
    HRESULT (STDMETHODCALLTYPE* CreateTextEntity)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2* This,
        HSTRING text,
        __x_ABI_CWindows_CAI_CActions_CITextActionEntity** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2Vtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_CreateFileEntity(This, path, result) \
    ((This)->lpVtbl->CreateFileEntity(This, path, result))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_CreateDocumentEntity(This, path, result) \
    ((This)->lpVtbl->CreateDocumentEntity(This, path, result))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_CreatePhotoEntity(This, path, result) \
    ((This)->lpVtbl->CreatePhotoEntity(This, path, result))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_CreateTextEntity(This, text, result) \
    ((This)->lpVtbl->CreateTextEntity(This, text, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IActionEntityFactory3
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntityFactory
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityFactory3[] = L"Windows.AI.Actions.IActionEntityFactory3";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateRemoteFileEntity)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3* This,
        HSTRING sourceId,
        enum __x_ABI_CWindows_CAI_CActions_CRemoteFileKind fileKind,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* sourceUri,
        HSTRING fileId,
        HSTRING contentType,
        HSTRING driveId,
        HSTRING accountId,
        HSTRING extension,
        __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity** result);
    HRESULT (STDMETHODCALLTYPE* CreateTextEntityWithTextFormat)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3* This,
        HSTRING text,
        enum __x_ABI_CWindows_CAI_CActions_CActionEntityTextFormat textFormat,
        __x_ABI_CWindows_CAI_CActions_CITextActionEntity** result);
    HRESULT (STDMETHODCALLTYPE* CreateStreamingTextActionEntityWriter)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3* This,
        enum __x_ABI_CWindows_CAI_CActions_CActionEntityTextFormat textFormat,
        __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3Vtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_CreateRemoteFileEntity(This, sourceId, fileKind, sourceUri, fileId, contentType, driveId, accountId, extension, result) \
    ((This)->lpVtbl->CreateRemoteFileEntity(This, sourceId, fileKind, sourceUri, fileId, contentType, driveId, accountId, extension, result))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_CreateTextEntityWithTextFormat(This, text, textFormat, result) \
    ((This)->lpVtbl->CreateTextEntityWithTextFormat(This, text, textFormat, result))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_CreateStreamingTextActionEntityWriter(This, textFormat, result) \
    ((This)->lpVtbl->CreateStreamingTextActionEntityWriter(This, textFormat, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory3_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IActionEntityFactory4
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntityFactory
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityFactory4[] = L"Windows.AI.Actions.IActionEntityFactory4";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateTableEntity)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4* This,
        UINT32 dataLength,
        HSTRING* data,
        UINT32 columnCount,
        __x_ABI_CWindows_CAI_CActions_CITableActionEntity** result);
    HRESULT (STDMETHODCALLTYPE* CreateContactEntity)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContact* contact,
        __x_ABI_CWindows_CAI_CActions_CIContactActionEntity** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4Vtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_CreateTableEntity(This, dataLength, data, columnCount, result) \
    ((This)->lpVtbl->CreateTableEntity(This, dataLength, data, columnCount, result))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_CreateContactEntity(This, contact, result) \
    ((This)->lpVtbl->CreateContactEntity(This, contact, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory4_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.AI.Actions.IActionEntityFactory5
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 6.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntityFactory
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityFactory5[] = L"Windows.AI.Actions.IActionEntityFactory5";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateUriEntity)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* Uri,
        __x_ABI_CWindows_CAI_CActions_CIUriActionEntity** result);
    HRESULT (STDMETHODCALLTYPE* CreateArrayEntity)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5* This,
        enum __x_ABI_CWindows_CAI_CActions_CActionEntityKind kind,
        UINT32 entitiesLength,
        __x_ABI_CWindows_CAI_CActions_CIActionEntity** entities,
        __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5Vtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_CreateUriEntity(This, Uri, result) \
    ((This)->lpVtbl->CreateUriEntity(This, Uri, result))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_CreateArrayEntity(This, kind, entitiesLength, entities, result) \
    ((This)->lpVtbl->CreateArrayEntity(This, kind, entitiesLength, entities, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory5_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IActionEntityFactory6
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 7.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntityFactory
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityFactory6[] = L"Windows.AI.Actions.IActionEntityFactory6";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateDateTimeEntity)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime dateTime,
        __x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity** result);
    HRESULT (STDMETHODCALLTYPE* CreateAppointmentEntity)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6* This,
        HSTRING sourceId,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        UINT32 attendeesLength,
        __x_ABI_CWindows_CAI_CActions_CIContactActionEntity** attendees,
        __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6Vtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_CreateDateTimeEntity(This, dateTime, result) \
    ((This)->lpVtbl->CreateDateTimeEntity(This, dateTime, result))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_CreateAppointmentEntity(This, sourceId, appointment, attendeesLength, attendees, result) \
    ((This)->lpVtbl->CreateAppointmentEntity(This, sourceId, appointment, attendeesLength, attendees, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory6_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IActionEntityFactory7
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntityFactory
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityFactory7[] = L"Windows.AI.Actions.IActionEntityFactory7";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateCustomTextEntity)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7* This,
        HSTRING kind,
        HSTRING keyPhrase,
        __FIMapView_2_HSTRING_IInspectable* props,
        __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity** result);
    HRESULT (STDMETHODCALLTYPE* CreateArrayEntityWithCustomKind)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7* This,
        enum __x_ABI_CWindows_CAI_CActions_CActionEntityKind elementKind,
        HSTRING customKind,
        UINT32 entitiesLength,
        __x_ABI_CWindows_CAI_CActions_CIActionEntity** entities,
        __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7Vtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_CreateCustomTextEntity(This, kind, keyPhrase, props, result) \
    ((This)->lpVtbl->CreateCustomTextEntity(This, kind, keyPhrase, props, result))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_CreateArrayEntityWithCustomKind(This, elementKind, customKind, entitiesLength, entities, result) \
    ((This)->lpVtbl->CreateArrayEntityWithCustomKind(This, elementKind, customKind, entitiesLength, entities, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactory7_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IActionEntityFactoryFactory
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionEntityFactory
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionEntityFactoryFactory[] = L"Windows.AI.Actions.IActionEntityFactoryFactory";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactoryVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionEntityFactoryFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IActionFeedback
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionFeedback
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionFeedback_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionFeedback_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionFeedback[] = L"Windows.AI.Actions.IActionFeedback";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionFeedbackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionFeedback* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionFeedback* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionFeedback* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionFeedback* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionFeedback* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionFeedback* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FeedbackKind)(__x_ABI_CWindows_CAI_CActions_CIActionFeedback* This,
        enum __x_ABI_CWindows_CAI_CActions_CActionFeedbackKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionFeedbackVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionFeedback
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionFeedbackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionFeedback_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionFeedback_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionFeedback_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionFeedback_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionFeedback_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionFeedback_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionFeedback_get_FeedbackKind(This, value) \
    ((This)->lpVtbl->get_FeedbackKind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionFeedback;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionFeedback_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IActionInvocationContext
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionInvocationContext
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionInvocationContext[] = L"Windows.AI.Actions.IActionInvocationContext";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionInvocationContextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EntityFactory)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* This,
        __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2** value);
    HRESULT (STDMETHODCALLTYPE* SetInputEntity)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* This,
        HSTRING inputName,
        __x_ABI_CWindows_CAI_CActions_CIActionEntity* inputValue);
    HRESULT (STDMETHODCALLTYPE* GetInputEntities)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* This,
        UINT32* resultLength,
        __x_ABI_CWindows_CAI_CActions_CINamedActionEntity*** result);
    HRESULT (STDMETHODCALLTYPE* SetOutputEntity)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* This,
        HSTRING outputName,
        __x_ABI_CWindows_CAI_CActions_CIActionEntity* outputValue);
    HRESULT (STDMETHODCALLTYPE* GetOutputEntities)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* This,
        UINT32* resultLength,
        __x_ABI_CWindows_CAI_CActions_CINamedActionEntity*** result);
    HRESULT (STDMETHODCALLTYPE* get_Result)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* This,
        enum __x_ABI_CWindows_CAI_CActions_CActionInvocationResult* value);
    HRESULT (STDMETHODCALLTYPE* put_Result)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* This,
        enum __x_ABI_CWindows_CAI_CActions_CActionInvocationResult value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* put_ExtendedError)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext* This,
        HRESULT value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionInvocationContextVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionInvocationContextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_get_EntityFactory(This, value) \
    ((This)->lpVtbl->get_EntityFactory(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_SetInputEntity(This, inputName, inputValue) \
    ((This)->lpVtbl->SetInputEntity(This, inputName, inputValue))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_GetInputEntities(This, resultLength, result) \
    ((This)->lpVtbl->GetInputEntities(This, resultLength, result))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_SetOutputEntity(This, outputName, outputValue) \
    ((This)->lpVtbl->SetOutputEntity(This, outputName, outputValue))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_GetOutputEntities(This, resultLength, result) \
    ((This)->lpVtbl->GetOutputEntities(This, resultLength, result))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_get_Result(This, value) \
    ((This)->lpVtbl->get_Result(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_put_Result(This, value) \
    ((This)->lpVtbl->put_Result(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_put_ExtendedError(This, value) \
    ((This)->lpVtbl->put_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionInvocationContext;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IActionInvocationContext2
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionInvocationContext
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionInvocationContext2[] = L"Windows.AI.Actions.IActionInvocationContext2";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InvokerWindowId)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2* This,
        struct __x_ABI_CWindows_CUI_CWindowId* value);
    HRESULT (STDMETHODCALLTYPE* get_HelpDetails)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2* This,
        __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails** value);
    HRESULT (STDMETHODCALLTYPE* get_ActionId)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_InvokerAppUserModelId)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2Vtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_get_InvokerWindowId(This, value) \
    ((This)->lpVtbl->get_InvokerWindowId(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_get_HelpDetails(This, value) \
    ((This)->lpVtbl->get_HelpDetails(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_get_ActionId(This, value) \
    ((This)->lpVtbl->get_ActionId(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_get_InvokerAppUserModelId(This, value) \
    ((This)->lpVtbl->get_InvokerAppUserModelId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionInvocationContext2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IActionInvocationHelpDetails
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionInvocationHelpDetails
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionInvocationHelpDetails[] = L"Windows.AI.Actions.IActionInvocationHelpDetails";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* This,
        enum __x_ABI_CWindows_CAI_CActions_CActionInvocationHelpKind* value);
    HRESULT (STDMETHODCALLTYPE* put_Kind)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* This,
        enum __x_ABI_CWindows_CAI_CActions_CActionInvocationHelpKind value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_HelpUri)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_HelpUri)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_HelpUriDescription)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_HelpUriDescription)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetailsVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_put_Kind(This, value) \
    ((This)->lpVtbl->put_Kind(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_get_HelpUri(This, value) \
    ((This)->lpVtbl->get_HelpUri(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_put_HelpUri(This, value) \
    ((This)->lpVtbl->put_HelpUri(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_get_HelpUriDescription(This, value) \
    ((This)->lpVtbl->get_HelpUriDescription(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_put_HelpUriDescription(This, value) \
    ((This)->lpVtbl->put_HelpUriDescription(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IActionInvocationHelpDetails2
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionInvocationHelpDetails
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionInvocationHelpDetails2[] = L"Windows.AI.Actions.IActionInvocationHelpDetails2";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_Changed)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2* This,
        __FITypedEventHandler_2_Windows__CAI__CActions__CActionInvocationHelpDetails_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Changed)(__x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2Vtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_add_Changed(This, handler, token) \
    ((This)->lpVtbl->add_Changed(This, handler, token))

#define __x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_remove_Changed(This, token) \
    ((This)->lpVtbl->remove_Changed(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionInvocationHelpDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IActionRuntime
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionRuntime
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionRuntime[] = L"Windows.AI.Actions.IActionRuntime";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionRuntimeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ActionCatalog)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime* This,
        __x_ABI_CWindows_CAI_CActions_CHosting_CIActionCatalog** value);
    HRESULT (STDMETHODCALLTYPE* get_EntityFactory)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime* This,
        __x_ABI_CWindows_CAI_CActions_CIActionEntityFactory2** value);
    HRESULT (STDMETHODCALLTYPE* CreateInvocationContext)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime* This,
        HSTRING actionId,
        __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionRuntimeVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionRuntime
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionRuntimeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime_get_ActionCatalog(This, value) \
    ((This)->lpVtbl->get_ActionCatalog(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime_get_EntityFactory(This, value) \
    ((This)->lpVtbl->get_EntityFactory(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime_CreateInvocationContext(This, actionId, result) \
    ((This)->lpVtbl->CreateInvocationContext(This, actionId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionRuntime;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IActionRuntime2
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionRuntime
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionRuntime2[] = L"Windows.AI.Actions.IActionRuntime2";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionRuntime2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateActionFeedback)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime2* This,
        enum __x_ABI_CWindows_CAI_CActions_CActionFeedbackKind feedbackKind,
        __x_ABI_CWindows_CAI_CActions_CIActionFeedback** result);
    HRESULT (STDMETHODCALLTYPE* SetActionAvailability)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime2* This,
        HSTRING actionId,
        boolean isAvailable);
    HRESULT (STDMETHODCALLTYPE* GetActionAvailability)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime2* This,
        HSTRING actionId,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionRuntime2Vtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionRuntime2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionRuntime2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime2_CreateActionFeedback(This, feedbackKind, result) \
    ((This)->lpVtbl->CreateActionFeedback(This, feedbackKind, result))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime2_SetActionAvailability(This, actionId, isAvailable) \
    ((This)->lpVtbl->SetActionAvailability(This, actionId, isAvailable))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime2_GetActionAvailability(This, actionId, result) \
    ((This)->lpVtbl->GetActionAvailability(This, actionId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionRuntime2;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IActionRuntime3
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionRuntime
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionRuntime3[] = L"Windows.AI.Actions.IActionRuntime3";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionRuntime3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInvocationContextWithWindowId)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime3* This,
        HSTRING actionId,
        struct __x_ABI_CWindows_CUI_CWindowId invokerWindowId,
        __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext** result);
    HRESULT (STDMETHODCALLTYPE* GetActionEntityById)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime3* This,
        HSTRING entityId,
        __x_ABI_CWindows_CAI_CActions_CIActionEntity** result);
    HRESULT (STDMETHODCALLTYPE* get_LatestSupportedSchemaVersion)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime3* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionRuntime3Vtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionRuntime3
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionRuntime3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime3_CreateInvocationContextWithWindowId(This, actionId, invokerWindowId, result) \
    ((This)->lpVtbl->CreateInvocationContextWithWindowId(This, actionId, invokerWindowId, result))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime3_GetActionEntityById(This, entityId, result) \
    ((This)->lpVtbl->GetActionEntityById(This, entityId, result))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime3_get_LatestSupportedSchemaVersion(This, value) \
    ((This)->lpVtbl->get_LatestSupportedSchemaVersion(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionRuntime3;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime3_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.AI.Actions.IActionRuntime4
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionRuntime
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionRuntime4[] = L"Windows.AI.Actions.IActionRuntime4";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionRuntime4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetActionInvocationContextFromToken)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime4* This,
        HSTRING token,
        __x_ABI_CWindows_CAI_CActions_CIActionInvocationContext** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionRuntime4Vtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionRuntime4
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionRuntime4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime4_GetActionInvocationContextFromToken(This, token, result) \
    ((This)->lpVtbl->GetActionInvocationContextFromToken(This, token, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionRuntime4;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime4_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.AI.Actions.IActionRuntime5
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionRuntime
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntime5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionRuntime5[] = L"Windows.AI.Actions.IActionRuntime5";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionRuntime5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CustomEntityStore)(__x_ABI_CWindows_CAI_CActions_CIActionRuntime5* This,
        __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionRuntime5Vtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionRuntime5
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionRuntime5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntime5_get_CustomEntityStore(This, value) \
    ((This)->lpVtbl->get_CustomEntityStore(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionRuntime5;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntime5_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IActionRuntimeFactory
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionRuntime
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionRuntimeFactory[] = L"Windows.AI.Actions.IActionRuntimeFactory";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactoryVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntimeFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IActionRuntimeStatics
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ActionRuntime
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IActionRuntimeStatics[] = L"Windows.AI.Actions.IActionRuntimeStatics";
typedef struct __x_ABI_CWindows_CAI_CActions_CIActionRuntimeStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics* This,
        __x_ABI_CWindows_CAI_CActions_CIActionRuntime** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIActionRuntimeStaticsVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIActionRuntimeStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIActionRuntimeStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.AI.Actions.IAppointmentActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 7.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.AppointmentActionEntity
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IAppointmentActionEntity[] = L"Windows.AI.Actions.IAppointmentActionEntity";
typedef struct __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourceId)(__x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Appointment)(__x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment** value);
    HRESULT (STDMETHODCALLTYPE* GetAttendees)(__x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity* This,
        UINT32* resultLength,
        __x_ABI_CWindows_CAI_CActions_CIContactActionEntity*** result);
    HRESULT (STDMETHODCALLTYPE* GetPresentedFiles)(__x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity* This,
        UINT32* resultLength,
        __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity*** result);
    HRESULT (STDMETHODCALLTYPE* SetPresentedFiles)(__x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity* This,
        UINT32 filesLength,
        __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity** files);
    HRESULT (STDMETHODCALLTYPE* GetSharedFiles)(__x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity* This,
        UINT32* resultLength,
        __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity*** result);
    HRESULT (STDMETHODCALLTYPE* SetSharedFiles)(__x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity* This,
        UINT32 filesLength,
        __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity** files);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntityVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_get_SourceId(This, value) \
    ((This)->lpVtbl->get_SourceId(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_get_Appointment(This, value) \
    ((This)->lpVtbl->get_Appointment(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_GetAttendees(This, resultLength, result) \
    ((This)->lpVtbl->GetAttendees(This, resultLength, result))

#define __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_GetPresentedFiles(This, resultLength, result) \
    ((This)->lpVtbl->GetPresentedFiles(This, resultLength, result))

#define __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_SetPresentedFiles(This, filesLength, files) \
    ((This)->lpVtbl->SetPresentedFiles(This, filesLength, files))

#define __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_GetSharedFiles(This, resultLength, result) \
    ((This)->lpVtbl->GetSharedFiles(This, resultLength, result))

#define __x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_SetSharedFiles(This, filesLength, files) \
    ((This)->lpVtbl->SetSharedFiles(This, filesLength, files))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIAppointmentActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IArrayActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 6.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ArrayActionEntity
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IArrayActionEntity[] = L"Windows.AI.Actions.IArrayActionEntity";
typedef struct __x_ABI_CWindows_CAI_CActions_CIArrayActionEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIArrayActionEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIArrayActionEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIArrayActionEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIArrayActionEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIArrayActionEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIArrayActionEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ElementKind)(__x_ABI_CWindows_CAI_CActions_CIArrayActionEntity* This,
        enum __x_ABI_CWindows_CAI_CActions_CActionEntityKind* value);
    HRESULT (STDMETHODCALLTYPE* GetAll)(__x_ABI_CWindows_CAI_CActions_CIArrayActionEntity* This,
        UINT32* resultLength,
        __x_ABI_CWindows_CAI_CActions_CIActionEntity*** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIArrayActionEntityVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIArrayActionEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_get_ElementKind(This, value) \
    ((This)->lpVtbl->get_ElementKind(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_GetAll(This, resultLength, result) \
    ((This)->lpVtbl->GetAll(This, resultLength, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIArrayActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IArrayActionEntity2
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ArrayActionEntity
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IArrayActionEntity2[] = L"Windows.AI.Actions.IArrayActionEntity2";
typedef struct __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CustomElementKind)(__x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2Vtbl;

interface __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_get_CustomElementKind(This, value) \
    ((This)->lpVtbl->get_CustomElementKind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIArrayActionEntity2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IContactActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.ContactActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIContactActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIContactActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IContactActionEntity[] = L"Windows.AI.Actions.IContactActionEntity";
typedef struct __x_ABI_CWindows_CAI_CActions_CIContactActionEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIContactActionEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIContactActionEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIContactActionEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIContactActionEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIContactActionEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIContactActionEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CAI_CActions_CIContactActionEntity* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContact** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIContactActionEntityVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIContactActionEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIContactActionEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIContactActionEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIContactActionEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIContactActionEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIContactActionEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIContactActionEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIContactActionEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIContactActionEntity_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIContactActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIContactActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.AI.Actions.ICustomActionEntityStore
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.CustomActionEntityStore
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_ICustomActionEntityStore[] = L"Windows.AI.Actions.ICustomActionEntityStore";
typedef struct __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetLastModifiedTime)(__x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore* This,
        HSTRING kind,
        struct __x_ABI_CWindows_CFoundation_CDateTime* result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore* This,
        __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity* entity);
    HRESULT (STDMETHODCALLTYPE* InsertMany)(__x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore* This,
        UINT32 entitiesLength,
        __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity** entities);
    HRESULT (STDMETHODCALLTYPE* Delete)(__x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore* This,
        HSTRING kind);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreVtbl;

interface __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_GetLastModifiedTime(This, kind, result) \
    ((This)->lpVtbl->GetLastModifiedTime(This, kind, result))

#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_Insert(This, entity) \
    ((This)->lpVtbl->Insert(This, entity))

#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_InsertMany(This, entitiesLength, entities) \
    ((This)->lpVtbl->InsertMany(This, entitiesLength, entities))

#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_Delete(This, kind) \
    ((This)->lpVtbl->Delete(This, kind))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.ICustomActionEntityStoreFactory
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.CustomActionEntityStore
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_ICustomActionEntityStoreFactory[] = L"Windows.AI.Actions.ICustomActionEntityStoreFactory";
typedef struct __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactoryVtbl;

interface __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CICustomActionEntityStoreFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.ICustomTextActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.CustomTextActionEntity
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_ICustomTextActionEntity[] = L"Windows.AI.Actions.ICustomTextActionEntity";
typedef struct __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CustomTextKind)(__x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_KeyPhrase)(__x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity* This,
        __FIMapView_2_HSTRING_IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntityVtbl;

interface __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_get_CustomTextKind(This, value) \
    ((This)->lpVtbl->get_CustomTextKind(This, value))

#define __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_get_KeyPhrase(This, value) \
    ((This)->lpVtbl->get_KeyPhrase(This, value))

#define __x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CICustomTextActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IDateTimeActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 7.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.DateTimeActionEntity
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IDateTimeActionEntity[] = L"Windows.AI.Actions.IDateTimeActionEntity";
typedef struct __x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DateTime)(__x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntityVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_get_DateTime(This, value) \
    ((This)->lpVtbl->get_DateTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIDateTimeActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IDocumentActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.DocumentActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IDocumentActionEntity[] = L"Windows.AI.Actions.IDocumentActionEntity";
typedef struct __x_ABI_CWindows_CAI_CActions_CIDocumentActionEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FullPath)(__x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIDocumentActionEntityVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIDocumentActionEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_get_FullPath(This, value) \
    ((This)->lpVtbl->get_FullPath(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIDocumentActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IFileActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.FileActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIFileActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIFileActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IFileActionEntity[] = L"Windows.AI.Actions.IFileActionEntity";
typedef struct __x_ABI_CWindows_CAI_CActions_CIFileActionEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIFileActionEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIFileActionEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIFileActionEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIFileActionEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIFileActionEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIFileActionEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FullPath)(__x_ABI_CWindows_CAI_CActions_CIFileActionEntity* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIFileActionEntityVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIFileActionEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIFileActionEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIFileActionEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIFileActionEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIFileActionEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIFileActionEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIFileActionEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIFileActionEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIFileActionEntity_get_FullPath(This, value) \
    ((This)->lpVtbl->get_FullPath(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIFileActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIFileActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.INamedActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.NamedActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CINamedActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CINamedActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_INamedActionEntity[] = L"Windows.AI.Actions.INamedActionEntity";
typedef struct __x_ABI_CWindows_CAI_CActions_CINamedActionEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CINamedActionEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CINamedActionEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CINamedActionEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CINamedActionEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CINamedActionEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CINamedActionEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CAI_CActions_CINamedActionEntity* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Name)(__x_ABI_CWindows_CAI_CActions_CINamedActionEntity* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Entity)(__x_ABI_CWindows_CAI_CActions_CINamedActionEntity* This,
        __x_ABI_CWindows_CAI_CActions_CIActionEntity** value);
    HRESULT (STDMETHODCALLTYPE* put_Entity)(__x_ABI_CWindows_CAI_CActions_CINamedActionEntity* This,
        __x_ABI_CWindows_CAI_CActions_CIActionEntity* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CINamedActionEntityVtbl;

interface __x_ABI_CWindows_CAI_CActions_CINamedActionEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CINamedActionEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CINamedActionEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CINamedActionEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CINamedActionEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CINamedActionEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CINamedActionEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CINamedActionEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CINamedActionEntity_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CAI_CActions_CINamedActionEntity_put_Name(This, value) \
    ((This)->lpVtbl->put_Name(This, value))

#define __x_ABI_CWindows_CAI_CActions_CINamedActionEntity_get_Entity(This, value) \
    ((This)->lpVtbl->get_Entity(This, value))

#define __x_ABI_CWindows_CAI_CActions_CINamedActionEntity_put_Entity(This, value) \
    ((This)->lpVtbl->put_Entity(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CINamedActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CINamedActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IPhotoActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.PhotoActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IPhotoActionEntity[] = L"Windows.AI.Actions.IPhotoActionEntity";
typedef struct __x_ABI_CWindows_CAI_CActions_CIPhotoActionEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FullPath)(__x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIPhotoActionEntityVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIPhotoActionEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_get_FullPath(This, value) \
    ((This)->lpVtbl->get_FullPath(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIPhotoActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.IRemoteFileActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.RemoteFileActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IRemoteFileActionEntity[] = L"Windows.AI.Actions.IRemoteFileActionEntity";
typedef struct __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourceId)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FileKind)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity* This,
        enum __x_ABI_CWindows_CAI_CActions_CRemoteFileKind* value);
    HRESULT (STDMETHODCALLTYPE* get_SourceUri)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_FileId)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ContentType)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DriveId)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AccountId)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Extension)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntityVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_get_SourceId(This, value) \
    ((This)->lpVtbl->get_SourceId(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_get_FileKind(This, value) \
    ((This)->lpVtbl->get_FileKind(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_get_SourceUri(This, value) \
    ((This)->lpVtbl->get_SourceUri(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_get_FileId(This, value) \
    ((This)->lpVtbl->get_FileId(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_get_ContentType(This, value) \
    ((This)->lpVtbl->get_ContentType(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_get_DriveId(This, value) \
    ((This)->lpVtbl->get_DriveId(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_get_AccountId(This, value) \
    ((This)->lpVtbl->get_AccountId(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_get_Extension(This, value) \
    ((This)->lpVtbl->get_Extension(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IRemoteFileActionEntity2
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 7.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.RemoteFileActionEntity
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IRemoteFileActionEntity2[] = L"Windows.AI.Actions.IRemoteFileActionEntity2";
typedef struct __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Filename)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Filename)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Creator)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2* This,
        __x_ABI_CWindows_CAI_CActions_CIContactActionEntity** value);
    HRESULT (STDMETHODCALLTYPE* put_Creator)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2* This,
        __x_ABI_CWindows_CAI_CActions_CIContactActionEntity* value);
    HRESULT (STDMETHODCALLTYPE* get_LastUpdatedTime)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_LastUpdatedTime)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* SetContributors)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2* This,
        UINT32 contributorsLength,
        __x_ABI_CWindows_CAI_CActions_CIContactActionEntity** contributors);
    HRESULT (STDMETHODCALLTYPE* GetContributors)(__x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2* This,
        UINT32* resultLength,
        __x_ABI_CWindows_CAI_CActions_CIContactActionEntity*** result);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2Vtbl;

interface __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_get_Filename(This, value) \
    ((This)->lpVtbl->get_Filename(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_put_Filename(This, value) \
    ((This)->lpVtbl->put_Filename(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_get_Creator(This, value) \
    ((This)->lpVtbl->get_Creator(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_put_Creator(This, value) \
    ((This)->lpVtbl->put_Creator(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_get_LastUpdatedTime(This, value) \
    ((This)->lpVtbl->get_LastUpdatedTime(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_put_LastUpdatedTime(This, value) \
    ((This)->lpVtbl->put_LastUpdatedTime(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_SetContributors(This, contributorsLength, contributors) \
    ((This)->lpVtbl->SetContributors(This, contributorsLength, contributors))

#define __x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_GetContributors(This, resultLength, result) \
    ((This)->lpVtbl->GetContributors(This, resultLength, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIRemoteFileActionEntity2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Interface Windows.AI.Actions.IStreamingTextActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.StreamingTextActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IStreamingTextActionEntity[] = L"Windows.AI.Actions.IStreamingTextActionEntity";
typedef struct __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsComplete)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* GetText)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_TextFormat)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity* This,
        enum __x_ABI_CWindows_CAI_CActions_CActionEntityTextFormat* value);
    HRESULT (STDMETHODCALLTYPE* add_TextChanged)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity* This,
        __FITypedEventHandler_2_Windows__CAI__CActions__CStreamingTextActionEntity_Windows__CAI__CActions__CStreamingTextActionEntityTextChangedArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TextChanged)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_get_IsComplete(This, value) \
    ((This)->lpVtbl->get_IsComplete(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_GetText(This, result) \
    ((This)->lpVtbl->GetText(This, result))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_get_TextFormat(This, value) \
    ((This)->lpVtbl->get_TextFormat(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_add_TextChanged(This, handler, token) \
    ((This)->lpVtbl->add_TextChanged(This, handler, token))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_remove_TextChanged(This, token) \
    ((This)->lpVtbl->remove_TextChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IStreamingTextActionEntityTextChangedArgs
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.StreamingTextActionEntityTextChangedArgs
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IStreamingTextActionEntityTextChangedArgs[] = L"Windows.AI.Actions.IStreamingTextActionEntityTextChangedArgs";
typedef struct __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsComplete)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgsVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_get_IsComplete(This, value) \
    ((This)->lpVtbl->get_IsComplete(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityTextChangedArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IStreamingTextActionEntityWriter
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.StreamingTextActionEntityWriter
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IStreamingTextActionEntityWriter[] = L"Windows.AI.Actions.IStreamingTextActionEntityWriter";
typedef struct __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ReaderEntity)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter* This,
        __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntity** value);
    HRESULT (STDMETHODCALLTYPE* get_TextFormat)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter* This,
        enum __x_ABI_CWindows_CAI_CActions_CActionEntityTextFormat* value);
    HRESULT (STDMETHODCALLTYPE* SetText)(__x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter* This,
        HSTRING text);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriterVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_get_ReaderEntity(This, value) \
    ((This)->lpVtbl->get_ReaderEntity(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_get_TextFormat(This, value) \
    ((This)->lpVtbl->get_TextFormat(This, value))

#define __x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_SetText(This, text) \
    ((This)->lpVtbl->SetText(This, text))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIStreamingTextActionEntityWriter_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.ITableActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.TableActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CAI_CActions_CITableActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CITableActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_ITableActionEntity[] = L"Windows.AI.Actions.ITableActionEntity";
typedef struct __x_ABI_CWindows_CAI_CActions_CITableActionEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CITableActionEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CITableActionEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CITableActionEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CITableActionEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CITableActionEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CITableActionEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetTextContent)(__x_ABI_CWindows_CAI_CActions_CITableActionEntity* This,
        UINT32* resultLength,
        HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_RowCount)(__x_ABI_CWindows_CAI_CActions_CITableActionEntity* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ColumnCount)(__x_ABI_CWindows_CAI_CActions_CITableActionEntity* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CITableActionEntityVtbl;

interface __x_ABI_CWindows_CAI_CActions_CITableActionEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CITableActionEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CITableActionEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CITableActionEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CITableActionEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CITableActionEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CITableActionEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CITableActionEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CITableActionEntity_GetTextContent(This, resultLength, result) \
    ((This)->lpVtbl->GetTextContent(This, resultLength, result))

#define __x_ABI_CWindows_CAI_CActions_CITableActionEntity_get_RowCount(This, value) \
    ((This)->lpVtbl->get_RowCount(This, value))

#define __x_ABI_CWindows_CAI_CActions_CITableActionEntity_get_ColumnCount(This, value) \
    ((This)->lpVtbl->get_ColumnCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CITableActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CITableActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.AI.Actions.ITextActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.TextActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CAI_CActions_CITextActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CITextActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_ITextActionEntity[] = L"Windows.AI.Actions.ITextActionEntity";
typedef struct __x_ABI_CWindows_CAI_CActions_CITextActionEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CITextActionEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CITextActionEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CITextActionEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CITextActionEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CITextActionEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CITextActionEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CAI_CActions_CITextActionEntity* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CITextActionEntityVtbl;

interface __x_ABI_CWindows_CAI_CActions_CITextActionEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CITextActionEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CITextActionEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CITextActionEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CITextActionEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CITextActionEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CITextActionEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CITextActionEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CITextActionEntity_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CITextActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CITextActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.AI.Actions.ITextActionEntity2
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.TextActionEntity
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CAI_CActions_CITextActionEntity2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CITextActionEntity2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_ITextActionEntity2[] = L"Windows.AI.Actions.ITextActionEntity2";
typedef struct __x_ABI_CWindows_CAI_CActions_CITextActionEntity2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CITextActionEntity2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CITextActionEntity2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CITextActionEntity2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CITextActionEntity2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CITextActionEntity2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CITextActionEntity2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TextFormat)(__x_ABI_CWindows_CAI_CActions_CITextActionEntity2* This,
        enum __x_ABI_CWindows_CAI_CActions_CActionEntityTextFormat* value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CITextActionEntity2Vtbl;

interface __x_ABI_CWindows_CAI_CActions_CITextActionEntity2
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CITextActionEntity2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CITextActionEntity2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CITextActionEntity2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CITextActionEntity2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CITextActionEntity2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CITextActionEntity2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CITextActionEntity2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CITextActionEntity2_get_TextFormat(This, value) \
    ((This)->lpVtbl->get_TextFormat(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CITextActionEntity2;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CITextActionEntity2_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.AI.Actions.IUriActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 6.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Interface is a part of the implementation of type Windows.AI.Actions.UriActionEntity
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CAI_CActions_CIUriActionEntity_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CAI_CActions_CIUriActionEntity_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_AI_Actions_IUriActionEntity[] = L"Windows.AI.Actions.IUriActionEntity";
typedef struct __x_ABI_CWindows_CAI_CActions_CIUriActionEntityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CAI_CActions_CIUriActionEntity* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CAI_CActions_CIUriActionEntity* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CAI_CActions_CIUriActionEntity* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CAI_CActions_CIUriActionEntity* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CAI_CActions_CIUriActionEntity* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CAI_CActions_CIUriActionEntity* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CAI_CActions_CIUriActionEntity* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CAI_CActions_CIUriActionEntityVtbl;

interface __x_ABI_CWindows_CAI_CActions_CIUriActionEntity
{
    CONST_VTBL struct __x_ABI_CWindows_CAI_CActions_CIUriActionEntityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CAI_CActions_CIUriActionEntity_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CAI_CActions_CIUriActionEntity_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CAI_CActions_CIUriActionEntity_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CAI_CActions_CIUriActionEntity_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CAI_CActions_CIUriActionEntity_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CAI_CActions_CIUriActionEntity_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CAI_CActions_CIUriActionEntity_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CAI_CActions_CIUriActionEntity;
#endif /* !defined(____x_ABI_CWindows_CAI_CActions_CIUriActionEntity_INTERFACE_DEFINED__) */
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.AI.Actions.ActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IActionEntity ** Default Interface **
 *    Windows.AI.Actions.IActionEntity2
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ActionEntity[] = L"Windows.AI.Actions.ActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.ActionEntityDisplayInfo
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IActionEntityDisplayInfo ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ActionEntityDisplayInfo_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ActionEntityDisplayInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ActionEntityDisplayInfo[] = L"Windows.AI.Actions.ActionEntityDisplayInfo";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.ActionEntityFactory
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IActionEntityFactory2 ** Default Interface **
 *    Windows.AI.Actions.IActionEntityFactory3
 *    Windows.AI.Actions.IActionEntityFactory4
 *    Windows.AI.Actions.IActionEntityFactory5
 *    Windows.AI.Actions.IActionEntityFactory6
 *    Windows.AI.Actions.IActionEntityFactory7
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ActionEntityFactory_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ActionEntityFactory_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ActionEntityFactory[] = L"Windows.AI.Actions.ActionEntityFactory";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.ActionFeedback
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IActionFeedback ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ActionFeedback_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ActionFeedback_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ActionFeedback[] = L"Windows.AI.Actions.ActionFeedback";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.AI.Actions.ActionInvocationContext
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IActionInvocationContext ** Default Interface **
 *    Windows.AI.Actions.IActionInvocationContext2
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ActionInvocationContext_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ActionInvocationContext_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ActionInvocationContext[] = L"Windows.AI.Actions.ActionInvocationContext";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.ActionInvocationHelpDetails
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IActionInvocationHelpDetails ** Default Interface **
 *    Windows.AI.Actions.IActionInvocationHelpDetails2
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ActionInvocationHelpDetails_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ActionInvocationHelpDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ActionInvocationHelpDetails[] = L"Windows.AI.Actions.ActionInvocationHelpDetails";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.AI.Actions.ActionRuntime
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.AI.Actions.IActionRuntimeStatics interface starting with version 8.0 of the Windows.AI.Actions.ActionsContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IActionRuntime ** Default Interface **
 *    Windows.AI.Actions.IActionRuntime2
 *    Windows.AI.Actions.IActionRuntime3
 *    Windows.AI.Actions.IActionRuntime4
 *    Windows.AI.Actions.IActionRuntime5
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ActionRuntime_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ActionRuntime_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ActionRuntime[] = L"Windows.AI.Actions.ActionRuntime";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.AppointmentActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 7.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IAppointmentActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_AI_Actions_AppointmentActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_AppointmentActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_AppointmentActionEntity[] = L"Windows.AI.Actions.AppointmentActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.AI.Actions.ArrayActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 6.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IArrayActionEntity ** Default Interface **
 *    Windows.AI.Actions.IArrayActionEntity2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ArrayActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ArrayActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ArrayActionEntity[] = L"Windows.AI.Actions.ArrayActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.AI.Actions.ContactActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IContactActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_AI_Actions_ContactActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_ContactActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_ContactActionEntity[] = L"Windows.AI.Actions.ContactActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.AI.Actions.CustomActionEntityStore
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.ICustomActionEntityStore ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_AI_Actions_CustomActionEntityStore_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_CustomActionEntityStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_CustomActionEntityStore[] = L"Windows.AI.Actions.CustomActionEntityStore";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.AI.Actions.CustomTextActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 8.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.ICustomTextActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#ifndef RUNTIMECLASS_Windows_AI_Actions_CustomTextActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_CustomTextActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_CustomTextActionEntity[] = L"Windows.AI.Actions.CustomTextActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x80000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.AI.Actions.DateTimeActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 7.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IDateTimeActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_AI_Actions_DateTimeActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_DateTimeActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_DateTimeActionEntity[] = L"Windows.AI.Actions.DateTimeActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x70000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

/*
 *
 * Class Windows.AI.Actions.DocumentActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IDocumentActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_DocumentActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_DocumentActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_DocumentActionEntity[] = L"Windows.AI.Actions.DocumentActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.FileActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IFileActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_FileActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_FileActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_FileActionEntity[] = L"Windows.AI.Actions.FileActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.NamedActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.INamedActionEntity ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_NamedActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_NamedActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_NamedActionEntity[] = L"Windows.AI.Actions.NamedActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.PhotoActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IPhotoActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_PhotoActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_PhotoActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_PhotoActionEntity[] = L"Windows.AI.Actions.PhotoActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.RemoteFileActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IRemoteFileActionEntity ** Default Interface **
 *    Windows.AI.Actions.IRemoteFileActionEntity2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_AI_Actions_RemoteFileActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_RemoteFileActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_RemoteFileActionEntity[] = L"Windows.AI.Actions.RemoteFileActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.AI.Actions.StreamingTextActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IStreamingTextActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_AI_Actions_StreamingTextActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_StreamingTextActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_StreamingTextActionEntity[] = L"Windows.AI.Actions.StreamingTextActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.AI.Actions.StreamingTextActionEntityTextChangedArgs
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IStreamingTextActionEntityTextChangedArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_AI_Actions_StreamingTextActionEntityTextChangedArgs_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_StreamingTextActionEntityTextChangedArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_StreamingTextActionEntityTextChangedArgs[] = L"Windows.AI.Actions.StreamingTextActionEntityTextChangedArgs";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.AI.Actions.StreamingTextActionEntityWriter
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IStreamingTextActionEntityWriter ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_AI_Actions_StreamingTextActionEntityWriter_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_StreamingTextActionEntityWriter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_StreamingTextActionEntityWriter[] = L"Windows.AI.Actions.StreamingTextActionEntityWriter";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.AI.Actions.TableActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.ITableActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_AI_Actions_TableActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_TableActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_TableActionEntity[] = L"Windows.AI.Actions.TableActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.AI.Actions.TextActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.ITextActionEntity ** Default Interface **
 *    Windows.AI.Actions.ITextActionEntity2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_AI_Actions_TextActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_TextActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_TextActionEntity[] = L"Windows.AI.Actions.TextActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.AI.Actions.UriActionEntity
 *
 * Introduced to Windows.AI.Actions.ActionsContract in version 6.0
 *
 * Type is for evaluation purposes and is subject to change or removal in future updates.
 *
 * Class implements the following interfaces:
 *    Windows.AI.Actions.IUriActionEntity ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)
#if WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_AI_Actions_UriActionEntity_DEFINED
#define RUNTIMECLASS_Windows_AI_Actions_UriActionEntity_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_AI_Actions_UriActionEntity[] = L"Windows.AI.Actions.UriActionEntity";
#endif
#endif // WINDOWS_AI_ACTIONS_ACTIONSCONTRACT_VERSION >= 0x60000
#endif // defined(ENABLE_WINRT_EXPERIMENTAL_TYPES)

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eai2Eactions_p_h__

#endif // __windows2Eai2Eactions_h__
