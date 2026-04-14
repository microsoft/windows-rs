
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
#ifndef __windows2Eui2Enotifications_h__
#define __windows2Eui2Enotifications_h__
#ifndef __windows2Eui2Enotifications_p_h__
#define __windows2Eui2Enotifications_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)
#define WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)
#define WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION 0x130000
#endif // defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)

#if !defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)
#define WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.ApplicationModel.h"
#include "Windows.Data.Xml.Dom.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IAdaptiveNotificationContent;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent ABI::Windows::UI::Notifications::IAdaptiveNotificationContent

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IAdaptiveNotificationText;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText ABI::Windows::UI::Notifications::IAdaptiveNotificationText

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IBadgeNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification ABI::Windows::UI::Notifications::IBadgeNotification

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IBadgeNotificationFactory;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory ABI::Windows::UI::Notifications::IBadgeNotificationFactory

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IBadgeUpdateManagerForUser;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser ABI::Windows::UI::Notifications::IBadgeUpdateManagerForUser

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IBadgeUpdateManagerStatics;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics ABI::Windows::UI::Notifications::IBadgeUpdateManagerStatics

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IBadgeUpdateManagerStatics2;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2 ABI::Windows::UI::Notifications::IBadgeUpdateManagerStatics2

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IBadgeUpdater;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater ABI::Windows::UI::Notifications::IBadgeUpdater

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IKnownAdaptiveNotificationHintsStatics;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics ABI::Windows::UI::Notifications::IKnownAdaptiveNotificationHintsStatics

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IKnownAdaptiveNotificationTextStylesStatics;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics ABI::Windows::UI::Notifications::IKnownAdaptiveNotificationTextStylesStatics

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IKnownNotificationBindingsStatics;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics ABI::Windows::UI::Notifications::IKnownNotificationBindingsStatics

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CINotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CINotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface INotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CINotification ABI::Windows::UI::Notifications::INotification

#endif // ____x_ABI_CWindows_CUI_CNotifications_CINotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface INotificationBinding;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding ABI::Windows::UI::Notifications::INotificationBinding

#endif // ____x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CINotificationData_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CINotificationData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface INotificationData;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CINotificationData ABI::Windows::UI::Notifications::INotificationData

#endif // ____x_ABI_CWindows_CUI_CNotifications_CINotificationData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface INotificationDataFactory;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory ABI::Windows::UI::Notifications::INotificationDataFactory

#endif // ____x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface INotificationVisual;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CINotificationVisual ABI::Windows::UI::Notifications::INotificationVisual

#endif // ____x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IScheduledTileNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification ABI::Windows::UI::Notifications::IScheduledTileNotification

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IScheduledTileNotificationFactory;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory ABI::Windows::UI::Notifications::IScheduledTileNotificationFactory

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IScheduledToastNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification ABI::Windows::UI::Notifications::IScheduledToastNotification

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IScheduledToastNotification2;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2 ABI::Windows::UI::Notifications::IScheduledToastNotification2

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IScheduledToastNotification3;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3 ABI::Windows::UI::Notifications::IScheduledToastNotification3

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IScheduledToastNotification4;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4 ABI::Windows::UI::Notifications::IScheduledToastNotification4

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IScheduledToastNotificationFactory;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory ABI::Windows::UI::Notifications::IScheduledToastNotificationFactory

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IScheduledToastNotificationShowingEventArgs;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs ABI::Windows::UI::Notifications::IScheduledToastNotificationShowingEventArgs

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IShownTileNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification ABI::Windows::UI::Notifications::IShownTileNotification

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface ITileFlyoutNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification ABI::Windows::UI::Notifications::ITileFlyoutNotification

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface ITileFlyoutNotificationFactory;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory ABI::Windows::UI::Notifications::ITileFlyoutNotificationFactory

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface ITileFlyoutUpdateManagerStatics;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics ABI::Windows::UI::Notifications::ITileFlyoutUpdateManagerStatics

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface ITileFlyoutUpdater;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater ABI::Windows::UI::Notifications::ITileFlyoutUpdater

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface ITileNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CITileNotification ABI::Windows::UI::Notifications::ITileNotification

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface ITileNotificationFactory;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory ABI::Windows::UI::Notifications::ITileNotificationFactory

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface ITileUpdateManagerForUser;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser ABI::Windows::UI::Notifications::ITileUpdateManagerForUser

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface ITileUpdateManagerStatics;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics ABI::Windows::UI::Notifications::ITileUpdateManagerStatics

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface ITileUpdateManagerStatics2;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2 ABI::Windows::UI::Notifications::ITileUpdateManagerStatics2

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface ITileUpdater;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater ABI::Windows::UI::Notifications::ITileUpdater

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface ITileUpdater2;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater2 ABI::Windows::UI::Notifications::ITileUpdater2

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastActivatedEventArgs;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs ABI::Windows::UI::Notifications::IToastActivatedEventArgs

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastActivatedEventArgs2;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2 ABI::Windows::UI::Notifications::IToastActivatedEventArgs2

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastCollection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastCollection;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollection ABI::Windows::UI::Notifications::IToastCollection

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastCollectionFactory;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory ABI::Windows::UI::Notifications::IToastCollectionFactory

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastCollectionManager;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager ABI::Windows::UI::Notifications::IToastCollectionManager

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastDismissedEventArgs;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs ABI::Windows::UI::Notifications::IToastDismissedEventArgs

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastFailedEventArgs;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs ABI::Windows::UI::Notifications::IToastFailedEventArgs

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification ABI::Windows::UI::Notifications::IToastNotification

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotification2;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2 ABI::Windows::UI::Notifications::IToastNotification2

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotification3;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3 ABI::Windows::UI::Notifications::IToastNotification3

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotification4;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4 ABI::Windows::UI::Notifications::IToastNotification4

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotification6;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification6 ABI::Windows::UI::Notifications::IToastNotification6

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotificationActionTriggerDetail;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail ABI::Windows::UI::Notifications::IToastNotificationActionTriggerDetail

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotificationFactory;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory ABI::Windows::UI::Notifications::IToastNotificationFactory

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotificationHistory;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory ABI::Windows::UI::Notifications::IToastNotificationHistory

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotificationHistory2;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2 ABI::Windows::UI::Notifications::IToastNotificationHistory2

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotificationHistoryChangedTriggerDetail;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail ABI::Windows::UI::Notifications::IToastNotificationHistoryChangedTriggerDetail

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotificationHistoryChangedTriggerDetail2;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2 ABI::Windows::UI::Notifications::IToastNotificationHistoryChangedTriggerDetail2

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotificationManagerForUser;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser ABI::Windows::UI::Notifications::IToastNotificationManagerForUser

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotificationManagerForUser2;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2 ABI::Windows::UI::Notifications::IToastNotificationManagerForUser2

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotificationManagerForUser3;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3 ABI::Windows::UI::Notifications::IToastNotificationManagerForUser3

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotificationManagerStatics;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics ABI::Windows::UI::Notifications::IToastNotificationManagerStatics

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotificationManagerStatics2;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2 ABI::Windows::UI::Notifications::IToastNotificationManagerStatics2

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotificationManagerStatics4;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4 ABI::Windows::UI::Notifications::IToastNotificationManagerStatics4

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotificationManagerStatics5;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5 ABI::Windows::UI::Notifications::IToastNotificationManagerStatics5

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotifier;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier ABI::Windows::UI::Notifications::IToastNotifier

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotifier2;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2 ABI::Windows::UI::Notifications::IToastNotifier2

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotifier3;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3 ABI::Windows::UI::Notifications::IToastNotifier3

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IUserNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotification ABI::Windows::UI::Notifications::IUserNotification

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IUserNotificationChangedEventArgs;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs ABI::Windows::UI::Notifications::IUserNotificationChangedEventArgs

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class ToastCollection;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CUI__CNotifications__CToastCollection_USE
#define DEF___FIIterator_1_Windows__CUI__CNotifications__CToastCollection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1512ed75-8c74-5520-ac88-134a1403f7ad"))
IIterator<ABI::Windows::UI::Notifications::ToastCollection*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastCollection*, ABI::Windows::UI::Notifications::IToastCollection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Notifications.ToastCollection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Notifications::ToastCollection*> __FIIterator_1_Windows__CUI__CNotifications__CToastCollection_t;
#define __FIIterator_1_Windows__CUI__CNotifications__CToastCollection ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CNotifications__CToastCollection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CNotifications__CToastCollection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CUI__CNotifications__CToastCollection_USE
#define DEF___FIIterable_1_Windows__CUI__CNotifications__CToastCollection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8928d527-db5d-5a10-ae9b-430fa0906e74"))
IIterable<ABI::Windows::UI::Notifications::ToastCollection*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastCollection*, ABI::Windows::UI::Notifications::IToastCollection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Notifications.ToastCollection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Notifications::ToastCollection*> __FIIterable_1_Windows__CUI__CNotifications__CToastCollection_t;
#define __FIIterable_1_Windows__CUI__CNotifications__CToastCollection ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CNotifications__CToastCollection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CNotifications__CToastCollection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_USE
#define DEF___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e09d090a-12e2-569b-ad29-fb0dfb98a1da"))
IVectorView<ABI::Windows::UI::Notifications::ToastCollection*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastCollection*, ABI::Windows::UI::Notifications::IToastCollection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Notifications.ToastCollection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Notifications::ToastCollection*> __FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_t;
#define __FIVectorView_1_Windows__CUI__CNotifications__CToastCollection ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("34d4fa14-252b-5cb4-a7da-971ee5daec7c"))
IAsyncOperation<__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.UI.Notifications.ToastCollection>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection*> __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4650e069-3052-530e-bc38-93c411773b77"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.UI.Notifications.ToastCollection>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9310ec47-9f0a-5999-80c2-4b31e9f77e8e"))
IAsyncOperation<ABI::Windows::UI::Notifications::ToastCollection*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastCollection*, ABI::Windows::UI::Notifications::IToastCollection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.Notifications.ToastCollection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::UI::Notifications::ToastCollection*> __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_t;
#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8d44ca1e-15d7-5bcb-b002-384c87171c74"))
IAsyncOperationCompletedHandler<ABI::Windows::UI::Notifications::ToastCollection*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastCollection*, ABI::Windows::UI::Notifications::IToastCollection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.Notifications.ToastCollection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::UI::Notifications::ToastCollection*> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class ToastNotificationHistory;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2a22fecb-4b96-551f-b7b9-a7b5beecad05"))
IAsyncOperation<ABI::Windows::UI::Notifications::ToastNotificationHistory*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastNotificationHistory*, ABI::Windows::UI::Notifications::IToastNotificationHistory*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.Notifications.ToastNotificationHistory>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::UI::Notifications::ToastNotificationHistory*> __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_t;
#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c661d5da-6762-5d93-9138-e7dacd571056"))
IAsyncOperationCompletedHandler<ABI::Windows::UI::Notifications::ToastNotificationHistory*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastNotificationHistory*, ABI::Windows::UI::Notifications::IToastNotificationHistory*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.Notifications.ToastNotificationHistory>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::UI::Notifications::ToastNotificationHistory*> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class ToastNotifier;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_USE
#define DEF___FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2dddc10e-38e6-5655-adf3-820e8fb14dcc"))
IAsyncOperation<ABI::Windows::UI::Notifications::ToastNotifier*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastNotifier*, ABI::Windows::UI::Notifications::IToastNotifier*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.UI.Notifications.ToastNotifier>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::UI::Notifications::ToastNotifier*> __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_t;
#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fde26ed7-bc37-5a7c-b3da-3e41ac97bba4"))
IAsyncOperationCompletedHandler<ABI::Windows::UI::Notifications::ToastNotifier*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastNotifier*, ABI::Windows::UI::Notifications::IToastNotifier*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.UI.Notifications.ToastNotifier>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::UI::Notifications::ToastNotifier*> __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("60310303-49c5-52e6-abc6-a9b36eccc716"))
IKeyValuePair<HSTRING, HSTRING> : IKeyValuePair_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, HSTRING> __FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("05eb86f1-7140-5517-b88d-cbaebe57e6b1"))
IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e9bdaaf0-cbf6-5c72-be90-29cbf3a1319b"))
IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */


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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CFoundation__CUri_USE
#define DEF___FIIterator_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1c157d0f-5efe-5cec-bbd6-0c6ce9af07a5"))
IIterator<ABI::Windows::Foundation::Uri*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Foundation::Uri*> __FIIterator_1_Windows__CFoundation__CUri_t;
#define __FIIterator_1_Windows__CFoundation__CUri ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CFoundation__CUri_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CFoundation__CUri_USE
#define DEF___FIIterable_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b0d63b78-78ad-5e31-b6d8-e32a0e16c447"))
IIterable<ABI::Windows::Foundation::Uri*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Foundation::Uri*> __FIIterable_1_Windows__CFoundation__CUri_t;
#define __FIIterable_1_Windows__CFoundation__CUri ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CFoundation__CUri_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class AdaptiveNotificationText;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_USE
#define DEF___FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("18015daa-cbc3-5a51-9f4b-3c069135b0e0"))
IIterator<ABI::Windows::UI::Notifications::AdaptiveNotificationText*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::AdaptiveNotificationText*, ABI::Windows::UI::Notifications::IAdaptiveNotificationText*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Notifications.AdaptiveNotificationText>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Notifications::AdaptiveNotificationText*> __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_t;
#define __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_USE
#define DEF___FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0343e8f2-ca4c-5f40-b8d1-3ff47047ce43"))
IIterable<ABI::Windows::UI::Notifications::AdaptiveNotificationText*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::AdaptiveNotificationText*, ABI::Windows::UI::Notifications::IAdaptiveNotificationText*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Notifications.AdaptiveNotificationText>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Notifications::AdaptiveNotificationText*> __FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_t;
#define __FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class NotificationBinding;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_USE
#define DEF___FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("66187f56-9ee1-5c48-82da-6cb9ddf879e7"))
IIterator<ABI::Windows::UI::Notifications::NotificationBinding*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::NotificationBinding*, ABI::Windows::UI::Notifications::INotificationBinding*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Notifications.NotificationBinding>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Notifications::NotificationBinding*> __FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_t;
#define __FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding_USE
#define DEF___FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e8d9489c-635e-5153-8ab7-389f2ee9faca"))
IIterable<ABI::Windows::UI::Notifications::NotificationBinding*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::NotificationBinding*, ABI::Windows::UI::Notifications::INotificationBinding*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Notifications.NotificationBinding>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Notifications::NotificationBinding*> __FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding_t;
#define __FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class ScheduledTileNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_USE
#define DEF___FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5665e491-6543-5036-935f-b5157950b359"))
IIterator<ABI::Windows::UI::Notifications::ScheduledTileNotification*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ScheduledTileNotification*, ABI::Windows::UI::Notifications::IScheduledTileNotification*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Notifications.ScheduledTileNotification>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Notifications::ScheduledTileNotification*> __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_t;
#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification_USE
#define DEF___FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4b60d1e5-52ae-5766-9720-be4ad086f952"))
IIterable<ABI::Windows::UI::Notifications::ScheduledTileNotification*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ScheduledTileNotification*, ABI::Windows::UI::Notifications::IScheduledTileNotification*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Notifications.ScheduledTileNotification>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Notifications::ScheduledTileNotification*> __FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification_t;
#define __FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class ScheduledToastNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_USE
#define DEF___FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("304238b6-888a-5dd2-96cd-bfca8927483b"))
IIterator<ABI::Windows::UI::Notifications::ScheduledToastNotification*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ScheduledToastNotification*, ABI::Windows::UI::Notifications::IScheduledToastNotification*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Notifications.ScheduledToastNotification>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Notifications::ScheduledToastNotification*> __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_t;
#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification_USE
#define DEF___FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7a7b2a51-c182-5846-a861-4f9c036f24ad"))
IIterable<ABI::Windows::UI::Notifications::ScheduledToastNotification*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ScheduledToastNotification*, ABI::Windows::UI::Notifications::IScheduledToastNotification*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Notifications.ScheduledToastNotification>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Notifications::ScheduledToastNotification*> __FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification_t;
#define __FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class ToastNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CUI__CNotifications__CToastNotification_USE
#define DEF___FIIterator_1_Windows__CUI__CNotifications__CToastNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fe1e726a-3aa9-5d98-b19b-97e3e17eec7b"))
IIterator<ABI::Windows::UI::Notifications::ToastNotification*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastNotification*, ABI::Windows::UI::Notifications::IToastNotification*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.UI.Notifications.ToastNotification>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::UI::Notifications::ToastNotification*> __FIIterator_1_Windows__CUI__CNotifications__CToastNotification_t;
#define __FIIterator_1_Windows__CUI__CNotifications__CToastNotification ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CUI__CNotifications__CToastNotification_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CUI__CNotifications__CToastNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CUI__CNotifications__CToastNotification_USE
#define DEF___FIIterable_1_Windows__CUI__CNotifications__CToastNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("52c9428b-d37a-554d-bf55-b8685d5f552d"))
IIterable<ABI::Windows::UI::Notifications::ToastNotification*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastNotification*, ABI::Windows::UI::Notifications::IToastNotification*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.UI.Notifications.ToastNotification>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::UI::Notifications::ToastNotification*> __FIIterable_1_Windows__CUI__CNotifications__CToastNotification_t;
#define __FIIterable_1_Windows__CUI__CNotifications__CToastNotification ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CUI__CNotifications__CToastNotification_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CUI__CNotifications__CToastNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIMapView_2_HSTRING_HSTRING_USE
#define DEF___FIMapView_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ac7f26f2-feb7-5b2a-8ac4-345bc62caede"))
IMapView<HSTRING, HSTRING> : IMapView_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, HSTRING> __FIMapView_2_HSTRING_HSTRING_t;
#define __FIMapView_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIMap_2_HSTRING_HSTRING_USE
#define DEF___FIMap_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f6d1f700-49c2-52ae-8154-826f9908773c"))
IMap<HSTRING, HSTRING> : IMap_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMap`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMap<HSTRING, HSTRING> __FIMap_2_HSTRING_HSTRING_t;
#define __FIMap_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIMap_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMap_2_HSTRING_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_USE
#define DEF___FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b60de92b-4e12-55af-b42f-afe2d70ba278"))
IVectorView<ABI::Windows::UI::Notifications::AdaptiveNotificationText*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::AdaptiveNotificationText*, ABI::Windows::UI::Notifications::IAdaptiveNotificationText*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Notifications.AdaptiveNotificationText>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Notifications::AdaptiveNotificationText*> __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_t;
#define __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_USE
#define DEF___FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9e4d0240-6d5c-582a-a29d-734e84750738"))
IVectorView<ABI::Windows::UI::Notifications::NotificationBinding*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::NotificationBinding*, ABI::Windows::UI::Notifications::INotificationBinding*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Notifications.NotificationBinding>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Notifications::NotificationBinding*> __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_t;
#define __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_USE
#define DEF___FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4f729c64-a213-52e9-af8b-58adca3e597f"))
IVectorView<ABI::Windows::UI::Notifications::ScheduledTileNotification*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ScheduledTileNotification*, ABI::Windows::UI::Notifications::IScheduledTileNotification*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Notifications.ScheduledTileNotification>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Notifications::ScheduledTileNotification*> __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_t;
#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_USE
#define DEF___FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ba0aff1f-6a8a-5a7e-a9f7-505b6266a436"))
IVectorView<ABI::Windows::UI::Notifications::ScheduledToastNotification*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ScheduledToastNotification*, ABI::Windows::UI::Notifications::IScheduledToastNotification*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Notifications.ScheduledToastNotification>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Notifications::ScheduledToastNotification*> __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_t;
#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_USE
#define DEF___FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a819f3de-60aa-5159-8407-f0a7fb1f6832"))
IVectorView<ABI::Windows::UI::Notifications::ToastNotification*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastNotification*, ABI::Windows::UI::Notifications::IToastNotification*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.UI.Notifications.ToastNotification>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::UI::Notifications::ToastNotification*> __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_t;
#define __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_USE
#define DEF___FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7f7d8ad7-b4d2-5a03-be6f-2b89875fb32d"))
IVector<ABI::Windows::UI::Notifications::NotificationBinding*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::NotificationBinding*, ABI::Windows::UI::Notifications::INotificationBinding*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.UI.Notifications.NotificationBinding>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::UI::Notifications::NotificationBinding*> __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_t;
#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ab54de2d-97d9-5528-b6ad-105afe156530"))
ITypedEventHandler<ABI::Windows::UI::Notifications::ToastNotification*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastNotification*, ABI::Windows::UI::Notifications::IToastNotification*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Notifications.ToastNotification, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Notifications::ToastNotification*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class ToastDismissedEventArgs;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("61c2402f-0ed0-5a18-ab69-59f4aa99a368"))
ITypedEventHandler<ABI::Windows::UI::Notifications::ToastNotification*, ABI::Windows::UI::Notifications::ToastDismissedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastNotification*, ABI::Windows::UI::Notifications::IToastNotification*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastDismissedEventArgs*, ABI::Windows::UI::Notifications::IToastDismissedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Notifications.ToastNotification, Windows.UI.Notifications.ToastDismissedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Notifications::ToastNotification*, ABI::Windows::UI::Notifications::ToastDismissedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class ToastFailedEventArgs;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("95e3e803-c969-5e3a-9753-ea2ad22a9a33"))
ITypedEventHandler<ABI::Windows::UI::Notifications::ToastNotification*, ABI::Windows::UI::Notifications::ToastFailedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastNotification*, ABI::Windows::UI::Notifications::IToastNotification*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastFailedEventArgs*, ABI::Windows::UI::Notifications::IToastFailedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Notifications.ToastNotification, Windows.UI.Notifications.ToastFailedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Notifications::ToastNotification*, ABI::Windows::UI::Notifications::ToastFailedEventArgs*> __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class ToastNotificationManagerForUser;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("821bdf6b-029a-5299-93f3-3077b2ee5e33"))
ITypedEventHandler<ABI::Windows::UI::Notifications::ToastNotificationManagerForUser*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastNotificationManagerForUser*, ABI::Windows::UI::Notifications::IToastNotificationManagerForUser*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Notifications.ToastNotificationManagerForUser, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Notifications::ToastNotificationManagerForUser*, IInspectable*> __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class ScheduledToastNotificationShowingEventArgs;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b1bb0cab-f8b9-5909-a872-ef29e05a8c7a"))
ITypedEventHandler<ABI::Windows::UI::Notifications::ToastNotifier*, ABI::Windows::UI::Notifications::ScheduledToastNotificationShowingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ToastNotifier*, ABI::Windows::UI::Notifications::IToastNotifier*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::UI::Notifications::ScheduledToastNotificationShowingEventArgs*, ABI::Windows::UI::Notifications::IScheduledToastNotificationShowingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.UI.Notifications.ToastNotifier, Windows.UI.Notifications.ScheduledToastNotificationShowingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::UI::Notifications::ToastNotifier*, ABI::Windows::UI::Notifications::ScheduledToastNotificationShowingEventArgs*> __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class AppInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IAppInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIAppInfo ABI::Windows::ApplicationModel::IAppInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    class XmlDocument;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Data {
            namespace Xml {
                namespace Dom {
                    interface IXmlDocument;
                } /* Dom */
            } /* Xml */
        } /* Data */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument ABI::Windows::Data::Xml::Dom::IXmlDocument

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                class ValueSet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                interface IPropertySet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet ABI::Windows::Foundation::Collections::IPropertySet

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IAsyncAction;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIAsyncAction ABI::Windows::Foundation::IAsyncAction

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

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
        namespace System {
            class User;
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            interface IUser;
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CIUser ABI::Windows::System::IUser

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                typedef enum AdaptiveNotificationContentKind : int AdaptiveNotificationContentKind;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                typedef enum BadgeTemplateType : int BadgeTemplateType;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                typedef enum NotificationMirroring : int NotificationMirroring;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                typedef enum NotificationSetting : int NotificationSetting;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                typedef enum NotificationUpdateResult : int NotificationUpdateResult;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                typedef enum PeriodicUpdateRecurrence : int PeriodicUpdateRecurrence;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                typedef enum TileFlyoutTemplateType : int TileFlyoutTemplateType;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                typedef enum TileTemplateType : int TileTemplateType;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                typedef enum ToastDismissalReason : int ToastDismissalReason;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                typedef enum ToastHistoryChangedType : int ToastHistoryChangedType;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                typedef enum ToastNotificationMode : int ToastNotificationMode;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                typedef enum ToastNotificationPriority : int ToastNotificationPriority;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                typedef enum ToastTemplateType : int ToastTemplateType;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                typedef enum UserNotificationChangedKind : int UserNotificationChangedKind;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class BadgeNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class BadgeUpdateManagerForUser;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class BadgeUpdater;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class Notification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class NotificationData;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class NotificationVisual;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class TileFlyoutNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class TileFlyoutUpdater;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class TileNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class TileUpdateManagerForUser;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class TileUpdater;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class ToastCollectionManager;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.UI.Notifications.AdaptiveNotificationContentKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                enum AdaptiveNotificationContentKind : int
                {
                    AdaptiveNotificationContentKind_Text = 0,
                };
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Notifications.BadgeTemplateType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                enum BadgeTemplateType : int
                {
                    BadgeTemplateType_BadgeGlyph = 0,
                    BadgeTemplateType_BadgeNumber = 1,
                };
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Notifications.NotificationKinds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                enum NotificationKinds : unsigned int
                {
                    NotificationKinds_Unknown = 0,
                    NotificationKinds_Toast = 0x1,
                };

                DEFINE_ENUM_FLAG_OPERATORS(NotificationKinds)
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Notifications.NotificationMirroring
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                enum NotificationMirroring : int
                {
                    NotificationMirroring_Allowed = 0,
                    NotificationMirroring_Disabled = 1,
                };
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Notifications.NotificationSetting
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                enum NotificationSetting : int
                {
                    NotificationSetting_Enabled = 0,
                    NotificationSetting_DisabledForApplication = 1,
                    NotificationSetting_DisabledForUser = 2,
                    NotificationSetting_DisabledByGroupPolicy = 3,
                    NotificationSetting_DisabledByManifest = 4,
                };
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Notifications.NotificationUpdateResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                enum NotificationUpdateResult : int
                {
                    NotificationUpdateResult_Succeeded = 0,
                    NotificationUpdateResult_Failed = 1,
                    NotificationUpdateResult_NotificationNotFound = 2,
                };
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.Notifications.PeriodicUpdateRecurrence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                enum PeriodicUpdateRecurrence : int
                {
                    PeriodicUpdateRecurrence_HalfHour = 0,
                    PeriodicUpdateRecurrence_Hour = 1,
                    PeriodicUpdateRecurrence_SixHours = 2,
                    PeriodicUpdateRecurrence_TwelveHours = 3,
                    PeriodicUpdateRecurrence_Daily = 4,
                };
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Notifications.TileFlyoutTemplateType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                enum TileFlyoutTemplateType : int
                {
                    TileFlyoutTemplateType_TileFlyoutTemplate01 = 0,
                };
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Notifications.TileTemplateType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                enum TileTemplateType : int
                {
                    TileTemplateType_TileSquareImage
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileSquareImage may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150Image.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 0,
                    TileTemplateType_TileSquareBlock
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileSquareBlock may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150Block.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 1,
                    TileTemplateType_TileSquareText01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileSquareText01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150Text01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 2,
                    TileTemplateType_TileSquareText02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileSquareText02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150Text02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 3,
                    TileTemplateType_TileSquareText03
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileSquareText03 may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150Text03.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 4,
                    TileTemplateType_TileSquareText04
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileSquareText04 may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150Text04.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 5,
                    TileTemplateType_TileSquarePeekImageAndText01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileSquarePeekImageAndText01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150PeekImageAndText01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 6,
                    TileTemplateType_TileSquarePeekImageAndText02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileSquarePeekImageAndText02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150PeekImageAndText02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 7,
                    TileTemplateType_TileSquarePeekImageAndText03
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileSquarePeekImageAndText03 may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150PeekImageAndText03.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 8,
                    TileTemplateType_TileSquarePeekImageAndText04
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileSquarePeekImageAndText04 may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150PeekImageAndText04.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 9,
                    TileTemplateType_TileWideImage
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideImage may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Image.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 10,
                    TileTemplateType_TileWideImageCollection
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideImageCollection may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150ImageCollection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 11,
                    TileTemplateType_TileWideImageAndText01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideImageAndText01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150ImageAndText01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 12,
                    TileTemplateType_TileWideImageAndText02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideImageAndText02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150ImageAndText02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 13,
                    TileTemplateType_TileWideBlockAndText01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideBlockAndText01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150BlockAndText01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 14,
                    TileTemplateType_TileWideBlockAndText02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideBlockAndText02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150BlockAndText02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 15,
                    TileTemplateType_TileWidePeekImageCollection01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWidePeekImageCollection01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImageCollection01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 16,
                    TileTemplateType_TileWidePeekImageCollection02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWidePeekImageCollection02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImageCollection02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 17,
                    TileTemplateType_TileWidePeekImageCollection03
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWidePeekImageCollection03 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImageCollection03.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 18,
                    TileTemplateType_TileWidePeekImageCollection04
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWidePeekImageCollection04 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImageCollection04.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 19,
                    TileTemplateType_TileWidePeekImageCollection05
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWidePeekImageCollection05 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImageCollection05.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 20,
                    TileTemplateType_TileWidePeekImageCollection06
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWidePeekImageCollection06 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImageCollection06.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 21,
                    TileTemplateType_TileWidePeekImageAndText01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWidePeekImageAndText01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImageAndText01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 22,
                    TileTemplateType_TileWidePeekImageAndText02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWidePeekImageAndText02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImageAndText02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 23,
                    TileTemplateType_TileWidePeekImage01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWidePeekImage01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImage01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 24,
                    TileTemplateType_TileWidePeekImage02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWidePeekImage02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImage02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 25,
                    TileTemplateType_TileWidePeekImage03
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWidePeekImage03 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImage03.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 26,
                    TileTemplateType_TileWidePeekImage04
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWidePeekImage04 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImage04.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 27,
                    TileTemplateType_TileWidePeekImage05
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWidePeekImage05 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImage05.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 28,
                    TileTemplateType_TileWidePeekImage06
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWidePeekImage06 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImage06.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 29,
                    TileTemplateType_TileWideSmallImageAndText01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideSmallImageAndText01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150SmallImageAndText01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 30,
                    TileTemplateType_TileWideSmallImageAndText02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideSmallImageAndText02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150SmallImageAndText02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 31,
                    TileTemplateType_TileWideSmallImageAndText03
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideSmallImageAndText03 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150SmallImageAndText03.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 32,
                    TileTemplateType_TileWideSmallImageAndText04
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideSmallImageAndText04 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150SmallImageAndText04.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 33,
                    TileTemplateType_TileWideSmallImageAndText05
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideSmallImageAndText05 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150SmallImageAndText05.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 34,
                    TileTemplateType_TileWideText01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideText01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 35,
                    TileTemplateType_TileWideText02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideText02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 36,
                    TileTemplateType_TileWideText03
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideText03 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text03.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 37,
                    TileTemplateType_TileWideText04
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideText04 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text04.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 38,
                    TileTemplateType_TileWideText05
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideText05 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text05.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 39,
                    TileTemplateType_TileWideText06
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideText06 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text06.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 40,
                    TileTemplateType_TileWideText07
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideText07 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text07.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 41,
                    TileTemplateType_TileWideText08
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideText08 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text08.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 42,
                    TileTemplateType_TileWideText09
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideText09 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text09.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 43,
                    TileTemplateType_TileWideText10
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideText10 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 44,
                    TileTemplateType_TileWideText11
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATEDENUMERATOR("TileWideText11 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text11.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    = 45,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare150x150Image = 0,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare150x150Block = 1,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare150x150Text01 = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare150x150Text02 = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare150x150Text03 = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare150x150Text04 = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare150x150PeekImageAndText01 = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare150x150PeekImageAndText02 = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare150x150PeekImageAndText03 = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare150x150PeekImageAndText04 = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150Image = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150ImageCollection = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150ImageAndText01 = 12,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150ImageAndText02 = 13,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150BlockAndText01 = 14,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150BlockAndText02 = 15,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150PeekImageCollection01 = 16,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150PeekImageCollection02 = 17,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150PeekImageCollection03 = 18,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150PeekImageCollection04 = 19,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150PeekImageCollection05 = 20,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150PeekImageCollection06 = 21,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150PeekImageAndText01 = 22,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150PeekImageAndText02 = 23,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150PeekImage01 = 24,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150PeekImage02 = 25,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150PeekImage03 = 26,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150PeekImage04 = 27,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150PeekImage05 = 28,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150PeekImage06 = 29,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150SmallImageAndText01 = 30,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150SmallImageAndText02 = 31,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150SmallImageAndText03 = 32,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150SmallImageAndText04 = 33,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150SmallImageAndText05 = 34,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150Text01 = 35,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150Text02 = 36,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150Text03 = 37,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150Text04 = 38,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150Text05 = 39,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150Text06 = 40,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150Text07 = 41,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150Text08 = 42,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150Text09 = 43,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150Text10 = 44,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150Text11 = 45,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310BlockAndText01 = 46,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310BlockAndText02 = 47,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310Image = 48,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310ImageAndText01 = 49,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310ImageAndText02 = 50,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310ImageAndTextOverlay01 = 51,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310ImageAndTextOverlay02 = 52,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310ImageAndTextOverlay03 = 53,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310ImageCollectionAndText01 = 54,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310ImageCollectionAndText02 = 55,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310ImageCollection = 56,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310SmallImagesAndTextList01 = 57,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310SmallImagesAndTextList02 = 58,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310SmallImagesAndTextList03 = 59,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310SmallImagesAndTextList04 = 60,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310Text01 = 61,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310Text02 = 62,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310Text03 = 63,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310Text04 = 64,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310Text05 = 65,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310Text06 = 66,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310Text07 = 67,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310Text08 = 68,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310TextList01 = 69,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310TextList02 = 70,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310TextList03 = 71,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310SmallImageAndText01 = 72,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310SmallImagesAndTextList05 = 73,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare310x310Text09 = 74,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare71x71IconWithBadge = 75,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare150x150IconWithBadge = 76,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileWide310x150IconWithBadgeAndText = 77,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileSquare71x71Image = 78,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    TileTemplateType_TileTall150x310Image = 79,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Notifications.ToastDismissalReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                enum ToastDismissalReason : int
                {
                    ToastDismissalReason_UserCanceled = 0,
                    ToastDismissalReason_ApplicationHidden = 1,
                    ToastDismissalReason_TimedOut = 2,
                };
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Notifications.ToastHistoryChangedType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                enum ToastHistoryChangedType : int
                {
                    ToastHistoryChangedType_Cleared = 0,
                    ToastHistoryChangedType_Removed = 1,
                    ToastHistoryChangedType_Expired = 2,
                    ToastHistoryChangedType_Added = 3,
                };
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Notifications.ToastNotificationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                enum ToastNotificationMode : int
                {
                    ToastNotificationMode_Unrestricted = 0,
                    ToastNotificationMode_PriorityOnly = 1,
                    ToastNotificationMode_AlarmsOnly = 2,
                };
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Struct Windows.UI.Notifications.ToastNotificationPriority
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                enum ToastNotificationPriority : int
                {
                    ToastNotificationPriority_Default = 0,
                    ToastNotificationPriority_High = 1,
                };
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.Notifications.ToastTemplateType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                enum ToastTemplateType : int
                {
                    ToastTemplateType_ToastImageAndText01 = 0,
                    ToastTemplateType_ToastImageAndText02 = 1,
                    ToastTemplateType_ToastImageAndText03 = 2,
                    ToastTemplateType_ToastImageAndText04 = 3,
                    ToastTemplateType_ToastText01 = 4,
                    ToastTemplateType_ToastText02 = 5,
                    ToastTemplateType_ToastText03 = 6,
                    ToastTemplateType_ToastText04 = 7,
                };
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Notifications.UserNotificationChangedKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                enum UserNotificationChangedKind : int
                {
                    UserNotificationChangedKind_Added = 0,
                    UserNotificationChangedKind_Removed = 1,
                };
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IAdaptiveNotificationContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IAdaptiveNotificationContent[] = L"Windows.UI.Notifications.IAdaptiveNotificationContent";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("eb0dbe66-7448-448d-9db8-d78acd2abba9")
                IAdaptiveNotificationContent : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::UI::Notifications::AdaptiveNotificationContentKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Hints(
                        __FIMap_2_HSTRING_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdaptiveNotificationContent = __uuidof(IAdaptiveNotificationContent);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IAdaptiveNotificationText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.AdaptiveNotificationText
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IAdaptiveNotificationText[] = L"Windows.UI.Notifications.IAdaptiveNotificationText";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("46d4a3be-609a-4326-a40b-bfde872034a3")
                IAdaptiveNotificationText : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Text(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Language(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Language(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAdaptiveNotificationText = __uuidof(IAdaptiveNotificationText);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IBadgeNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.BadgeNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IBadgeNotification[] = L"Windows.UI.Notifications.IBadgeNotification";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("075cb4ca-d08a-4e2f-9233-7e289c1f7722")
                IBadgeNotification : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Content(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ExpirationTime(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExpirationTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBadgeNotification = __uuidof(IBadgeNotification);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IBadgeNotificationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.BadgeNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IBadgeNotificationFactory[] = L"Windows.UI.Notifications.IBadgeNotificationFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("edf255ce-0618-4d59-948a-5a61040c52f9")
                IBadgeNotificationFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateBadgeNotification(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument* content,
                        ABI::Windows::UI::Notifications::IBadgeNotification** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBadgeNotificationFactory = __uuidof(IBadgeNotificationFactory);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IBadgeUpdateManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.BadgeUpdateManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IBadgeUpdateManagerForUser[] = L"Windows.UI.Notifications.IBadgeUpdateManagerForUser";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("996b21bc-0386-44e5-ba8d-0c1077a62e92")
                IBadgeUpdateManagerForUser : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateBadgeUpdaterForApplication(
                        ABI::Windows::UI::Notifications::IBadgeUpdater** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateBadgeUpdaterForApplicationWithId(
                        HSTRING applicationId,
                        ABI::Windows::UI::Notifications::IBadgeUpdater** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateBadgeUpdaterForSecondaryTile(
                        HSTRING tileId,
                        ABI::Windows::UI::Notifications::IBadgeUpdater** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBadgeUpdateManagerForUser = __uuidof(IBadgeUpdateManagerForUser);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IBadgeUpdateManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.BadgeUpdateManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IBadgeUpdateManagerStatics[] = L"Windows.UI.Notifications.IBadgeUpdateManagerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("33400faa-6dd5-4105-aebc-9b50fca492da")
                IBadgeUpdateManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateBadgeUpdaterForApplication(
                        ABI::Windows::UI::Notifications::IBadgeUpdater** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateBadgeUpdaterForApplicationWithId(
                        HSTRING applicationId,
                        ABI::Windows::UI::Notifications::IBadgeUpdater** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateBadgeUpdaterForSecondaryTile(
                        HSTRING tileId,
                        ABI::Windows::UI::Notifications::IBadgeUpdater** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTemplateContent(
                        ABI::Windows::UI::Notifications::BadgeTemplateType type,
                        ABI::Windows::Data::Xml::Dom::IXmlDocument** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBadgeUpdateManagerStatics = __uuidof(IBadgeUpdateManagerStatics);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IBadgeUpdateManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.BadgeUpdateManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IBadgeUpdateManagerStatics2[] = L"Windows.UI.Notifications.IBadgeUpdateManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("979a35ce-f940-48bf-94e8-ca244d400b41")
                IBadgeUpdateManagerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::UI::Notifications::IBadgeUpdateManagerForUser** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBadgeUpdateManagerStatics2 = __uuidof(IBadgeUpdateManagerStatics2);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IBadgeUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.BadgeUpdater
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IBadgeUpdater[] = L"Windows.UI.Notifications.IBadgeUpdater";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("b5fa1fd4-7562-4f6c-bfa3-1b6ed2e57f2f")
                IBadgeUpdater : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Update(
                        ABI::Windows::UI::Notifications::IBadgeNotification* notification
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Clear(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartPeriodicUpdate(
                        ABI::Windows::Foundation::IUriRuntimeClass* badgeContent,
                        ABI::Windows::UI::Notifications::PeriodicUpdateRecurrence requestedInterval
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartPeriodicUpdateAtTime(
                        ABI::Windows::Foundation::IUriRuntimeClass* badgeContent,
                        ABI::Windows::Foundation::DateTime startTime,
                        ABI::Windows::UI::Notifications::PeriodicUpdateRecurrence requestedInterval
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopPeriodicUpdate(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IBadgeUpdater = __uuidof(IBadgeUpdater);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IKnownAdaptiveNotificationHintsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.KnownAdaptiveNotificationHints
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IKnownAdaptiveNotificationHintsStatics[] = L"Windows.UI.Notifications.IKnownAdaptiveNotificationHintsStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("06206598-d496-497d-8692-4f7d7c2770df")
                IKnownAdaptiveNotificationHintsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Style(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Wrap(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxLines(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinLines(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TextStacking(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Align(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKnownAdaptiveNotificationHintsStatics = __uuidof(IKnownAdaptiveNotificationHintsStatics);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IKnownAdaptiveNotificationTextStylesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.KnownAdaptiveNotificationTextStyles
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IKnownAdaptiveNotificationTextStylesStatics[] = L"Windows.UI.Notifications.IKnownAdaptiveNotificationTextStylesStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("202192d7-8996-45aa-8ba1-d461d72c2a1b")
                IKnownAdaptiveNotificationTextStylesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Caption(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Body(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Base(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Subtitle(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Subheader(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Header(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TitleNumeral(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SubheaderNumeral(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HeaderNumeral(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CaptionSubtle(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BodySubtle(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BaseSubtle(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SubtitleSubtle(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TitleSubtle(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SubheaderSubtle(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SubheaderNumeralSubtle(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HeaderSubtle(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HeaderNumeralSubtle(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKnownAdaptiveNotificationTextStylesStatics = __uuidof(IKnownAdaptiveNotificationTextStylesStatics);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IKnownNotificationBindingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.KnownNotificationBindings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IKnownNotificationBindingsStatics[] = L"Windows.UI.Notifications.IKnownNotificationBindingsStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("79427bae-a8b7-4d58-89ea-76a7b7bccded")
                IKnownNotificationBindingsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ToastGeneric(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IKnownNotificationBindingsStatics = __uuidof(IKnownNotificationBindingsStatics);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.INotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.Notification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CINotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CINotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_INotification[] = L"Windows.UI.Notifications.INotification";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("108037fe-eb76-4f82-97bc-da07530a2e20")
                INotification : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExpirationTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ExpirationTime(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Visual(
                        ABI::Windows::UI::Notifications::INotificationVisual** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Visual(
                        ABI::Windows::UI::Notifications::INotificationVisual* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INotification = __uuidof(INotification);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CINotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CINotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.INotificationBinding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.NotificationBinding
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_INotificationBinding[] = L"Windows.UI.Notifications.INotificationBinding";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("f29e4b85-0370-4ad3-b4ea-da9e35e7eabf")
                INotificationBinding : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Template(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Template(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Language(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Language(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Hints(
                        __FIMap_2_HSTRING_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTextElements(
                        __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INotificationBinding = __uuidof(INotificationBinding);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CINotificationBinding;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.INotificationData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.NotificationData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CINotificationData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CINotificationData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_INotificationData[] = L"Windows.UI.Notifications.INotificationData";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("9ffd2312-9d6a-4aaf-b6ac-ff17f0c1f280")
                INotificationData : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Values(
                        __FIMap_2_HSTRING_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SequenceNumber(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SequenceNumber(
                        UINT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INotificationData = __uuidof(INotificationData);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CINotificationData;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CINotificationData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.INotificationDataFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.NotificationData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_INotificationDataFactory[] = L"Windows.UI.Notifications.INotificationDataFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("23c1e33a-1c10-46fb-8040-dec384621cf8")
                INotificationDataFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateNotificationDataWithValuesAndSequenceNumber(
                        __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* initialValues,
                        UINT32 sequenceNumber,
                        ABI::Windows::UI::Notifications::INotificationData** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateNotificationDataWithValues(
                        __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* initialValues,
                        ABI::Windows::UI::Notifications::INotificationData** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INotificationDataFactory = __uuidof(INotificationDataFactory);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.INotificationVisual
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.NotificationVisual
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_INotificationVisual[] = L"Windows.UI.Notifications.INotificationVisual";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("68835b8e-aa56-4e11-86d3-5f9a6957bc5b")
                INotificationVisual : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Language(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Language(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Bindings(
                        __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetBinding(
                        HSTRING templateName,
                        ABI::Windows::UI::Notifications::INotificationBinding** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_INotificationVisual = __uuidof(INotificationVisual);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CINotificationVisual;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IScheduledTileNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ScheduledTileNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IScheduledTileNotification[] = L"Windows.UI.Notifications.IScheduledTileNotification";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("0abca6d5-99dc-4c78-a11c-c9e7f86d7ef7")
                IScheduledTileNotification : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Content(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeliveryTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ExpirationTime(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExpirationTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Tag(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Tag(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Id(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IScheduledTileNotification = __uuidof(IScheduledTileNotification);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IScheduledTileNotificationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ScheduledTileNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IScheduledTileNotificationFactory[] = L"Windows.UI.Notifications.IScheduledTileNotificationFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("3383138a-98c0-4c3b-bbd6-4a633c7cfc29")
                IScheduledTileNotificationFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateScheduledTileNotification(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument* content,
                        ABI::Windows::Foundation::DateTime deliveryTime,
                        ABI::Windows::UI::Notifications::IScheduledTileNotification** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IScheduledTileNotificationFactory = __uuidof(IScheduledTileNotificationFactory);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IScheduledToastNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ScheduledToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IScheduledToastNotification[] = L"Windows.UI.Notifications.IScheduledToastNotification";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("79f577f8-0de7-48cd-9740-9b370490c838")
                IScheduledToastNotification : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Content(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeliveryTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SnoozeInterval(
                        __FIReference_1_Windows__CFoundation__CTimeSpan** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaximumSnoozeCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Id(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IScheduledToastNotification = __uuidof(IScheduledToastNotification);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IScheduledToastNotification2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ScheduledToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IScheduledToastNotification2[] = L"Windows.UI.Notifications.IScheduledToastNotification2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("a66ea09c-31b4-43b0-b5dd-7a40e85363b1")
                IScheduledToastNotification2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Tag(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Tag(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Group(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Group(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SuppressPopup(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SuppressPopup(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IScheduledToastNotification2 = __uuidof(IScheduledToastNotification2);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IScheduledToastNotification3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ScheduledToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IScheduledToastNotification3[] = L"Windows.UI.Notifications.IScheduledToastNotification3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("98429e8b-bd32-4a3b-9d15-22aea49462a1")
                IScheduledToastNotification3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NotificationMirroring(
                        ABI::Windows::UI::Notifications::NotificationMirroring* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NotificationMirroring(
                        ABI::Windows::UI::Notifications::NotificationMirroring value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RemoteId(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IScheduledToastNotification3 = __uuidof(IScheduledToastNotification3);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IScheduledToastNotification4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ScheduledToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IScheduledToastNotification4[] = L"Windows.UI.Notifications.IScheduledToastNotification4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("1d4761fd-bdef-4e4a-96be-0101369b58d2")
                IScheduledToastNotification4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExpirationTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ExpirationTime(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IScheduledToastNotification4 = __uuidof(IScheduledToastNotification4);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Notifications.IScheduledToastNotificationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ScheduledToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IScheduledToastNotificationFactory[] = L"Windows.UI.Notifications.IScheduledToastNotificationFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("e7bed191-0bb9-4189-8394-31761b476fd7")
                IScheduledToastNotificationFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateScheduledToastNotification(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument* content,
                        ABI::Windows::Foundation::DateTime deliveryTime,
                        ABI::Windows::UI::Notifications::IScheduledToastNotification** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateScheduledToastNotificationRecurring(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument* content,
                        ABI::Windows::Foundation::DateTime deliveryTime,
                        ABI::Windows::Foundation::TimeSpan snoozeInterval,
                        UINT32 maximumSnoozeCount,
                        ABI::Windows::UI::Notifications::IScheduledToastNotification** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IScheduledToastNotificationFactory = __uuidof(IScheduledToastNotificationFactory);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IScheduledToastNotificationShowingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ScheduledToastNotificationShowingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IScheduledToastNotificationShowingEventArgs[] = L"Windows.UI.Notifications.IScheduledToastNotificationShowingEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("6173f6b4-412a-5e2c-a6ed-a0209aef9a09")
                IScheduledToastNotificationShowingEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Cancel(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Cancel(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ScheduledToastNotification(
                        ABI::Windows::UI::Notifications::IScheduledToastNotification** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IScheduledToastNotificationShowingEventArgs = __uuidof(IScheduledToastNotificationShowingEventArgs);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Notifications.IShownTileNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ShownTileNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IShownTileNotification[] = L"Windows.UI.Notifications.IShownTileNotification";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("342d8988-5af2-481a-a6a3-f2fdc78de88e")
                IShownTileNotification : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Arguments(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IShownTileNotification = __uuidof(IShownTileNotification);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.ITileFlyoutNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileFlyoutNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileFlyoutNotification[] = L"Windows.UI.Notifications.ITileFlyoutNotification";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("9a53b261-c70c-42be-b2f3-f42aa97d34e5")
                ITileFlyoutNotification : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Content(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ExpirationTime(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExpirationTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITileFlyoutNotification = __uuidof(ITileFlyoutNotification);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.ITileFlyoutNotificationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileFlyoutNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileFlyoutNotificationFactory[] = L"Windows.UI.Notifications.ITileFlyoutNotificationFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("ef556ff5-5226-4f2b-b278-88a35dfe569f")
                ITileFlyoutNotificationFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateTileFlyoutNotification(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument* content,
                        ABI::Windows::UI::Notifications::ITileFlyoutNotification** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITileFlyoutNotificationFactory = __uuidof(ITileFlyoutNotificationFactory);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.ITileFlyoutUpdateManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileFlyoutUpdateManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileFlyoutUpdateManagerStatics[] = L"Windows.UI.Notifications.ITileFlyoutUpdateManagerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("04363b0b-1ac0-4b99-88e7-ada83e953d48")
                ITileFlyoutUpdateManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateTileFlyoutUpdaterForApplication(
                        ABI::Windows::UI::Notifications::ITileFlyoutUpdater** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateTileFlyoutUpdaterForApplicationWithId(
                        HSTRING applicationId,
                        ABI::Windows::UI::Notifications::ITileFlyoutUpdater** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateTileFlyoutUpdaterForSecondaryTile(
                        HSTRING tileId,
                        ABI::Windows::UI::Notifications::ITileFlyoutUpdater** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTemplateContent(
                        ABI::Windows::UI::Notifications::TileFlyoutTemplateType type,
                        ABI::Windows::Data::Xml::Dom::IXmlDocument** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITileFlyoutUpdateManagerStatics = __uuidof(ITileFlyoutUpdateManagerStatics);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.ITileFlyoutUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileFlyoutUpdater
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileFlyoutUpdater[] = L"Windows.UI.Notifications.ITileFlyoutUpdater";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("8d40c76a-c465-4052-a740-5c2654c1a089")
                ITileFlyoutUpdater : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Update(
                        ABI::Windows::UI::Notifications::ITileFlyoutNotification* notification
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Clear(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartPeriodicUpdate(
                        ABI::Windows::Foundation::IUriRuntimeClass* tileFlyoutContent,
                        ABI::Windows::UI::Notifications::PeriodicUpdateRecurrence requestedInterval
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartPeriodicUpdateAtTime(
                        ABI::Windows::Foundation::IUriRuntimeClass* tileFlyoutContent,
                        ABI::Windows::Foundation::DateTime startTime,
                        ABI::Windows::UI::Notifications::PeriodicUpdateRecurrence requestedInterval
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopPeriodicUpdate(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Setting(
                        ABI::Windows::UI::Notifications::NotificationSetting* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITileFlyoutUpdater = __uuidof(ITileFlyoutUpdater);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.ITileNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileNotification[] = L"Windows.UI.Notifications.ITileNotification";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("ebaec8fa-50ec-4c18-b4d0-3af02e5540ab")
                ITileNotification : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Content(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ExpirationTime(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExpirationTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Tag(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Tag(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITileNotification = __uuidof(ITileNotification);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileNotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.ITileNotificationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileNotificationFactory[] = L"Windows.UI.Notifications.ITileNotificationFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("c6abdd6e-4928-46c8-bdbf-81a047dea0d4")
                ITileNotificationFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateTileNotification(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument* content,
                        ABI::Windows::UI::Notifications::ITileNotification** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITileNotificationFactory = __uuidof(ITileNotificationFactory);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.ITileUpdateManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileUpdateManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileUpdateManagerForUser[] = L"Windows.UI.Notifications.ITileUpdateManagerForUser";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("55141348-2ee2-4e2d-9cc1-216a20decc9f")
                ITileUpdateManagerForUser : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateTileUpdaterForApplication(
                        ABI::Windows::UI::Notifications::ITileUpdater** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateTileUpdaterForApplicationWithId(
                        HSTRING applicationId,
                        ABI::Windows::UI::Notifications::ITileUpdater** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateTileUpdaterForSecondaryTile(
                        HSTRING tileId,
                        ABI::Windows::UI::Notifications::ITileUpdater** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITileUpdateManagerForUser = __uuidof(ITileUpdateManagerForUser);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.ITileUpdateManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileUpdateManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileUpdateManagerStatics[] = L"Windows.UI.Notifications.ITileUpdateManagerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("da159e5d-3ea9-4986-8d84-b09d5e12276d")
                ITileUpdateManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateTileUpdaterForApplication(
                        ABI::Windows::UI::Notifications::ITileUpdater** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateTileUpdaterForApplicationWithId(
                        HSTRING applicationId,
                        ABI::Windows::UI::Notifications::ITileUpdater** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateTileUpdaterForSecondaryTile(
                        HSTRING tileId,
                        ABI::Windows::UI::Notifications::ITileUpdater** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTemplateContent(
                        ABI::Windows::UI::Notifications::TileTemplateType type,
                        ABI::Windows::Data::Xml::Dom::IXmlDocument** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITileUpdateManagerStatics = __uuidof(ITileUpdateManagerStatics);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.ITileUpdateManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileUpdateManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileUpdateManagerStatics2[] = L"Windows.UI.Notifications.ITileUpdateManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("731c1ddc-8e14-4b7c-a34b-9d22de76c84d")
                ITileUpdateManagerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::UI::Notifications::ITileUpdateManagerForUser** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITileUpdateManagerStatics2 = __uuidof(ITileUpdateManagerStatics2);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.ITileUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileUpdater
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileUpdater[] = L"Windows.UI.Notifications.ITileUpdater";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("0942a48b-1d91-44ec-9243-c1e821c29a20")
                ITileUpdater : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Update(
                        ABI::Windows::UI::Notifications::ITileNotification* notification
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Clear(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EnableNotificationQueue(
                        boolean enable
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Setting(
                        ABI::Windows::UI::Notifications::NotificationSetting* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddToSchedule(
                        ABI::Windows::UI::Notifications::IScheduledTileNotification* scheduledTile
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveFromSchedule(
                        ABI::Windows::UI::Notifications::IScheduledTileNotification* scheduledTile
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetScheduledTileNotifications(
                        __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartPeriodicUpdate(
                        ABI::Windows::Foundation::IUriRuntimeClass* tileContent,
                        ABI::Windows::UI::Notifications::PeriodicUpdateRecurrence requestedInterval
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartPeriodicUpdateAtTime(
                        ABI::Windows::Foundation::IUriRuntimeClass* tileContent,
                        ABI::Windows::Foundation::DateTime startTime,
                        ABI::Windows::UI::Notifications::PeriodicUpdateRecurrence requestedInterval
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StopPeriodicUpdate(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartPeriodicUpdateBatch(
                        __FIIterable_1_Windows__CFoundation__CUri* tileContents,
                        ABI::Windows::UI::Notifications::PeriodicUpdateRecurrence requestedInterval
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartPeriodicUpdateBatchAtTime(
                        __FIIterable_1_Windows__CFoundation__CUri* tileContents,
                        ABI::Windows::Foundation::DateTime startTime,
                        ABI::Windows::UI::Notifications::PeriodicUpdateRecurrence requestedInterval
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITileUpdater = __uuidof(ITileUpdater);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileUpdater;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.ITileUpdater2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileUpdater
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileUpdater2[] = L"Windows.UI.Notifications.ITileUpdater2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("a2266e12-15ee-43ed-83f5-65b352bb1a84")
                ITileUpdater2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE EnableNotificationQueueForSquare150x150(
                        boolean enable
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EnableNotificationQueueForWide310x150(
                        boolean enable
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EnableNotificationQueueForSquare310x310(
                        boolean enable
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITileUpdater2 = __uuidof(ITileUpdater2);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileUpdater2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastActivatedEventArgs[] = L"Windows.UI.Notifications.IToastActivatedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("e3bf92f3-c197-436f-8265-0625824f8dac")
                IToastActivatedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Arguments(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastActivatedEventArgs = __uuidof(IToastActivatedEventArgs);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastActivatedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastActivatedEventArgs2[] = L"Windows.UI.Notifications.IToastActivatedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("ab7da512-cc61-568e-81be-304ac31038fa")
                IToastActivatedEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UserInput(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastActivatedEventArgs2 = __uuidof(IToastActivatedEventArgs2);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Notifications.IToastCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastCollection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastCollection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastCollection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastCollection[] = L"Windows.UI.Notifications.IToastCollection";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("0a8bc3b0-e0be-4858-bc2a-89dfe0b32863")
                IToastCollection : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LaunchArgs(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LaunchArgs(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Icon(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Icon(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastCollection = __uuidof(IToastCollection);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastCollection;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastCollection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.IToastCollectionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastCollection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastCollectionFactory[] = L"Windows.UI.Notifications.IToastCollectionFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("164dd3d7-73c4-44f7-b4ff-fb6d4bf1f4c6")
                IToastCollectionFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        HSTRING collectionId,
                        HSTRING displayName,
                        HSTRING launchArgs,
                        ABI::Windows::Foundation::IUriRuntimeClass* iconUri,
                        ABI::Windows::UI::Notifications::IToastCollection** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastCollectionFactory = __uuidof(IToastCollectionFactory);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.IToastCollectionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastCollectionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastCollectionManager[] = L"Windows.UI.Notifications.IToastCollectionManager";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("2a1821fe-179d-49bc-b79d-a527920d3665")
                IToastCollectionManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SaveToastCollectionAsync(
                        ABI::Windows::UI::Notifications::IToastCollection* collection,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllToastCollectionsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetToastCollectionAsync(
                        HSTRING collectionId,
                        __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveToastCollectionAsync(
                        HSTRING collectionId,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveAllToastCollectionsAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastCollectionManager = __uuidof(IToastCollectionManager);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.IToastDismissedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastDismissedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastDismissedEventArgs[] = L"Windows.UI.Notifications.IToastDismissedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("3f89d935-d9cb-4538-a0f0-ffe7659938f8")
                IToastDismissedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Reason(
                        ABI::Windows::UI::Notifications::ToastDismissalReason* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastDismissedEventArgs = __uuidof(IToastDismissedEventArgs);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastFailedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastFailedEventArgs[] = L"Windows.UI.Notifications.IToastFailedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("35176862-cfd4-44f8-ad64-f500fd896c3b")
                IToastFailedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ErrorCode(
                        HRESULT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastFailedEventArgs = __uuidof(IToastFailedEventArgs);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotification[] = L"Windows.UI.Notifications.IToastNotification";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("997e2675-059e-4e60-8b06-1760917c8b80")
                IToastNotification : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Content(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ExpirationTime(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExpirationTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Dismissed(
                        __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Dismissed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Activated(
                        __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Activated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Failed(
                        __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Failed(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotification = __uuidof(IToastNotification);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotification2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotification2[] = L"Windows.UI.Notifications.IToastNotification2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("9dfb9fd1-143a-490e-90bf-b9fba7132de7")
                IToastNotification2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Tag(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Tag(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Group(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Group(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SuppressPopup(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SuppressPopup(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotification2 = __uuidof(IToastNotification2);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotification2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotification3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotification3[] = L"Windows.UI.Notifications.IToastNotification3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("31e8aed8-8141-4f99-bc0a-c4ed21297d77")
                IToastNotification3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NotificationMirroring(
                        ABI::Windows::UI::Notifications::NotificationMirroring* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NotificationMirroring(
                        ABI::Windows::UI::Notifications::NotificationMirroring value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RemoteId(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotification3 = __uuidof(IToastNotification3);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotification3;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotification4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotification4[] = L"Windows.UI.Notifications.IToastNotification4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("15154935-28ea-4727-88e9-c58680e2d118")
                IToastNotification4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Data(
                        ABI::Windows::UI::Notifications::INotificationData** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Data(
                        ABI::Windows::UI::Notifications::INotificationData* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Priority(
                        ABI::Windows::UI::Notifications::ToastNotificationPriority* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Priority(
                        ABI::Windows::UI::Notifications::ToastNotificationPriority value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotification4 = __uuidof(IToastNotification4);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotification4;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotification6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotification6[] = L"Windows.UI.Notifications.IToastNotification6";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("43ebfe53-89ae-5c1e-a279-3aecfe9b6f54")
                IToastNotification6 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExpiresOnReboot(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ExpiresOnReboot(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotification6 = __uuidof(IToastNotification6);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotification6;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationActionTriggerDetail
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationActionTriggerDetail
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationActionTriggerDetail[] = L"Windows.UI.Notifications.IToastNotificationActionTriggerDetail";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("9445135a-38f3-42f6-96aa-7955b0f03da2")
                IToastNotificationActionTriggerDetail : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Argument(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserInput(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotificationActionTriggerDetail = __uuidof(IToastNotificationActionTriggerDetail);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationFactory[] = L"Windows.UI.Notifications.IToastNotificationFactory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("04124b20-82c6-4229-b109-fd9ed4662b53")
                IToastNotificationFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateToastNotification(
                        ABI::Windows::Data::Xml::Dom::IXmlDocument* content,
                        ABI::Windows::UI::Notifications::IToastNotification** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotificationFactory = __uuidof(IToastNotificationFactory);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationHistory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationHistory
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationHistory[] = L"Windows.UI.Notifications.IToastNotificationHistory";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("5caddc63-01d3-4c97-986f-0533483fee14")
                IToastNotificationHistory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RemoveGroup(
                        HSTRING group
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveGroupWithId(
                        HSTRING group,
                        HSTRING applicationId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveGroupedTagWithId(
                        HSTRING tag,
                        HSTRING group,
                        HSTRING applicationId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveGroupedTag(
                        HSTRING tag,
                        HSTRING group
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Remove(
                        HSTRING tag
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Clear(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ClearWithId(
                        HSTRING applicationId
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotificationHistory = __uuidof(IToastNotificationHistory);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationHistory2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationHistory
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationHistory2[] = L"Windows.UI.Notifications.IToastNotificationHistory2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("3bc3d253-2f31-4092-9129-8ad5abf067da")
                IToastNotificationHistory2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetHistory(
                        __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetHistoryWithId(
                        HSTRING applicationId,
                        __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotificationHistory2 = __uuidof(IToastNotificationHistory2);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationHistoryChangedTriggerDetail
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationHistoryChangedTriggerDetail[] = L"Windows.UI.Notifications.IToastNotificationHistoryChangedTriggerDetail";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("db037ffa-0068-412c-9c83-267c37f65670")
                IToastNotificationHistoryChangedTriggerDetail : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ChangeType(
                        ABI::Windows::UI::Notifications::ToastHistoryChangedType* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotificationHistoryChangedTriggerDetail = __uuidof(IToastNotificationHistoryChangedTriggerDetail);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationHistoryChangedTriggerDetail2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationHistoryChangedTriggerDetail2[] = L"Windows.UI.Notifications.IToastNotificationHistoryChangedTriggerDetail2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("0b36e982-c871-49fb-babb-25bdbc4cc45b")
                IToastNotificationHistoryChangedTriggerDetail2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CollectionId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotificationHistoryChangedTriggerDetail2 = __uuidof(IToastNotificationHistoryChangedTriggerDetail2);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationManagerForUser[] = L"Windows.UI.Notifications.IToastNotificationManagerForUser";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("79ab57f6-43fe-487b-8a7f-99567200ae94")
                IToastNotificationManagerForUser : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateToastNotifier(
                        ABI::Windows::UI::Notifications::IToastNotifier** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateToastNotifierWithId(
                        HSTRING applicationId,
                        ABI::Windows::UI::Notifications::IToastNotifier** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_History(
                        ABI::Windows::UI::Notifications::IToastNotificationHistory** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotificationManagerForUser = __uuidof(IToastNotificationManagerForUser);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationManagerForUser2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationManagerForUser2[] = L"Windows.UI.Notifications.IToastNotificationManagerForUser2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("679c64b7-81ab-42c2-8819-c958767753f4")
                IToastNotificationManagerForUser2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetToastNotifierForToastCollectionIdAsync(
                        HSTRING collectionId,
                        __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetHistoryForToastCollectionIdAsync(
                        HSTRING collectionId,
                        __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetToastCollectionManager(
                        ABI::Windows::UI::Notifications::IToastCollectionManager** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetToastCollectionManagerWithAppId(
                        HSTRING appId,
                        ABI::Windows::UI::Notifications::IToastCollectionManager** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotificationManagerForUser2 = __uuidof(IToastNotificationManagerForUser2);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationManagerForUser3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationManagerForUser3[] = L"Windows.UI.Notifications.IToastNotificationManagerForUser3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("3efcb176-6cc1-56dc-973b-251f7aacb1c5")
                IToastNotificationManagerForUser3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_NotificationMode(
                        ABI::Windows::UI::Notifications::ToastNotificationMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_NotificationModeChanged(
                        __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_NotificationModeChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotificationManagerForUser3 = __uuidof(IToastNotificationManagerForUser3);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationManagerStatics[] = L"Windows.UI.Notifications.IToastNotificationManagerStatics";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("50ac103f-d235-4598-bbef-98fe4d1a3ad4")
                IToastNotificationManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateToastNotifier(
                        ABI::Windows::UI::Notifications::IToastNotifier** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateToastNotifierWithId(
                        HSTRING applicationId,
                        ABI::Windows::UI::Notifications::IToastNotifier** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTemplateContent(
                        ABI::Windows::UI::Notifications::ToastTemplateType type,
                        ABI::Windows::Data::Xml::Dom::IXmlDocument** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotificationManagerStatics = __uuidof(IToastNotificationManagerStatics);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationManagerStatics2[] = L"Windows.UI.Notifications.IToastNotificationManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("7ab93c52-0e48-4750-ba9d-1a4113981847")
                IToastNotificationManagerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_History(
                        ABI::Windows::UI::Notifications::IToastNotificationHistory** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotificationManagerStatics2 = __uuidof(IToastNotificationManagerStatics2);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationManagerStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationManagerStatics4[] = L"Windows.UI.Notifications.IToastNotificationManagerStatics4";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("8f993fd3-e516-45fb-8130-398e93fa52c3")
                IToastNotificationManagerStatics4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::UI::Notifications::IToastNotificationManagerForUser** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConfigureNotificationMirroring(
                        ABI::Windows::UI::Notifications::NotificationMirroring value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotificationManagerStatics4 = __uuidof(IToastNotificationManagerStatics4);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationManagerStatics5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationManagerStatics5[] = L"Windows.UI.Notifications.IToastNotificationManagerStatics5";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("d6f5f569-d40d-407c-8989-88cab42cfd14")
                IToastNotificationManagerStatics5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::UI::Notifications::IToastNotificationManagerForUser** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotificationManagerStatics5 = __uuidof(IToastNotificationManagerStatics5);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotifier[] = L"Windows.UI.Notifications.IToastNotifier";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("75927b93-03f3-41ec-91d3-6e5bac1b38e7")
                IToastNotifier : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Show(
                        ABI::Windows::UI::Notifications::IToastNotification* notification
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Hide(
                        ABI::Windows::UI::Notifications::IToastNotification* notification
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Setting(
                        ABI::Windows::UI::Notifications::NotificationSetting* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddToSchedule(
                        ABI::Windows::UI::Notifications::IScheduledToastNotification* scheduledToast
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveFromSchedule(
                        ABI::Windows::UI::Notifications::IScheduledToastNotification* scheduledToast
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetScheduledToastNotifications(
                        __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotifier = __uuidof(IToastNotifier);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotifier;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotifier2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotifier2[] = L"Windows.UI.Notifications.IToastNotifier2";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("354389c6-7c01-4bd5-9c20-604340cd2b74")
                IToastNotifier2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE UpdateWithTagAndGroup(
                        ABI::Windows::UI::Notifications::INotificationData* data,
                        HSTRING tag,
                        HSTRING group,
                        ABI::Windows::UI::Notifications::NotificationUpdateResult* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UpdateWithTag(
                        ABI::Windows::UI::Notifications::INotificationData* data,
                        HSTRING tag,
                        ABI::Windows::UI::Notifications::NotificationUpdateResult* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotifier2 = __uuidof(IToastNotifier2);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotifier3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotifier3[] = L"Windows.UI.Notifications.IToastNotifier3";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("ae75a04a-3b0c-51ad-b7e8-b08ab6052549")
                IToastNotifier3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_ScheduledToastNotificationShowing(
                        __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ScheduledToastNotificationShowing(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IToastNotifier3 = __uuidof(IToastNotifier3);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Notifications.IUserNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.UserNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IUserNotification[] = L"Windows.UI.Notifications.IUserNotification";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("adf7e52f-4e53-42d5-9c33-eb5ea515b23e")
                IUserNotification : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Notification(
                        ABI::Windows::UI::Notifications::INotification** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppInfo(
                        ABI::Windows::ApplicationModel::IAppInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CreationTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserNotification = __uuidof(IUserNotification);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIUserNotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IUserNotificationChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.UserNotificationChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IUserNotificationChangedEventArgs[] = L"Windows.UI.Notifications.IUserNotificationChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                MIDL_INTERFACE("b6bd6839-79cf-4b25-82c0-0ce1eef81f8c")
                IUserNotificationChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ChangeKind(
                        ABI::Windows::UI::Notifications::UserNotificationChangedKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserNotificationId(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUserNotificationChangedEventArgs = __uuidof(IUserNotificationChangedEventArgs);
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.AdaptiveNotificationText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IAdaptiveNotificationText ** Default Interface **
 *    Windows.UI.Notifications.IAdaptiveNotificationContent
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_AdaptiveNotificationText_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_AdaptiveNotificationText_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_AdaptiveNotificationText[] = L"Windows.UI.Notifications.AdaptiveNotificationText";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.BadgeNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Notifications.IBadgeNotificationFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IBadgeNotification ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_BadgeNotification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_BadgeNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_BadgeNotification[] = L"Windows.UI.Notifications.BadgeNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.BadgeUpdateManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.IBadgeUpdateManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Notifications.IBadgeUpdateManagerStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_BadgeUpdateManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_BadgeUpdateManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_BadgeUpdateManager[] = L"Windows.UI.Notifications.BadgeUpdateManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.BadgeUpdateManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IBadgeUpdateManagerForUser ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_BadgeUpdateManagerForUser_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_BadgeUpdateManagerForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_BadgeUpdateManagerForUser[] = L"Windows.UI.Notifications.BadgeUpdateManagerForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.BadgeUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IBadgeUpdater ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_BadgeUpdater_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_BadgeUpdater_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_BadgeUpdater[] = L"Windows.UI.Notifications.BadgeUpdater";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.KnownAdaptiveNotificationHints
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.IKnownAdaptiveNotificationHintsStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_KnownAdaptiveNotificationHints_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_KnownAdaptiveNotificationHints_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_KnownAdaptiveNotificationHints[] = L"Windows.UI.Notifications.KnownAdaptiveNotificationHints";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.KnownAdaptiveNotificationTextStyles
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.IKnownAdaptiveNotificationTextStylesStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_KnownAdaptiveNotificationTextStyles_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_KnownAdaptiveNotificationTextStyles_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_KnownAdaptiveNotificationTextStyles[] = L"Windows.UI.Notifications.KnownAdaptiveNotificationTextStyles";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.KnownNotificationBindings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.IKnownNotificationBindingsStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_KnownNotificationBindings_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_KnownNotificationBindings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_KnownNotificationBindings[] = L"Windows.UI.Notifications.KnownNotificationBindings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.Notification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.INotification ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_Notification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_Notification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_Notification[] = L"Windows.UI.Notifications.Notification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.NotificationBinding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.INotificationBinding ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_NotificationBinding_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_NotificationBinding_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_NotificationBinding[] = L"Windows.UI.Notifications.NotificationBinding";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.NotificationData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Notifications.INotificationDataFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.INotificationData ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_NotificationData_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_NotificationData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_NotificationData[] = L"Windows.UI.Notifications.NotificationData";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Notifications.NotificationVisual
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.INotificationVisual ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_NotificationVisual_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_NotificationVisual_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_NotificationVisual[] = L"Windows.UI.Notifications.NotificationVisual";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.ScheduledTileNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Notifications.IScheduledTileNotificationFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IScheduledTileNotification ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ScheduledTileNotification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ScheduledTileNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ScheduledTileNotification[] = L"Windows.UI.Notifications.ScheduledTileNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ScheduledToastNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Notifications.IScheduledToastNotificationFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IScheduledToastNotification ** Default Interface **
 *    Windows.UI.Notifications.IScheduledToastNotification2
 *    Windows.UI.Notifications.IScheduledToastNotification3
 *    Windows.UI.Notifications.IScheduledToastNotification4
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ScheduledToastNotification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ScheduledToastNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ScheduledToastNotification[] = L"Windows.UI.Notifications.ScheduledToastNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ScheduledToastNotificationShowingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IScheduledToastNotificationShowingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ScheduledToastNotificationShowingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ScheduledToastNotificationShowingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ScheduledToastNotificationShowingEventArgs[] = L"Windows.UI.Notifications.ScheduledToastNotificationShowingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.Notifications.ShownTileNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IShownTileNotification ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ShownTileNotification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ShownTileNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ShownTileNotification[] = L"Windows.UI.Notifications.ShownTileNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.TileFlyoutNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Notifications.ITileFlyoutNotificationFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.ITileFlyoutNotification ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_TileFlyoutNotification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_TileFlyoutNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_TileFlyoutNotification[] = L"Windows.UI.Notifications.TileFlyoutNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.TileFlyoutUpdateManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.ITileFlyoutUpdateManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_TileFlyoutUpdateManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_TileFlyoutUpdateManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_TileFlyoutUpdateManager[] = L"Windows.UI.Notifications.TileFlyoutUpdateManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.TileFlyoutUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.ITileFlyoutUpdater ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_TileFlyoutUpdater_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_TileFlyoutUpdater_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_TileFlyoutUpdater[] = L"Windows.UI.Notifications.TileFlyoutUpdater";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.TileNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Notifications.ITileNotificationFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.ITileNotification ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_TileNotification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_TileNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_TileNotification[] = L"Windows.UI.Notifications.TileNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.TileUpdateManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.ITileUpdateManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Notifications.ITileUpdateManagerStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_TileUpdateManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_TileUpdateManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_TileUpdateManager[] = L"Windows.UI.Notifications.TileUpdateManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.TileUpdateManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.ITileUpdateManagerForUser ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_TileUpdateManagerForUser_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_TileUpdateManagerForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_TileUpdateManagerForUser[] = L"Windows.UI.Notifications.TileUpdateManagerForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.TileUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.ITileUpdater ** Default Interface **
 *    Windows.UI.Notifications.ITileUpdater2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_TileUpdater_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_TileUpdater_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_TileUpdater[] = L"Windows.UI.Notifications.TileUpdater";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastActivatedEventArgs ** Default Interface **
 *    Windows.UI.Notifications.IToastActivatedEventArgs2
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastActivatedEventArgs[] = L"Windows.UI.Notifications.ToastActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Notifications.IToastCollectionFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastCollection ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastCollection_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastCollection[] = L"Windows.UI.Notifications.ToastCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Notifications.ToastCollectionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastCollectionManager ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastCollectionManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastCollectionManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastCollectionManager[] = L"Windows.UI.Notifications.ToastCollectionManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Notifications.ToastDismissedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastDismissedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastDismissedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastDismissedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastDismissedEventArgs[] = L"Windows.UI.Notifications.ToastDismissedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastFailedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastFailedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastFailedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastFailedEventArgs[] = L"Windows.UI.Notifications.ToastFailedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Notifications.IToastNotificationFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastNotification ** Default Interface **
 *    Windows.UI.Notifications.IToastNotification2
 *    Windows.UI.Notifications.IToastNotification3
 *    Windows.UI.Notifications.IToastNotification4
 *    Windows.UI.Notifications.IToastNotification6
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastNotification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastNotification[] = L"Windows.UI.Notifications.ToastNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastNotificationActionTriggerDetail
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastNotificationActionTriggerDetail ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationActionTriggerDetail_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationActionTriggerDetail_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastNotificationActionTriggerDetail[] = L"Windows.UI.Notifications.ToastNotificationActionTriggerDetail";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastNotificationHistory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastNotificationHistory2
 *    Windows.UI.Notifications.IToastNotificationHistory ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationHistory_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationHistory_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastNotificationHistory[] = L"Windows.UI.Notifications.ToastNotificationHistory";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastNotificationHistoryChangedTriggerDetail ** Default Interface **
 *    Windows.UI.Notifications.IToastNotificationHistoryChangedTriggerDetail2
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationHistoryChangedTriggerDetail_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationHistoryChangedTriggerDetail_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastNotificationHistoryChangedTriggerDetail[] = L"Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastNotificationManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.IToastNotificationManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Notifications.IToastNotificationManagerStatics4 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Notifications.IToastNotificationManagerStatics5 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Notifications.IToastNotificationManagerStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastNotificationManager[] = L"Windows.UI.Notifications.ToastNotificationManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastNotificationManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastNotificationManagerForUser ** Default Interface **
 *    Windows.UI.Notifications.IToastNotificationManagerForUser2
 *    Windows.UI.Notifications.IToastNotificationManagerForUser3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationManagerForUser_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationManagerForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastNotificationManagerForUser[] = L"Windows.UI.Notifications.ToastNotificationManagerForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.ToastNotifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastNotifier ** Default Interface **
 *    Windows.UI.Notifications.IToastNotifier2
 *    Windows.UI.Notifications.IToastNotifier3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastNotifier_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastNotifier_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastNotifier[] = L"Windows.UI.Notifications.ToastNotifier";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.UserNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IUserNotification ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_UserNotification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_UserNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_UserNotification[] = L"Windows.UI.Notifications.UserNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.UserNotificationChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IUserNotificationChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_UserNotificationChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_UserNotificationChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_UserNotificationChangedEventArgs[] = L"Windows.UI.Notifications.UserNotificationChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2 __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics __x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CINotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CINotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CINotification __x_ABI_CWindows_CUI_CNotifications_CINotification;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CINotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CINotificationData_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CINotificationData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CINotificationData __x_ABI_CWindows_CUI_CNotifications_CINotificationData;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CINotificationData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory __x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CINotificationVisual __x_ABI_CWindows_CUI_CNotifications_CINotificationVisual;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2 __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3 __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4 __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CITileNotification __x_ABI_CWindows_CUI_CNotifications_CITileNotification;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory __x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2 __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CITileUpdater __x_ABI_CWindows_CUI_CNotifications_CITileUpdater;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CITileUpdater2 __x_ABI_CWindows_CUI_CNotifications_CITileUpdater2;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2 __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastCollection_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastCollection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastCollection __x_ABI_CWindows_CUI_CNotifications_CIToastCollection;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastCollection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs __x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs __x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotification __x_ABI_CWindows_CUI_CNotifications_CIToastNotification;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2 __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3 __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4 __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotification6 __x_ABI_CWindows_CUI_CNotifications_CIToastNotification6;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2 __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2 __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2 __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3 __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2 __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4 __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5 __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2 __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3 __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIUserNotification __x_ABI_CWindows_CUI_CNotifications_CIUserNotification;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CNotifications__CToastCollection __FIIterator_1_Windows__CUI__CNotifications__CToastCollection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CNotifications__CToastCollection;

typedef struct __FIIterator_1_Windows__CUI__CNotifications__CToastCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CNotifications__CToastCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CNotifications__CToastCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CNotifications__CToastCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CNotifications__CToastCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CNotifications__CToastCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CNotifications__CToastCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CNotifications__CToastCollection* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastCollection** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CNotifications__CToastCollection* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CNotifications__CToastCollection* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CNotifications__CToastCollection* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CIToastCollection** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CNotifications__CToastCollectionVtbl;

interface __FIIterator_1_Windows__CUI__CNotifications__CToastCollection
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CNotifications__CToastCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CNotifications__CToastCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastCollection_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastCollection_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastCollection_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastCollection_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CNotifications__CToastCollection __FIIterable_1_Windows__CUI__CNotifications__CToastCollection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CNotifications__CToastCollection;

typedef struct __FIIterable_1_Windows__CUI__CNotifications__CToastCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CNotifications__CToastCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CNotifications__CToastCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CNotifications__CToastCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CNotifications__CToastCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CNotifications__CToastCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CNotifications__CToastCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CNotifications__CToastCollection* This,
        __FIIterator_1_Windows__CUI__CNotifications__CToastCollection** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CNotifications__CToastCollectionVtbl;

interface __FIIterable_1_Windows__CUI__CNotifications__CToastCollection
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CNotifications__CToastCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CNotifications__CToastCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CNotifications__CToastCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CNotifications__CToastCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CNotifications__CToastCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CNotifications__CToastCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CNotifications__CToastCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CNotifications__CToastCollection_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CNotifications__CToastCollection __FIVectorView_1_Windows__CUI__CNotifications__CToastCollection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection;

typedef struct __FIVectorView_1_Windows__CUI__CNotifications__CToastCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CNotifications_CIToastCollection** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastCollection* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CIToastCollection** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CNotifications__CToastCollectionVtbl;

interface __FIVectorView_1_Windows__CUI__CNotifications__CToastCollection
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CNotifications__CToastCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        __FIVectorView_1_Windows__CUI__CNotifications__CToastCollection** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollectionVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollectionVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection;

typedef struct __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastCollection** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollectionVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection* This,
        __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollectionVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastCollection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory;

typedef struct __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistoryVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory* This,
        __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistoryVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotificationHistory_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier;

typedef struct __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifierVtbl;

interface __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier* This,
        __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifierVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CUI__CNotifications__CToastNotifier_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_HSTRING __FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIKeyValuePair_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_HSTRING** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CFoundation__CUri __FIIterator_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CFoundation__CUri;

typedef struct __FIIterator_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CFoundation__CUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CFoundation__CUri* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CFoundation__CUri* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CFoundation__CUri* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CFoundation__CUriVtbl;

interface __FIIterator_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIIterator_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CFoundation__CUri_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CFoundation__CUri_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CFoundation__CUri_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CFoundation__CUri_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CFoundation__CUri __FIIterable_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CFoundation__CUri;

typedef struct __FIIterable_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CFoundation__CUri* This,
        __FIIterator_1_Windows__CFoundation__CUri** result);

    END_INTERFACE
} __FIIterable_1_Windows__CFoundation__CUriVtbl;

interface __FIIterable_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIIterable_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CFoundation__CUri_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText;

typedef struct __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationTextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationTextVtbl;

interface __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationTextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText __FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText;

typedef struct __FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationTextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        __FIIterator_1_Windows__CUI__CNotifications__CAdaptiveNotificationText** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationTextVtbl;

interface __FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationTextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding __FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding;

typedef struct __FIIterator_1_Windows__CUI__CNotifications__CNotificationBindingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CNotifications__CNotificationBindingVtbl;

interface __FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CNotifications__CNotificationBindingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding __FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding;

typedef struct __FIIterable_1_Windows__CUI__CNotifications__CNotificationBindingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        __FIIterator_1_Windows__CUI__CNotifications__CNotificationBinding** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CNotifications__CNotificationBindingVtbl;

interface __FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CNotifications__CNotificationBindingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CNotifications__CNotificationBinding_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification;

typedef struct __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotificationVtbl;

interface __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification __FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification;

typedef struct __FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        __FIIterator_1_Windows__CUI__CNotifications__CScheduledTileNotification** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotificationVtbl;

interface __FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CNotifications__CScheduledTileNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification;

typedef struct __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotificationVtbl;

interface __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification __FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification;

typedef struct __FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        __FIIterator_1_Windows__CUI__CNotifications__CScheduledToastNotification** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotificationVtbl;

interface __FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CNotifications__CScheduledToastNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CUI__CNotifications__CToastNotification_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CUI__CNotifications__CToastNotification_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CUI__CNotifications__CToastNotification __FIIterator_1_Windows__CUI__CNotifications__CToastNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CUI__CNotifications__CToastNotification;

typedef struct __FIIterator_1_Windows__CUI__CNotifications__CToastNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CUI__CNotifications__CToastNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CUI__CNotifications__CToastNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CUI__CNotifications__CToastNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CUI__CNotifications__CToastNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CUI__CNotifications__CToastNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CUI__CNotifications__CToastNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CUI__CNotifications__CToastNotification* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CUI__CNotifications__CToastNotification* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CUI__CNotifications__CToastNotification* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CUI__CNotifications__CToastNotification* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CUI__CNotifications__CToastNotificationVtbl;

interface __FIIterator_1_Windows__CUI__CNotifications__CToastNotification
{
    CONST_VTBL struct __FIIterator_1_Windows__CUI__CNotifications__CToastNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CUI__CNotifications__CToastNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastNotification_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastNotification_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastNotification_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CUI__CNotifications__CToastNotification_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CUI__CNotifications__CToastNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CUI__CNotifications__CToastNotification_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CUI__CNotifications__CToastNotification_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CUI__CNotifications__CToastNotification __FIIterable_1_Windows__CUI__CNotifications__CToastNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CUI__CNotifications__CToastNotification;

typedef struct __FIIterable_1_Windows__CUI__CNotifications__CToastNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CUI__CNotifications__CToastNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CUI__CNotifications__CToastNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CUI__CNotifications__CToastNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CUI__CNotifications__CToastNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CUI__CNotifications__CToastNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CUI__CNotifications__CToastNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CUI__CNotifications__CToastNotification* This,
        __FIIterator_1_Windows__CUI__CNotifications__CToastNotification** result);

    END_INTERFACE
} __FIIterable_1_Windows__CUI__CNotifications__CToastNotificationVtbl;

interface __FIIterable_1_Windows__CUI__CNotifications__CToastNotification
{
    CONST_VTBL struct __FIIterable_1_Windows__CUI__CNotifications__CToastNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CUI__CNotifications__CToastNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CUI__CNotifications__CToastNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CUI__CNotifications__CToastNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CUI__CNotifications__CToastNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CUI__CNotifications__CToastNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CUI__CNotifications__CToastNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CUI__CNotifications__CToastNotification_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CUI__CNotifications__CToastNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

#if !defined(____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_HSTRING;

typedef struct __FIMapView_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** first,
        __FIMapView_2_HSTRING_HSTRING** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_HSTRINGVtbl;

interface __FIMapView_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIMapView_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMap_2_HSTRING_HSTRING __FIMap_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMap_2_HSTRING_HSTRING;

typedef struct __FIMap_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMap_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMap_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMap_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMap_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMap_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMap_2_HSTRING_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIMap_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* Insert)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING value,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Remove)(__FIMap_2_HSTRING_HSTRING* This,
        HSTRING key);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIMap_2_HSTRING_HSTRING* This);

    END_INTERFACE
} __FIMap_2_HSTRING_HSTRINGVtbl;

interface __FIMap_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIMap_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMap_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMap_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMap_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMap_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMap_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMap_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMap_2_HSTRING_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMap_2_HSTRING_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMap_2_HSTRING_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMap_2_HSTRING_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIMap_2_HSTRING_HSTRING_Insert(This, key, value, result) \
    ((This)->lpVtbl->Insert(This, key, value, result))

#define __FIMap_2_HSTRING_HSTRING_Remove(This, key) \
    ((This)->lpVtbl->Remove(This, key))

#define __FIMap_2_HSTRING_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#endif /* COBJMACROS */

#endif // ____FIMap_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText;

typedef struct __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationTextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationTextVtbl;

interface __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationTextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding;

typedef struct __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBindingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBindingVtbl;

interface __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBindingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification;

typedef struct __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotificationVtbl;

interface __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification;

typedef struct __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotificationVtbl;

interface __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CUI__CNotifications__CToastNotification;

typedef struct __FIVectorView_1_Windows__CUI__CNotifications__CToastNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CUI__CNotifications__CToastNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CUI__CNotifications__CToastNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CUI__CNotifications__CToastNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CUI__CNotifications__CToastNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CUI__CNotifications__CToastNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CUI__CNotifications__CToastNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CUI__CNotifications__CToastNotification* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CUI__CNotifications__CToastNotification* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CUI__CNotifications__CToastNotification* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CUI__CNotifications__CToastNotification* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CUI__CNotifications__CToastNotificationVtbl;

interface __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification
{
    CONST_VTBL struct __FIVectorView_1_Windows__CUI__CNotifications__CToastNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CUI__CNotifications__CToastNotification_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CUI__CNotifications__CNotificationBinding;

typedef struct __FIVector_1_Windows__CUI__CNotifications__CNotificationBindingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        __FIVectorView_1_Windows__CUI__CNotifications__CNotificationBinding** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        UINT32 index,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CUI__CNotifications__CNotificationBinding* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding** items);

    END_INTERFACE
} __FIVector_1_Windows__CUI__CNotifications__CNotificationBindingVtbl;

interface __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding
{
    CONST_VTBL struct __FIVector_1_Windows__CUI__CNotifications__CNotificationBindingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CUI__CNotifications__CNotificationBinding_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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
#if !defined(____FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification* sender,
        __x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification* sender,
        __x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier* sender,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppInfo __x_ABI_CWindows_CApplicationModel_CIAppInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__
#define ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument;

#endif // ____x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CUI_CNotifications_CAdaptiveNotificationContentKind __x_ABI_CWindows_CUI_CNotifications_CAdaptiveNotificationContentKind;

typedef enum __x_ABI_CWindows_CUI_CNotifications_CBadgeTemplateType __x_ABI_CWindows_CUI_CNotifications_CBadgeTemplateType;

typedef enum __x_ABI_CWindows_CUI_CNotifications_CNotificationMirroring __x_ABI_CWindows_CUI_CNotifications_CNotificationMirroring;

typedef enum __x_ABI_CWindows_CUI_CNotifications_CNotificationSetting __x_ABI_CWindows_CUI_CNotifications_CNotificationSetting;

typedef enum __x_ABI_CWindows_CUI_CNotifications_CNotificationUpdateResult __x_ABI_CWindows_CUI_CNotifications_CNotificationUpdateResult;

typedef enum __x_ABI_CWindows_CUI_CNotifications_CPeriodicUpdateRecurrence __x_ABI_CWindows_CUI_CNotifications_CPeriodicUpdateRecurrence;

typedef enum __x_ABI_CWindows_CUI_CNotifications_CTileFlyoutTemplateType __x_ABI_CWindows_CUI_CNotifications_CTileFlyoutTemplateType;

typedef enum __x_ABI_CWindows_CUI_CNotifications_CTileTemplateType __x_ABI_CWindows_CUI_CNotifications_CTileTemplateType;

typedef enum __x_ABI_CWindows_CUI_CNotifications_CToastDismissalReason __x_ABI_CWindows_CUI_CNotifications_CToastDismissalReason;

typedef enum __x_ABI_CWindows_CUI_CNotifications_CToastHistoryChangedType __x_ABI_CWindows_CUI_CNotifications_CToastHistoryChangedType;

typedef enum __x_ABI_CWindows_CUI_CNotifications_CToastNotificationMode __x_ABI_CWindows_CUI_CNotifications_CToastNotificationMode;

typedef enum __x_ABI_CWindows_CUI_CNotifications_CToastNotificationPriority __x_ABI_CWindows_CUI_CNotifications_CToastNotificationPriority;

typedef enum __x_ABI_CWindows_CUI_CNotifications_CToastTemplateType __x_ABI_CWindows_CUI_CNotifications_CToastTemplateType;

typedef enum __x_ABI_CWindows_CUI_CNotifications_CUserNotificationChangedKind __x_ABI_CWindows_CUI_CNotifications_CUserNotificationChangedKind;

/*
 *
 * Struct Windows.UI.Notifications.AdaptiveNotificationContentKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CNotifications_CAdaptiveNotificationContentKind
{
    AdaptiveNotificationContentKind_Text = 0,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Notifications.BadgeTemplateType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CNotifications_CBadgeTemplateType
{
    BadgeTemplateType_BadgeGlyph = 0,
    BadgeTemplateType_BadgeNumber = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Notifications.NotificationKinds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CNotifications_CNotificationKinds
{
    NotificationKinds_Unknown = 0,
    NotificationKinds_Toast = 0x1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Notifications.NotificationMirroring
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CNotifications_CNotificationMirroring
{
    NotificationMirroring_Allowed = 0,
    NotificationMirroring_Disabled = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.UI.Notifications.NotificationSetting
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CNotifications_CNotificationSetting
{
    NotificationSetting_Enabled = 0,
    NotificationSetting_DisabledForApplication = 1,
    NotificationSetting_DisabledForUser = 2,
    NotificationSetting_DisabledByGroupPolicy = 3,
    NotificationSetting_DisabledByManifest = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Notifications.NotificationUpdateResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CUI_CNotifications_CNotificationUpdateResult
{
    NotificationUpdateResult_Succeeded = 0,
    NotificationUpdateResult_Failed = 1,
    NotificationUpdateResult_NotificationNotFound = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.Notifications.PeriodicUpdateRecurrence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CNotifications_CPeriodicUpdateRecurrence
{
    PeriodicUpdateRecurrence_HalfHour = 0,
    PeriodicUpdateRecurrence_Hour = 1,
    PeriodicUpdateRecurrence_SixHours = 2,
    PeriodicUpdateRecurrence_TwelveHours = 3,
    PeriodicUpdateRecurrence_Daily = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Notifications.TileFlyoutTemplateType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CNotifications_CTileFlyoutTemplateType
{
    TileFlyoutTemplateType_TileFlyoutTemplate01 = 0,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Notifications.TileTemplateType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CNotifications_CTileTemplateType
{
    TileTemplateType_TileSquareImage
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileSquareImage may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150Image.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 0,
    TileTemplateType_TileSquareBlock
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileSquareBlock may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150Block.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 1,
    TileTemplateType_TileSquareText01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileSquareText01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150Text01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 2,
    TileTemplateType_TileSquareText02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileSquareText02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150Text02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 3,
    TileTemplateType_TileSquareText03
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileSquareText03 may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150Text03.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 4,
    TileTemplateType_TileSquareText04
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileSquareText04 may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150Text04.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 5,
    TileTemplateType_TileSquarePeekImageAndText01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileSquarePeekImageAndText01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150PeekImageAndText01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 6,
    TileTemplateType_TileSquarePeekImageAndText02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileSquarePeekImageAndText02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150PeekImageAndText02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 7,
    TileTemplateType_TileSquarePeekImageAndText03
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileSquarePeekImageAndText03 may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150PeekImageAndText03.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 8,
    TileTemplateType_TileSquarePeekImageAndText04
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileSquarePeekImageAndText04 may be altered or unavailable for releases after Windows 8.1. Instead, use TileSquare150x150PeekImageAndText04.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 9,
    TileTemplateType_TileWideImage
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideImage may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Image.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 10,
    TileTemplateType_TileWideImageCollection
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideImageCollection may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150ImageCollection.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 11,
    TileTemplateType_TileWideImageAndText01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideImageAndText01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150ImageAndText01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 12,
    TileTemplateType_TileWideImageAndText02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideImageAndText02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150ImageAndText02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 13,
    TileTemplateType_TileWideBlockAndText01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideBlockAndText01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150BlockAndText01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 14,
    TileTemplateType_TileWideBlockAndText02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideBlockAndText02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150BlockAndText02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 15,
    TileTemplateType_TileWidePeekImageCollection01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWidePeekImageCollection01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImageCollection01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 16,
    TileTemplateType_TileWidePeekImageCollection02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWidePeekImageCollection02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImageCollection02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 17,
    TileTemplateType_TileWidePeekImageCollection03
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWidePeekImageCollection03 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImageCollection03.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 18,
    TileTemplateType_TileWidePeekImageCollection04
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWidePeekImageCollection04 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImageCollection04.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 19,
    TileTemplateType_TileWidePeekImageCollection05
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWidePeekImageCollection05 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImageCollection05.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 20,
    TileTemplateType_TileWidePeekImageCollection06
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWidePeekImageCollection06 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImageCollection06.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 21,
    TileTemplateType_TileWidePeekImageAndText01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWidePeekImageAndText01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImageAndText01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 22,
    TileTemplateType_TileWidePeekImageAndText02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWidePeekImageAndText02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImageAndText02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 23,
    TileTemplateType_TileWidePeekImage01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWidePeekImage01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImage01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 24,
    TileTemplateType_TileWidePeekImage02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWidePeekImage02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImage02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 25,
    TileTemplateType_TileWidePeekImage03
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWidePeekImage03 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImage03.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 26,
    TileTemplateType_TileWidePeekImage04
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWidePeekImage04 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImage04.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 27,
    TileTemplateType_TileWidePeekImage05
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWidePeekImage05 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImage05.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 28,
    TileTemplateType_TileWidePeekImage06
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWidePeekImage06 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150PeekImage06.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 29,
    TileTemplateType_TileWideSmallImageAndText01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideSmallImageAndText01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150SmallImageAndText01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 30,
    TileTemplateType_TileWideSmallImageAndText02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideSmallImageAndText02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150SmallImageAndText02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 31,
    TileTemplateType_TileWideSmallImageAndText03
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideSmallImageAndText03 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150SmallImageAndText03.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 32,
    TileTemplateType_TileWideSmallImageAndText04
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideSmallImageAndText04 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150SmallImageAndText04.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 33,
    TileTemplateType_TileWideSmallImageAndText05
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideSmallImageAndText05 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150SmallImageAndText05.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 34,
    TileTemplateType_TileWideText01
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideText01 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text01.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 35,
    TileTemplateType_TileWideText02
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideText02 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text02.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 36,
    TileTemplateType_TileWideText03
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideText03 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text03.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 37,
    TileTemplateType_TileWideText04
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideText04 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text04.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 38,
    TileTemplateType_TileWideText05
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideText05 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text05.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 39,
    TileTemplateType_TileWideText06
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideText06 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text06.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 40,
    TileTemplateType_TileWideText07
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideText07 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text07.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 41,
    TileTemplateType_TileWideText08
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideText08 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text08.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 42,
    TileTemplateType_TileWideText09
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideText09 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text09.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 43,
    TileTemplateType_TileWideText10
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideText10 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text10.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 44,
    TileTemplateType_TileWideText11
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATEDENUMERATOR("TileWideText11 may be altered or unavailable for releases after Windows 8.1. Instead, use TileWide310x150Text11.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    = 45,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare150x150Image = 0,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare150x150Block = 1,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare150x150Text01 = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare150x150Text02 = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare150x150Text03 = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare150x150Text04 = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare150x150PeekImageAndText01 = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare150x150PeekImageAndText02 = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare150x150PeekImageAndText03 = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare150x150PeekImageAndText04 = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150Image = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150ImageCollection = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150ImageAndText01 = 12,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150ImageAndText02 = 13,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150BlockAndText01 = 14,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150BlockAndText02 = 15,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150PeekImageCollection01 = 16,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150PeekImageCollection02 = 17,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150PeekImageCollection03 = 18,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150PeekImageCollection04 = 19,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150PeekImageCollection05 = 20,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150PeekImageCollection06 = 21,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150PeekImageAndText01 = 22,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150PeekImageAndText02 = 23,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150PeekImage01 = 24,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150PeekImage02 = 25,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150PeekImage03 = 26,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150PeekImage04 = 27,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150PeekImage05 = 28,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150PeekImage06 = 29,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150SmallImageAndText01 = 30,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150SmallImageAndText02 = 31,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150SmallImageAndText03 = 32,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150SmallImageAndText04 = 33,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150SmallImageAndText05 = 34,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150Text01 = 35,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150Text02 = 36,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150Text03 = 37,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150Text04 = 38,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150Text05 = 39,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150Text06 = 40,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150Text07 = 41,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150Text08 = 42,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150Text09 = 43,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150Text10 = 44,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150Text11 = 45,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310BlockAndText01 = 46,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310BlockAndText02 = 47,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310Image = 48,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310ImageAndText01 = 49,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310ImageAndText02 = 50,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310ImageAndTextOverlay01 = 51,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310ImageAndTextOverlay02 = 52,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310ImageAndTextOverlay03 = 53,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310ImageCollectionAndText01 = 54,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310ImageCollectionAndText02 = 55,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310ImageCollection = 56,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310SmallImagesAndTextList01 = 57,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310SmallImagesAndTextList02 = 58,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310SmallImagesAndTextList03 = 59,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310SmallImagesAndTextList04 = 60,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310Text01 = 61,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310Text02 = 62,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310Text03 = 63,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310Text04 = 64,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310Text05 = 65,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310Text06 = 66,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310Text07 = 67,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310Text08 = 68,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310TextList01 = 69,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310TextList02 = 70,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310TextList03 = 71,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310SmallImageAndText01 = 72,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310SmallImagesAndTextList05 = 73,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare310x310Text09 = 74,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare71x71IconWithBadge = 75,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare150x150IconWithBadge = 76,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileWide310x150IconWithBadgeAndText = 77,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileSquare71x71Image = 78,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    TileTemplateType_TileTall150x310Image = 79,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Notifications.ToastDismissalReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CNotifications_CToastDismissalReason
{
    ToastDismissalReason_UserCanceled = 0,
    ToastDismissalReason_ApplicationHidden = 1,
    ToastDismissalReason_TimedOut = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Notifications.ToastHistoryChangedType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CNotifications_CToastHistoryChangedType
{
    ToastHistoryChangedType_Cleared = 0,
    ToastHistoryChangedType_Removed = 1,
    ToastHistoryChangedType_Expired = 2,
    ToastHistoryChangedType_Added = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Notifications.ToastNotificationMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
enum __x_ABI_CWindows_CUI_CNotifications_CToastNotificationMode
{
    ToastNotificationMode_Unrestricted = 0,
    ToastNotificationMode_PriorityOnly = 1,
    ToastNotificationMode_AlarmsOnly = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Struct Windows.UI.Notifications.ToastNotificationPriority
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CUI_CNotifications_CToastNotificationPriority
{
    ToastNotificationPriority_Default = 0,
    ToastNotificationPriority_High = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.UI.Notifications.ToastTemplateType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CUI_CNotifications_CToastTemplateType
{
    ToastTemplateType_ToastImageAndText01 = 0,
    ToastTemplateType_ToastImageAndText02 = 1,
    ToastTemplateType_ToastImageAndText03 = 2,
    ToastTemplateType_ToastImageAndText04 = 3,
    ToastTemplateType_ToastText01 = 4,
    ToastTemplateType_ToastText02 = 5,
    ToastTemplateType_ToastText03 = 6,
    ToastTemplateType_ToastText04 = 7,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.UI.Notifications.UserNotificationChangedKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CUI_CNotifications_CUserNotificationChangedKind
{
    UserNotificationChangedKind_Added = 0,
    UserNotificationChangedKind_Removed = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IAdaptiveNotificationContent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IAdaptiveNotificationContent[] = L"Windows.UI.Notifications.IAdaptiveNotificationContent";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CAdaptiveNotificationContentKind* value);
    HRESULT (STDMETHODCALLTYPE* get_Hints)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent* This,
        __FIMap_2_HSTRING_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContentVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_get_Hints(This, value) \
    ((This)->lpVtbl->get_Hints(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationContent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IAdaptiveNotificationText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.AdaptiveNotificationText
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IAdaptiveNotificationText[] = L"Windows.UI.Notifications.IAdaptiveNotificationText";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationTextVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Text)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Language)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Language)(__x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationTextVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationTextVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_put_Text(This, value) \
    ((This)->lpVtbl->put_Text(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_get_Language(This, value) \
    ((This)->lpVtbl->get_Language(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_put_Language(This, value) \
    ((This)->lpVtbl->put_Language(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIAdaptiveNotificationText_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IBadgeNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.BadgeNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IBadgeNotification[] = L"Windows.UI.Notifications.IBadgeNotification";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument** value);
    HRESULT (STDMETHODCALLTYPE* put_ExpirationTime)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_ExpirationTime)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_put_ExpirationTime(This, value) \
    ((This)->lpVtbl->put_ExpirationTime(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_get_ExpirationTime(This, value) \
    ((This)->lpVtbl->get_ExpirationTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IBadgeNotificationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.BadgeNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IBadgeNotificationFactory[] = L"Windows.UI.Notifications.IBadgeNotificationFactory";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateBadgeNotification)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* content,
        __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactoryVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_CreateBadgeNotification(This, content, value) \
    ((This)->lpVtbl->CreateBadgeNotification(This, content, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeNotificationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IBadgeUpdateManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.BadgeUpdateManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IBadgeUpdateManagerForUser[] = L"Windows.UI.Notifications.IBadgeUpdateManagerForUser";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUserVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateBadgeUpdaterForApplication)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser* This,
        __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater** result);
    HRESULT (STDMETHODCALLTYPE* CreateBadgeUpdaterForApplicationWithId)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser* This,
        HSTRING applicationId,
        __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater** result);
    HRESULT (STDMETHODCALLTYPE* CreateBadgeUpdaterForSecondaryTile)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser* This,
        HSTRING tileId,
        __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater** result);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUserVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUserVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_CreateBadgeUpdaterForApplication(This, result) \
    ((This)->lpVtbl->CreateBadgeUpdaterForApplication(This, result))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_CreateBadgeUpdaterForApplicationWithId(This, applicationId, result) \
    ((This)->lpVtbl->CreateBadgeUpdaterForApplicationWithId(This, applicationId, result))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_CreateBadgeUpdaterForSecondaryTile(This, tileId, result) \
    ((This)->lpVtbl->CreateBadgeUpdaterForSecondaryTile(This, tileId, result))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IBadgeUpdateManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.BadgeUpdateManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IBadgeUpdateManagerStatics[] = L"Windows.UI.Notifications.IBadgeUpdateManagerStatics";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateBadgeUpdaterForApplication)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics* This,
        __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater** result);
    HRESULT (STDMETHODCALLTYPE* CreateBadgeUpdaterForApplicationWithId)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics* This,
        HSTRING applicationId,
        __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater** result);
    HRESULT (STDMETHODCALLTYPE* CreateBadgeUpdaterForSecondaryTile)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics* This,
        HSTRING tileId,
        __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater** result);
    HRESULT (STDMETHODCALLTYPE* GetTemplateContent)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CBadgeTemplateType type,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_CreateBadgeUpdaterForApplication(This, result) \
    ((This)->lpVtbl->CreateBadgeUpdaterForApplication(This, result))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_CreateBadgeUpdaterForApplicationWithId(This, applicationId, result) \
    ((This)->lpVtbl->CreateBadgeUpdaterForApplicationWithId(This, applicationId, result))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_CreateBadgeUpdaterForSecondaryTile(This, tileId, result) \
    ((This)->lpVtbl->CreateBadgeUpdaterForSecondaryTile(This, tileId, result))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_GetTemplateContent(This, type, result) \
    ((This)->lpVtbl->GetTemplateContent(This, type, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IBadgeUpdateManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.BadgeUpdateManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IBadgeUpdateManagerStatics2[] = L"Windows.UI.Notifications.IBadgeUpdateManagerStatics2";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerForUser** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_GetForUser(This, user, result) \
    ((This)->lpVtbl->GetForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdateManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IBadgeUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.BadgeUpdater
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IBadgeUpdater[] = L"Windows.UI.Notifications.IBadgeUpdater";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdaterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Update)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater* This,
        __x_ABI_CWindows_CUI_CNotifications_CIBadgeNotification* notification);
    HRESULT (STDMETHODCALLTYPE* Clear)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater* This);
    HRESULT (STDMETHODCALLTYPE* StartPeriodicUpdate)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* badgeContent,
        enum __x_ABI_CWindows_CUI_CNotifications_CPeriodicUpdateRecurrence requestedInterval);
    HRESULT (STDMETHODCALLTYPE* StartPeriodicUpdateAtTime)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* badgeContent,
        struct __x_ABI_CWindows_CFoundation_CDateTime startTime,
        enum __x_ABI_CWindows_CUI_CNotifications_CPeriodicUpdateRecurrence requestedInterval);
    HRESULT (STDMETHODCALLTYPE* StopPeriodicUpdate)(__x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater* This);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdaterVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdaterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_Update(This, notification) \
    ((This)->lpVtbl->Update(This, notification))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_StartPeriodicUpdate(This, badgeContent, requestedInterval) \
    ((This)->lpVtbl->StartPeriodicUpdate(This, badgeContent, requestedInterval))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_StartPeriodicUpdateAtTime(This, badgeContent, startTime, requestedInterval) \
    ((This)->lpVtbl->StartPeriodicUpdateAtTime(This, badgeContent, startTime, requestedInterval))

#define __x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_StopPeriodicUpdate(This) \
    ((This)->lpVtbl->StopPeriodicUpdate(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIBadgeUpdater_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IKnownAdaptiveNotificationHintsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.KnownAdaptiveNotificationHints
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IKnownAdaptiveNotificationHintsStatics[] = L"Windows.UI.Notifications.IKnownAdaptiveNotificationHintsStatics";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Style)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Wrap)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxLines)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MinLines)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TextStacking)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Align)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStaticsVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_get_Style(This, value) \
    ((This)->lpVtbl->get_Style(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_get_Wrap(This, value) \
    ((This)->lpVtbl->get_Wrap(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_get_MaxLines(This, value) \
    ((This)->lpVtbl->get_MaxLines(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_get_MinLines(This, value) \
    ((This)->lpVtbl->get_MinLines(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_get_TextStacking(This, value) \
    ((This)->lpVtbl->get_TextStacking(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_get_Align(This, value) \
    ((This)->lpVtbl->get_Align(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationHintsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IKnownAdaptiveNotificationTextStylesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.KnownAdaptiveNotificationTextStyles
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IKnownAdaptiveNotificationTextStylesStatics[] = L"Windows.UI.Notifications.IKnownAdaptiveNotificationTextStylesStatics";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Caption)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Body)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Base)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Subtitle)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Subheader)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Header)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TitleNumeral)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SubheaderNumeral)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HeaderNumeral)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CaptionSubtle)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BodySubtle)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BaseSubtle)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SubtitleSubtle)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TitleSubtle)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SubheaderSubtle)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SubheaderNumeralSubtle)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HeaderSubtle)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HeaderNumeralSubtle)(__x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStaticsVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_Caption(This, value) \
    ((This)->lpVtbl->get_Caption(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_Body(This, value) \
    ((This)->lpVtbl->get_Body(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_Base(This, value) \
    ((This)->lpVtbl->get_Base(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_Subtitle(This, value) \
    ((This)->lpVtbl->get_Subtitle(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_Subheader(This, value) \
    ((This)->lpVtbl->get_Subheader(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_Header(This, value) \
    ((This)->lpVtbl->get_Header(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_TitleNumeral(This, value) \
    ((This)->lpVtbl->get_TitleNumeral(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_SubheaderNumeral(This, value) \
    ((This)->lpVtbl->get_SubheaderNumeral(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_HeaderNumeral(This, value) \
    ((This)->lpVtbl->get_HeaderNumeral(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_CaptionSubtle(This, value) \
    ((This)->lpVtbl->get_CaptionSubtle(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_BodySubtle(This, value) \
    ((This)->lpVtbl->get_BodySubtle(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_BaseSubtle(This, value) \
    ((This)->lpVtbl->get_BaseSubtle(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_SubtitleSubtle(This, value) \
    ((This)->lpVtbl->get_SubtitleSubtle(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_TitleSubtle(This, value) \
    ((This)->lpVtbl->get_TitleSubtle(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_SubheaderSubtle(This, value) \
    ((This)->lpVtbl->get_SubheaderSubtle(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_SubheaderNumeralSubtle(This, value) \
    ((This)->lpVtbl->get_SubheaderNumeralSubtle(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_HeaderSubtle(This, value) \
    ((This)->lpVtbl->get_HeaderSubtle(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_get_HeaderNumeralSubtle(This, value) \
    ((This)->lpVtbl->get_HeaderNumeralSubtle(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIKnownAdaptiveNotificationTextStylesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IKnownNotificationBindingsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.KnownNotificationBindings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IKnownNotificationBindingsStatics[] = L"Windows.UI.Notifications.IKnownNotificationBindingsStatics";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ToastGeneric)(__x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStaticsVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_get_ToastGeneric(This, value) \
    ((This)->lpVtbl->get_ToastGeneric(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIKnownNotificationBindingsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.INotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.Notification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CINotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CINotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_INotification[] = L"Windows.UI.Notifications.INotification";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CINotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CINotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CINotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CINotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CINotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CINotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CINotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExpirationTime)(__x_ABI_CWindows_CUI_CNotifications_CINotification* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_ExpirationTime)(__x_ABI_CWindows_CUI_CNotifications_CINotification* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Visual)(__x_ABI_CWindows_CUI_CNotifications_CINotification* This,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationVisual** value);
    HRESULT (STDMETHODCALLTYPE* put_Visual)(__x_ABI_CWindows_CUI_CNotifications_CINotification* This,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationVisual* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CINotificationVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CINotification
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CINotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CINotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CINotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CINotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CINotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CINotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CINotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CINotification_get_ExpirationTime(This, value) \
    ((This)->lpVtbl->get_ExpirationTime(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CINotification_put_ExpirationTime(This, value) \
    ((This)->lpVtbl->put_ExpirationTime(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CINotification_get_Visual(This, value) \
    ((This)->lpVtbl->get_Visual(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CINotification_put_Visual(This, value) \
    ((This)->lpVtbl->put_Visual(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CINotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CINotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.INotificationBinding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.NotificationBinding
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_INotificationBinding[] = L"Windows.UI.Notifications.INotificationBinding";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CINotificationBindingVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Template)(__x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Template)(__x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Language)(__x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Language)(__x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Hints)(__x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* This,
        __FIMap_2_HSTRING_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* GetTextElements)(__x_ABI_CWindows_CUI_CNotifications_CINotificationBinding* This,
        __FIVectorView_1_Windows__CUI__CNotifications__CAdaptiveNotificationText** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CINotificationBindingVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CINotificationBindingVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_get_Template(This, value) \
    ((This)->lpVtbl->get_Template(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_put_Template(This, value) \
    ((This)->lpVtbl->put_Template(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_get_Language(This, value) \
    ((This)->lpVtbl->get_Language(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_put_Language(This, value) \
    ((This)->lpVtbl->put_Language(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_get_Hints(This, value) \
    ((This)->lpVtbl->get_Hints(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_GetTextElements(This, result) \
    ((This)->lpVtbl->GetTextElements(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CINotificationBinding;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CINotificationBinding_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.INotificationData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.NotificationData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CINotificationData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CINotificationData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_INotificationData[] = L"Windows.UI.Notifications.INotificationData";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CINotificationDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CINotificationData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CINotificationData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CINotificationData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CINotificationData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CINotificationData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CINotificationData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Values)(__x_ABI_CWindows_CUI_CNotifications_CINotificationData* This,
        __FIMap_2_HSTRING_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_SequenceNumber)(__x_ABI_CWindows_CUI_CNotifications_CINotificationData* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_SequenceNumber)(__x_ABI_CWindows_CUI_CNotifications_CINotificationData* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CINotificationDataVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CINotificationData
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CINotificationDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationData_get_Values(This, value) \
    ((This)->lpVtbl->get_Values(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationData_get_SequenceNumber(This, value) \
    ((This)->lpVtbl->get_SequenceNumber(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationData_put_SequenceNumber(This, value) \
    ((This)->lpVtbl->put_SequenceNumber(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CINotificationData;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CINotificationData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.INotificationDataFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.NotificationData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_INotificationDataFactory[] = L"Windows.UI.Notifications.INotificationDataFactory";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateNotificationDataWithValuesAndSequenceNumber)(__x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory* This,
        __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* initialValues,
        UINT32 sequenceNumber,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationData** value);
    HRESULT (STDMETHODCALLTYPE* CreateNotificationDataWithValues)(__x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory* This,
        __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* initialValues,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationData** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactoryVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_CreateNotificationDataWithValuesAndSequenceNumber(This, initialValues, sequenceNumber, value) \
    ((This)->lpVtbl->CreateNotificationDataWithValuesAndSequenceNumber(This, initialValues, sequenceNumber, value))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_CreateNotificationDataWithValues(This, initialValues, value) \
    ((This)->lpVtbl->CreateNotificationDataWithValues(This, initialValues, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CINotificationDataFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.INotificationVisual
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.NotificationVisual
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_INotificationVisual[] = L"Windows.UI.Notifications.INotificationVisual";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CINotificationVisualVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CINotificationVisual* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CINotificationVisual* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CINotificationVisual* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CINotificationVisual* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CINotificationVisual* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CINotificationVisual* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Language)(__x_ABI_CWindows_CUI_CNotifications_CINotificationVisual* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Language)(__x_ABI_CWindows_CUI_CNotifications_CINotificationVisual* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Bindings)(__x_ABI_CWindows_CUI_CNotifications_CINotificationVisual* This,
        __FIVector_1_Windows__CUI__CNotifications__CNotificationBinding** value);
    HRESULT (STDMETHODCALLTYPE* GetBinding)(__x_ABI_CWindows_CUI_CNotifications_CINotificationVisual* This,
        HSTRING templateName,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationBinding** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CINotificationVisualVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CINotificationVisual
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CINotificationVisualVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_get_Language(This, value) \
    ((This)->lpVtbl->get_Language(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_put_Language(This, value) \
    ((This)->lpVtbl->put_Language(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_get_Bindings(This, value) \
    ((This)->lpVtbl->get_Bindings(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_GetBinding(This, templateName, result) \
    ((This)->lpVtbl->GetBinding(This, templateName, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CINotificationVisual;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CINotificationVisual_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IScheduledTileNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ScheduledTileNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IScheduledTileNotification[] = L"Windows.UI.Notifications.IScheduledTileNotification";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument** value);
    HRESULT (STDMETHODCALLTYPE* get_DeliveryTime)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* put_ExpirationTime)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_ExpirationTime)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_Tag)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Tag)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Id)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_get_DeliveryTime(This, value) \
    ((This)->lpVtbl->get_DeliveryTime(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_put_ExpirationTime(This, value) \
    ((This)->lpVtbl->put_ExpirationTime(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_get_ExpirationTime(This, value) \
    ((This)->lpVtbl->get_ExpirationTime(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_put_Tag(This, value) \
    ((This)->lpVtbl->put_Tag(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_get_Tag(This, value) \
    ((This)->lpVtbl->get_Tag(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_put_Id(This, value) \
    ((This)->lpVtbl->put_Id(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IScheduledTileNotificationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ScheduledTileNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IScheduledTileNotificationFactory[] = L"Windows.UI.Notifications.IScheduledTileNotificationFactory";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateScheduledTileNotification)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* content,
        struct __x_ABI_CWindows_CFoundation_CDateTime deliveryTime,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactoryVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_CreateScheduledTileNotification(This, content, deliveryTime, value) \
    ((This)->lpVtbl->CreateScheduledTileNotification(This, content, deliveryTime, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotificationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IScheduledToastNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ScheduledToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IScheduledToastNotification[] = L"Windows.UI.Notifications.IScheduledToastNotification";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument** value);
    HRESULT (STDMETHODCALLTYPE* get_DeliveryTime)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_SnoozeInterval)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* get_MaximumSnoozeCount)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Id)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_get_DeliveryTime(This, value) \
    ((This)->lpVtbl->get_DeliveryTime(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_get_SnoozeInterval(This, value) \
    ((This)->lpVtbl->get_SnoozeInterval(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_get_MaximumSnoozeCount(This, value) \
    ((This)->lpVtbl->get_MaximumSnoozeCount(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_put_Id(This, value) \
    ((This)->lpVtbl->put_Id(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IScheduledToastNotification2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ScheduledToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IScheduledToastNotification2[] = L"Windows.UI.Notifications.IScheduledToastNotification2";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Tag)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Tag)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Group)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Group)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_SuppressPopup)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_SuppressPopup)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_put_Tag(This, value) \
    ((This)->lpVtbl->put_Tag(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_get_Tag(This, value) \
    ((This)->lpVtbl->get_Tag(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_put_Group(This, value) \
    ((This)->lpVtbl->put_Group(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_get_Group(This, value) \
    ((This)->lpVtbl->get_Group(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_put_SuppressPopup(This, value) \
    ((This)->lpVtbl->put_SuppressPopup(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_get_SuppressPopup(This, value) \
    ((This)->lpVtbl->get_SuppressPopup(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IScheduledToastNotification3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ScheduledToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IScheduledToastNotification3[] = L"Windows.UI.Notifications.IScheduledToastNotification3";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NotificationMirroring)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CNotificationMirroring* value);
    HRESULT (STDMETHODCALLTYPE* put_NotificationMirroring)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CNotificationMirroring value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteId)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_RemoteId)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_get_NotificationMirroring(This, value) \
    ((This)->lpVtbl->get_NotificationMirroring(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_put_NotificationMirroring(This, value) \
    ((This)->lpVtbl->put_NotificationMirroring(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_get_RemoteId(This, value) \
    ((This)->lpVtbl->get_RemoteId(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_put_RemoteId(This, value) \
    ((This)->lpVtbl->put_RemoteId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IScheduledToastNotification4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ScheduledToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IScheduledToastNotification4[] = L"Windows.UI.Notifications.IScheduledToastNotification4";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExpirationTime)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_ExpirationTime)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_get_ExpirationTime(This, value) \
    ((This)->lpVtbl->get_ExpirationTime(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_put_ExpirationTime(This, value) \
    ((This)->lpVtbl->put_ExpirationTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.UI.Notifications.IScheduledToastNotificationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ScheduledToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IScheduledToastNotificationFactory[] = L"Windows.UI.Notifications.IScheduledToastNotificationFactory";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateScheduledToastNotification)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* content,
        struct __x_ABI_CWindows_CFoundation_CDateTime deliveryTime,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification** value);
    HRESULT (STDMETHODCALLTYPE* CreateScheduledToastNotificationRecurring)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* content,
        struct __x_ABI_CWindows_CFoundation_CDateTime deliveryTime,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan snoozeInterval,
        UINT32 maximumSnoozeCount,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactoryVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_CreateScheduledToastNotification(This, content, deliveryTime, value) \
    ((This)->lpVtbl->CreateScheduledToastNotification(This, content, deliveryTime, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_CreateScheduledToastNotificationRecurring(This, content, deliveryTime, snoozeInterval, maximumSnoozeCount, value) \
    ((This)->lpVtbl->CreateScheduledToastNotificationRecurring(This, content, deliveryTime, snoozeInterval, maximumSnoozeCount, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IScheduledToastNotificationShowingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ScheduledToastNotificationShowingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IScheduledToastNotificationShowingEventArgs[] = L"Windows.UI.Notifications.IScheduledToastNotificationShowingEventArgs";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Cancel)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_Cancel)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ScheduledToastNotification)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs* This,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_get_Cancel(This, value) \
    ((This)->lpVtbl->get_Cancel(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_put_Cancel(This, value) \
    ((This)->lpVtbl->put_Cancel(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_get_ScheduledToastNotification(This, value) \
    ((This)->lpVtbl->get_ScheduledToastNotification(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotificationShowingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Notifications.IShownTileNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ShownTileNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IShownTileNotification[] = L"Windows.UI.Notifications.IShownTileNotification";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Arguments)(__x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotificationVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_get_Arguments(This, value) \
    ((This)->lpVtbl->get_Arguments(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIShownTileNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.ITileFlyoutNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileFlyoutNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileFlyoutNotification[] = L"Windows.UI.Notifications.ITileFlyoutNotification";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument** value);
    HRESULT (STDMETHODCALLTYPE* put_ExpirationTime)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_ExpirationTime)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_put_ExpirationTime(This, value) \
    ((This)->lpVtbl->put_ExpirationTime(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_get_ExpirationTime(This, value) \
    ((This)->lpVtbl->get_ExpirationTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.ITileFlyoutNotificationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileFlyoutNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileFlyoutNotificationFactory[] = L"Windows.UI.Notifications.ITileFlyoutNotificationFactory";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateTileFlyoutNotification)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* content,
        __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactoryVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_CreateTileFlyoutNotification(This, content, value) \
    ((This)->lpVtbl->CreateTileFlyoutNotification(This, content, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotificationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.ITileFlyoutUpdateManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileFlyoutUpdateManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileFlyoutUpdateManagerStatics[] = L"Windows.UI.Notifications.ITileFlyoutUpdateManagerStatics";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateTileFlyoutUpdaterForApplication)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics* This,
        __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater** result);
    HRESULT (STDMETHODCALLTYPE* CreateTileFlyoutUpdaterForApplicationWithId)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics* This,
        HSTRING applicationId,
        __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater** result);
    HRESULT (STDMETHODCALLTYPE* CreateTileFlyoutUpdaterForSecondaryTile)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics* This,
        HSTRING tileId,
        __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater** result);
    HRESULT (STDMETHODCALLTYPE* GetTemplateContent)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CTileFlyoutTemplateType type,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_CreateTileFlyoutUpdaterForApplication(This, result) \
    ((This)->lpVtbl->CreateTileFlyoutUpdaterForApplication(This, result))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_CreateTileFlyoutUpdaterForApplicationWithId(This, applicationId, result) \
    ((This)->lpVtbl->CreateTileFlyoutUpdaterForApplicationWithId(This, applicationId, result))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_CreateTileFlyoutUpdaterForSecondaryTile(This, tileId, result) \
    ((This)->lpVtbl->CreateTileFlyoutUpdaterForSecondaryTile(This, tileId, result))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_GetTemplateContent(This, type, result) \
    ((This)->lpVtbl->GetTemplateContent(This, type, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdateManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.ITileFlyoutUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileFlyoutUpdater
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileFlyoutUpdater[] = L"Windows.UI.Notifications.ITileFlyoutUpdater";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdaterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Update)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater* This,
        __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutNotification* notification);
    HRESULT (STDMETHODCALLTYPE* Clear)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater* This);
    HRESULT (STDMETHODCALLTYPE* StartPeriodicUpdate)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* tileFlyoutContent,
        enum __x_ABI_CWindows_CUI_CNotifications_CPeriodicUpdateRecurrence requestedInterval);
    HRESULT (STDMETHODCALLTYPE* StartPeriodicUpdateAtTime)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* tileFlyoutContent,
        struct __x_ABI_CWindows_CFoundation_CDateTime startTime,
        enum __x_ABI_CWindows_CUI_CNotifications_CPeriodicUpdateRecurrence requestedInterval);
    HRESULT (STDMETHODCALLTYPE* StopPeriodicUpdate)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater* This);
    HRESULT (STDMETHODCALLTYPE* get_Setting)(__x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CNotificationSetting* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdaterVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdaterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_Update(This, notification) \
    ((This)->lpVtbl->Update(This, notification))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_StartPeriodicUpdate(This, tileFlyoutContent, requestedInterval) \
    ((This)->lpVtbl->StartPeriodicUpdate(This, tileFlyoutContent, requestedInterval))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_StartPeriodicUpdateAtTime(This, tileFlyoutContent, startTime, requestedInterval) \
    ((This)->lpVtbl->StartPeriodicUpdateAtTime(This, tileFlyoutContent, startTime, requestedInterval))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_StopPeriodicUpdate(This) \
    ((This)->lpVtbl->StopPeriodicUpdate(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_get_Setting(This, value) \
    ((This)->lpVtbl->get_Setting(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileFlyoutUpdater_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.ITileNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileNotification[] = L"Windows.UI.Notifications.ITileNotification";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CITileNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CITileNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CITileNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CITileNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CITileNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CITileNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CITileNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CUI_CNotifications_CITileNotification* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument** value);
    HRESULT (STDMETHODCALLTYPE* put_ExpirationTime)(__x_ABI_CWindows_CUI_CNotifications_CITileNotification* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_ExpirationTime)(__x_ABI_CWindows_CUI_CNotifications_CITileNotification* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_Tag)(__x_ABI_CWindows_CUI_CNotifications_CITileNotification* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Tag)(__x_ABI_CWindows_CUI_CNotifications_CITileNotification* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CITileNotificationVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CITileNotification
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CITileNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotification_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotification_put_ExpirationTime(This, value) \
    ((This)->lpVtbl->put_ExpirationTime(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotification_get_ExpirationTime(This, value) \
    ((This)->lpVtbl->get_ExpirationTime(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotification_put_Tag(This, value) \
    ((This)->lpVtbl->put_Tag(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotification_get_Tag(This, value) \
    ((This)->lpVtbl->get_Tag(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileNotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.ITileNotificationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileNotificationFactory[] = L"Windows.UI.Notifications.ITileNotificationFactory";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateTileNotification)(__x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* content,
        __x_ABI_CWindows_CUI_CNotifications_CITileNotification** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactoryVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_CreateTileNotification(This, content, value) \
    ((This)->lpVtbl->CreateTileNotification(This, content, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileNotificationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.ITileUpdateManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileUpdateManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileUpdateManagerForUser[] = L"Windows.UI.Notifications.ITileUpdateManagerForUser";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUserVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateTileUpdaterForApplication)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser* This,
        __x_ABI_CWindows_CUI_CNotifications_CITileUpdater** result);
    HRESULT (STDMETHODCALLTYPE* CreateTileUpdaterForApplicationWithId)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser* This,
        HSTRING applicationId,
        __x_ABI_CWindows_CUI_CNotifications_CITileUpdater** result);
    HRESULT (STDMETHODCALLTYPE* CreateTileUpdaterForSecondaryTile)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser* This,
        HSTRING tileId,
        __x_ABI_CWindows_CUI_CNotifications_CITileUpdater** result);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUserVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUserVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_CreateTileUpdaterForApplication(This, result) \
    ((This)->lpVtbl->CreateTileUpdaterForApplication(This, result))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_CreateTileUpdaterForApplicationWithId(This, applicationId, result) \
    ((This)->lpVtbl->CreateTileUpdaterForApplicationWithId(This, applicationId, result))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_CreateTileUpdaterForSecondaryTile(This, tileId, result) \
    ((This)->lpVtbl->CreateTileUpdaterForSecondaryTile(This, tileId, result))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.ITileUpdateManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileUpdateManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileUpdateManagerStatics[] = L"Windows.UI.Notifications.ITileUpdateManagerStatics";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateTileUpdaterForApplication)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics* This,
        __x_ABI_CWindows_CUI_CNotifications_CITileUpdater** result);
    HRESULT (STDMETHODCALLTYPE* CreateTileUpdaterForApplicationWithId)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics* This,
        HSTRING applicationId,
        __x_ABI_CWindows_CUI_CNotifications_CITileUpdater** result);
    HRESULT (STDMETHODCALLTYPE* CreateTileUpdaterForSecondaryTile)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics* This,
        HSTRING tileId,
        __x_ABI_CWindows_CUI_CNotifications_CITileUpdater** result);
    HRESULT (STDMETHODCALLTYPE* GetTemplateContent)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CTileTemplateType type,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_CreateTileUpdaterForApplication(This, result) \
    ((This)->lpVtbl->CreateTileUpdaterForApplication(This, result))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_CreateTileUpdaterForApplicationWithId(This, applicationId, result) \
    ((This)->lpVtbl->CreateTileUpdaterForApplicationWithId(This, applicationId, result))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_CreateTileUpdaterForSecondaryTile(This, tileId, result) \
    ((This)->lpVtbl->CreateTileUpdaterForSecondaryTile(This, tileId, result))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_GetTemplateContent(This, type, result) \
    ((This)->lpVtbl->GetTemplateContent(This, type, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.ITileUpdateManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileUpdateManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileUpdateManagerStatics2[] = L"Windows.UI.Notifications.ITileUpdateManagerStatics2";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerForUser** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_GetForUser(This, user, result) \
    ((This)->lpVtbl->GetForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdateManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.ITileUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileUpdater
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileUpdater[] = L"Windows.UI.Notifications.ITileUpdater";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CITileUpdaterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Update)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This,
        __x_ABI_CWindows_CUI_CNotifications_CITileNotification* notification);
    HRESULT (STDMETHODCALLTYPE* Clear)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This);
    HRESULT (STDMETHODCALLTYPE* EnableNotificationQueue)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This,
        boolean enable);
    HRESULT (STDMETHODCALLTYPE* get_Setting)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CNotificationSetting* value);
    HRESULT (STDMETHODCALLTYPE* AddToSchedule)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* scheduledTile);
    HRESULT (STDMETHODCALLTYPE* RemoveFromSchedule)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledTileNotification* scheduledTile);
    HRESULT (STDMETHODCALLTYPE* GetScheduledTileNotifications)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This,
        __FIVectorView_1_Windows__CUI__CNotifications__CScheduledTileNotification** result);
    HRESULT (STDMETHODCALLTYPE* StartPeriodicUpdate)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* tileContent,
        enum __x_ABI_CWindows_CUI_CNotifications_CPeriodicUpdateRecurrence requestedInterval);
    HRESULT (STDMETHODCALLTYPE* StartPeriodicUpdateAtTime)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* tileContent,
        struct __x_ABI_CWindows_CFoundation_CDateTime startTime,
        enum __x_ABI_CWindows_CUI_CNotifications_CPeriodicUpdateRecurrence requestedInterval);
    HRESULT (STDMETHODCALLTYPE* StopPeriodicUpdate)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This);
    HRESULT (STDMETHODCALLTYPE* StartPeriodicUpdateBatch)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This,
        __FIIterable_1_Windows__CFoundation__CUri* tileContents,
        enum __x_ABI_CWindows_CUI_CNotifications_CPeriodicUpdateRecurrence requestedInterval);
    HRESULT (STDMETHODCALLTYPE* StartPeriodicUpdateBatchAtTime)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater* This,
        __FIIterable_1_Windows__CFoundation__CUri* tileContents,
        struct __x_ABI_CWindows_CFoundation_CDateTime startTime,
        enum __x_ABI_CWindows_CUI_CNotifications_CPeriodicUpdateRecurrence requestedInterval);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CITileUpdaterVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CITileUpdater
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CITileUpdaterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_Update(This, notification) \
    ((This)->lpVtbl->Update(This, notification))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_EnableNotificationQueue(This, enable) \
    ((This)->lpVtbl->EnableNotificationQueue(This, enable))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_get_Setting(This, value) \
    ((This)->lpVtbl->get_Setting(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_AddToSchedule(This, scheduledTile) \
    ((This)->lpVtbl->AddToSchedule(This, scheduledTile))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_RemoveFromSchedule(This, scheduledTile) \
    ((This)->lpVtbl->RemoveFromSchedule(This, scheduledTile))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_GetScheduledTileNotifications(This, result) \
    ((This)->lpVtbl->GetScheduledTileNotifications(This, result))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_StartPeriodicUpdate(This, tileContent, requestedInterval) \
    ((This)->lpVtbl->StartPeriodicUpdate(This, tileContent, requestedInterval))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_StartPeriodicUpdateAtTime(This, tileContent, startTime, requestedInterval) \
    ((This)->lpVtbl->StartPeriodicUpdateAtTime(This, tileContent, startTime, requestedInterval))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_StopPeriodicUpdate(This) \
    ((This)->lpVtbl->StopPeriodicUpdate(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_StartPeriodicUpdateBatch(This, tileContents, requestedInterval) \
    ((This)->lpVtbl->StartPeriodicUpdateBatch(This, tileContents, requestedInterval))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater_StartPeriodicUpdateBatchAtTime(This, tileContents, startTime, requestedInterval) \
    ((This)->lpVtbl->StartPeriodicUpdateBatchAtTime(This, tileContents, startTime, requestedInterval))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileUpdater;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdater_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.ITileUpdater2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.TileUpdater
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_ITileUpdater2[] = L"Windows.UI.Notifications.ITileUpdater2";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CITileUpdater2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* EnableNotificationQueueForSquare150x150)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater2* This,
        boolean enable);
    HRESULT (STDMETHODCALLTYPE* EnableNotificationQueueForWide310x150)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater2* This,
        boolean enable);
    HRESULT (STDMETHODCALLTYPE* EnableNotificationQueueForSquare310x310)(__x_ABI_CWindows_CUI_CNotifications_CITileUpdater2* This,
        boolean enable);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CITileUpdater2Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CITileUpdater2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CITileUpdater2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_EnableNotificationQueueForSquare150x150(This, enable) \
    ((This)->lpVtbl->EnableNotificationQueueForSquare150x150(This, enable))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_EnableNotificationQueueForWide310x150(This, enable) \
    ((This)->lpVtbl->EnableNotificationQueueForWide310x150(This, enable))

#define __x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_EnableNotificationQueueForSquare310x310(This, enable) \
    ((This)->lpVtbl->EnableNotificationQueueForSquare310x310(This, enable))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CITileUpdater2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CITileUpdater2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastActivatedEventArgs[] = L"Windows.UI.Notifications.IToastActivatedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Arguments)(__x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_get_Arguments(This, value) \
    ((This)->lpVtbl->get_Arguments(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastActivatedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastActivatedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastActivatedEventArgs2[] = L"Windows.UI.Notifications.IToastActivatedEventArgs2";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UserInput)(__x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_get_UserInput(This, value) \
    ((This)->lpVtbl->get_UserInput(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastActivatedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Notifications.IToastCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastCollection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastCollection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastCollection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastCollection[] = L"Windows.UI.Notifications.IToastCollection";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollection* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollection* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollection* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_LaunchArgs)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollection* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_LaunchArgs)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollection* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Icon)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollection* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_Icon)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollection* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastCollection
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollection_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollection_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollection_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollection_get_LaunchArgs(This, value) \
    ((This)->lpVtbl->get_LaunchArgs(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollection_put_LaunchArgs(This, value) \
    ((This)->lpVtbl->put_LaunchArgs(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollection_get_Icon(This, value) \
    ((This)->lpVtbl->get_Icon(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollection_put_Icon(This, value) \
    ((This)->lpVtbl->put_Icon(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastCollection;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastCollection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.IToastCollectionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastCollection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastCollectionFactory[] = L"Windows.UI.Notifications.IToastCollectionFactory";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory* This,
        HSTRING collectionId,
        HSTRING displayName,
        HSTRING launchArgs,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* iconUri,
        __x_ABI_CWindows_CUI_CNotifications_CIToastCollection** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactoryVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_CreateInstance(This, collectionId, displayName, launchArgs, iconUri, value) \
    ((This)->lpVtbl->CreateInstance(This, collectionId, displayName, launchArgs, iconUri, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.IToastCollectionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastCollectionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastCollectionManager[] = L"Windows.UI.Notifications.IToastCollectionManager";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SaveToastCollectionAsync)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastCollection* collection,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* FindAllToastCollectionsAsync)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CUI__CNotifications__CToastCollection** operation);
    HRESULT (STDMETHODCALLTYPE* GetToastCollectionAsync)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager* This,
        HSTRING collectionId,
        __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastCollection** operation);
    HRESULT (STDMETHODCALLTYPE* RemoveToastCollectionAsync)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager* This,
        HSTRING collectionId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* RemoveAllToastCollectionsAsync)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager* This,
        __x_ABI_CWindows_CSystem_CIUser** value);
    HRESULT (STDMETHODCALLTYPE* get_AppId)(__x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManagerVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_SaveToastCollectionAsync(This, collection, operation) \
    ((This)->lpVtbl->SaveToastCollectionAsync(This, collection, operation))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_FindAllToastCollectionsAsync(This, operation) \
    ((This)->lpVtbl->FindAllToastCollectionsAsync(This, operation))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_GetToastCollectionAsync(This, collectionId, operation) \
    ((This)->lpVtbl->GetToastCollectionAsync(This, collectionId, operation))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_RemoveToastCollectionAsync(This, collectionId, operation) \
    ((This)->lpVtbl->RemoveToastCollectionAsync(This, collectionId, operation))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_RemoveAllToastCollectionsAsync(This, operation) \
    ((This)->lpVtbl->RemoveAllToastCollectionsAsync(This, operation))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_get_AppId(This, value) \
    ((This)->lpVtbl->get_AppId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.IToastDismissedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastDismissedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastDismissedEventArgs[] = L"Windows.UI.Notifications.IToastDismissedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Reason)(__x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CToastDismissalReason* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_get_Reason(This, value) \
    ((This)->lpVtbl->get_Reason(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastDismissedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastFailedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastFailedEventArgs[] = L"Windows.UI.Notifications.IToastFailedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ErrorCode)(__x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_get_ErrorCode(This, value) \
    ((This)->lpVtbl->get_ErrorCode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastFailedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotification[] = L"Windows.UI.Notifications.IToastNotification";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Content)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument** value);
    HRESULT (STDMETHODCALLTYPE* put_ExpirationTime)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_ExpirationTime)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* add_Dismissed)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification* This,
        __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastDismissedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Dismissed)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Activated)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification* This,
        __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Activated)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Failed)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification* This,
        __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotification_Windows__CUI__CNotifications__CToastFailedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Failed)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotification
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification_get_Content(This, value) \
    ((This)->lpVtbl->get_Content(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification_put_ExpirationTime(This, value) \
    ((This)->lpVtbl->put_ExpirationTime(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification_get_ExpirationTime(This, value) \
    ((This)->lpVtbl->get_ExpirationTime(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification_add_Dismissed(This, handler, token) \
    ((This)->lpVtbl->add_Dismissed(This, handler, token))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification_remove_Dismissed(This, token) \
    ((This)->lpVtbl->remove_Dismissed(This, token))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification_add_Activated(This, handler, token) \
    ((This)->lpVtbl->add_Activated(This, handler, token))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification_remove_Activated(This, token) \
    ((This)->lpVtbl->remove_Activated(This, token))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification_add_Failed(This, handler, token) \
    ((This)->lpVtbl->add_Failed(This, handler, token))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification_remove_Failed(This, token) \
    ((This)->lpVtbl->remove_Failed(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotification2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotification2[] = L"Windows.UI.Notifications.IToastNotification2";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Tag)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Tag)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Group)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Group)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_SuppressPopup)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_SuppressPopup)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_put_Tag(This, value) \
    ((This)->lpVtbl->put_Tag(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_get_Tag(This, value) \
    ((This)->lpVtbl->get_Tag(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_put_Group(This, value) \
    ((This)->lpVtbl->put_Group(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_get_Group(This, value) \
    ((This)->lpVtbl->get_Group(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_put_SuppressPopup(This, value) \
    ((This)->lpVtbl->put_SuppressPopup(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_get_SuppressPopup(This, value) \
    ((This)->lpVtbl->get_SuppressPopup(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotification2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotification3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotification3[] = L"Windows.UI.Notifications.IToastNotification3";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NotificationMirroring)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification3* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CNotificationMirroring* value);
    HRESULT (STDMETHODCALLTYPE* put_NotificationMirroring)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification3* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CNotificationMirroring value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteId)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_RemoteId)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification3* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_get_NotificationMirroring(This, value) \
    ((This)->lpVtbl->get_NotificationMirroring(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_put_NotificationMirroring(This, value) \
    ((This)->lpVtbl->put_NotificationMirroring(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_get_RemoteId(This, value) \
    ((This)->lpVtbl->get_RemoteId(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_put_RemoteId(This, value) \
    ((This)->lpVtbl->put_RemoteId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotification3;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotification4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotification4[] = L"Windows.UI.Notifications.IToastNotification4";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Data)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification4* This,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationData** value);
    HRESULT (STDMETHODCALLTYPE* put_Data)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification4* This,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationData* value);
    HRESULT (STDMETHODCALLTYPE* get_Priority)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification4* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CToastNotificationPriority* value);
    HRESULT (STDMETHODCALLTYPE* put_Priority)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification4* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CToastNotificationPriority value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_get_Data(This, value) \
    ((This)->lpVtbl->get_Data(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_put_Data(This, value) \
    ((This)->lpVtbl->put_Data(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_get_Priority(This, value) \
    ((This)->lpVtbl->get_Priority(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_put_Priority(This, value) \
    ((This)->lpVtbl->put_Priority(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotification4;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotification6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotification6[] = L"Windows.UI.Notifications.IToastNotification6";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotification6Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification6* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification6* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification6* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification6* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification6* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification6* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExpiresOnReboot)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification6* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ExpiresOnReboot)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotification6* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotification6Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotification6
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotification6Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_get_ExpiresOnReboot(This, value) \
    ((This)->lpVtbl->get_ExpiresOnReboot(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_put_ExpiresOnReboot(This, value) \
    ((This)->lpVtbl->put_ExpiresOnReboot(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotification6;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotification6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationActionTriggerDetail
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationActionTriggerDetail
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationActionTriggerDetail[] = L"Windows.UI.Notifications.IToastNotificationActionTriggerDetail";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetailVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Argument)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_UserInput)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetailVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetailVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_get_Argument(This, value) \
    ((This)->lpVtbl->get_Argument(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_get_UserInput(This, value) \
    ((This)->lpVtbl->get_UserInput(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationActionTriggerDetail_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationFactory[] = L"Windows.UI.Notifications.IToastNotificationFactory";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateToastNotification)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory* This,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument* content,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactoryVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_CreateToastNotification(This, content, value) \
    ((This)->lpVtbl->CreateToastNotification(This, content, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationHistory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationHistory
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationHistory[] = L"Windows.UI.Notifications.IToastNotificationHistory";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RemoveGroup)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory* This,
        HSTRING group);
    HRESULT (STDMETHODCALLTYPE* RemoveGroupWithId)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory* This,
        HSTRING group,
        HSTRING applicationId);
    HRESULT (STDMETHODCALLTYPE* RemoveGroupedTagWithId)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory* This,
        HSTRING tag,
        HSTRING group,
        HSTRING applicationId);
    HRESULT (STDMETHODCALLTYPE* RemoveGroupedTag)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory* This,
        HSTRING tag,
        HSTRING group);
    HRESULT (STDMETHODCALLTYPE* Remove)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory* This,
        HSTRING tag);
    HRESULT (STDMETHODCALLTYPE* Clear)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory* This);
    HRESULT (STDMETHODCALLTYPE* ClearWithId)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory* This,
        HSTRING applicationId);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_RemoveGroup(This, group) \
    ((This)->lpVtbl->RemoveGroup(This, group))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_RemoveGroupWithId(This, group, applicationId) \
    ((This)->lpVtbl->RemoveGroupWithId(This, group, applicationId))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_RemoveGroupedTagWithId(This, tag, group, applicationId) \
    ((This)->lpVtbl->RemoveGroupedTagWithId(This, tag, group, applicationId))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_RemoveGroupedTag(This, tag, group) \
    ((This)->lpVtbl->RemoveGroupedTag(This, tag, group))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_Remove(This, tag) \
    ((This)->lpVtbl->Remove(This, tag))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_ClearWithId(This, applicationId) \
    ((This)->lpVtbl->ClearWithId(This, applicationId))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationHistory2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationHistory
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationHistory2[] = L"Windows.UI.Notifications.IToastNotificationHistory2";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetHistory)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2* This,
        __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification** result);
    HRESULT (STDMETHODCALLTYPE* GetHistoryWithId)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2* This,
        HSTRING applicationId,
        __FIVectorView_1_Windows__CUI__CNotifications__CToastNotification** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_GetHistory(This, result) \
    ((This)->lpVtbl->GetHistory(This, result))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_GetHistoryWithId(This, applicationId, result) \
    ((This)->lpVtbl->GetHistoryWithId(This, applicationId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationHistoryChangedTriggerDetail
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationHistoryChangedTriggerDetail[] = L"Windows.UI.Notifications.IToastNotificationHistoryChangedTriggerDetail";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetailVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChangeType)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CToastHistoryChangedType* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetailVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetailVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_get_ChangeType(This, value) \
    ((This)->lpVtbl->get_ChangeType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationHistoryChangedTriggerDetail2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationHistoryChangedTriggerDetail2[] = L"Windows.UI.Notifications.IToastNotificationHistoryChangedTriggerDetail2";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CollectionId)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_get_CollectionId(This, value) \
    ((This)->lpVtbl->get_CollectionId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistoryChangedTriggerDetail2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationManagerForUser[] = L"Windows.UI.Notifications.IToastNotificationManagerForUser";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUserVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateToastNotifier)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier** result);
    HRESULT (STDMETHODCALLTYPE* CreateToastNotifierWithId)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser* This,
        HSTRING applicationId,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier** result);
    HRESULT (STDMETHODCALLTYPE* get_History)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory** value);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUserVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUserVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_CreateToastNotifier(This, result) \
    ((This)->lpVtbl->CreateToastNotifier(This, result))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_CreateToastNotifierWithId(This, applicationId, result) \
    ((This)->lpVtbl->CreateToastNotifierWithId(This, applicationId, result))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_get_History(This, value) \
    ((This)->lpVtbl->get_History(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationManagerForUser2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationManagerForUser2[] = L"Windows.UI.Notifications.IToastNotificationManagerForUser2";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetToastNotifierForToastCollectionIdAsync)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2* This,
        HSTRING collectionId,
        __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotifier** operation);
    HRESULT (STDMETHODCALLTYPE* GetHistoryForToastCollectionIdAsync)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2* This,
        HSTRING collectionId,
        __FIAsyncOperation_1_Windows__CUI__CNotifications__CToastNotificationHistory** operation);
    HRESULT (STDMETHODCALLTYPE* GetToastCollectionManager)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager** result);
    HRESULT (STDMETHODCALLTYPE* GetToastCollectionManagerWithAppId)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2* This,
        HSTRING appId,
        __x_ABI_CWindows_CUI_CNotifications_CIToastCollectionManager** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_GetToastNotifierForToastCollectionIdAsync(This, collectionId, operation) \
    ((This)->lpVtbl->GetToastNotifierForToastCollectionIdAsync(This, collectionId, operation))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_GetHistoryForToastCollectionIdAsync(This, collectionId, operation) \
    ((This)->lpVtbl->GetHistoryForToastCollectionIdAsync(This, collectionId, operation))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_GetToastCollectionManager(This, result) \
    ((This)->lpVtbl->GetToastCollectionManager(This, result))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_GetToastCollectionManagerWithAppId(This, appId, result) \
    ((This)->lpVtbl->GetToastCollectionManagerWithAppId(This, appId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationManagerForUser3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationManagerForUser3[] = L"Windows.UI.Notifications.IToastNotificationManagerForUser3";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_NotificationMode)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CToastNotificationMode* value);
    HRESULT (STDMETHODCALLTYPE* add_NotificationModeChanged)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3* This,
        __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotificationManagerForUser_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_NotificationModeChanged)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_get_NotificationMode(This, value) \
    ((This)->lpVtbl->get_NotificationMode(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_add_NotificationModeChanged(This, handler, token) \
    ((This)->lpVtbl->add_NotificationModeChanged(This, handler, token))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_remove_NotificationModeChanged(This, token) \
    ((This)->lpVtbl->remove_NotificationModeChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationManagerStatics[] = L"Windows.UI.Notifications.IToastNotificationManagerStatics";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateToastNotifier)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier** result);
    HRESULT (STDMETHODCALLTYPE* CreateToastNotifierWithId)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics* This,
        HSTRING applicationId,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier** result);
    HRESULT (STDMETHODCALLTYPE* GetTemplateContent)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CToastTemplateType type,
        __x_ABI_CWindows_CData_CXml_CDom_CIXmlDocument** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStaticsVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_CreateToastNotifier(This, result) \
    ((This)->lpVtbl->CreateToastNotifier(This, result))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_CreateToastNotifierWithId(This, applicationId, result) \
    ((This)->lpVtbl->CreateToastNotifierWithId(This, applicationId, result))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_GetTemplateContent(This, type, result) \
    ((This)->lpVtbl->GetTemplateContent(This, type, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationManagerStatics2[] = L"Windows.UI.Notifications.IToastNotificationManagerStatics2";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_History)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationHistory** value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_get_History(This, value) \
    ((This)->lpVtbl->get_History(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationManagerStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationManagerStatics4[] = L"Windows.UI.Notifications.IToastNotificationManagerStatics4";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser** result);
    HRESULT (STDMETHODCALLTYPE* ConfigureNotificationMirroring)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CNotificationMirroring value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_GetForUser(This, user, result) \
    ((This)->lpVtbl->GetForUser(This, user, result))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_ConfigureNotificationMirroring(This, value) \
    ((This)->lpVtbl->ConfigureNotificationMirroring(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotificationManagerStatics5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotificationManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotificationManagerStatics5[] = L"Windows.UI.Notifications.IToastNotificationManagerStatics5";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerForUser** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotificationManagerStatics5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotifier[] = L"Windows.UI.Notifications.IToastNotifier";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Show)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification* notification);
    HRESULT (STDMETHODCALLTYPE* Hide)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification* notification);
    HRESULT (STDMETHODCALLTYPE* get_Setting)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CNotificationSetting* value);
    HRESULT (STDMETHODCALLTYPE* AddToSchedule)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier* This,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification* scheduledToast);
    HRESULT (STDMETHODCALLTYPE* RemoveFromSchedule)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier* This,
        __x_ABI_CWindows_CUI_CNotifications_CIScheduledToastNotification* scheduledToast);
    HRESULT (STDMETHODCALLTYPE* GetScheduledToastNotifications)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier* This,
        __FIVectorView_1_Windows__CUI__CNotifications__CScheduledToastNotification** result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotifierVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_Show(This, notification) \
    ((This)->lpVtbl->Show(This, notification))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_Hide(This, notification) \
    ((This)->lpVtbl->Hide(This, notification))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_get_Setting(This, value) \
    ((This)->lpVtbl->get_Setting(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_AddToSchedule(This, scheduledToast) \
    ((This)->lpVtbl->AddToSchedule(This, scheduledToast))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_RemoveFromSchedule(This, scheduledToast) \
    ((This)->lpVtbl->RemoveFromSchedule(This, scheduledToast))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_GetScheduledToastNotifications(This, result) \
    ((This)->lpVtbl->GetScheduledToastNotifications(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotifier;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotifier2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotifier2[] = L"Windows.UI.Notifications.IToastNotifier2";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* UpdateWithTagAndGroup)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2* This,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationData* data,
        HSTRING tag,
        HSTRING group,
        enum __x_ABI_CWindows_CUI_CNotifications_CNotificationUpdateResult* result);
    HRESULT (STDMETHODCALLTYPE* UpdateWithTag)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2* This,
        __x_ABI_CWindows_CUI_CNotifications_CINotificationData* data,
        HSTRING tag,
        enum __x_ABI_CWindows_CUI_CNotifications_CNotificationUpdateResult* result);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_UpdateWithTagAndGroup(This, data, tag, group, result) \
    ((This)->lpVtbl->UpdateWithTagAndGroup(This, data, tag, group, result))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_UpdateWithTag(This, data, tag, result) \
    ((This)->lpVtbl->UpdateWithTag(This, data, tag, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.UI.Notifications.IToastNotifier3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.ToastNotifier
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IToastNotifier3[] = L"Windows.UI.Notifications.IToastNotifier3";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ScheduledToastNotificationShowing)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3* This,
        __FITypedEventHandler_2_Windows__CUI__CNotifications__CToastNotifier_Windows__CUI__CNotifications__CScheduledToastNotificationShowingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ScheduledToastNotificationShowing)(__x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3Vtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_add_ScheduledToastNotificationShowing(This, handler, token) \
    ((This)->lpVtbl->add_ScheduledToastNotificationShowing(This, handler, token))

#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_remove_ScheduledToastNotificationShowing(This, token) \
    ((This)->lpVtbl->remove_ScheduledToastNotificationShowing(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIToastNotifier3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.UI.Notifications.IUserNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.UserNotification
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IUserNotification[] = L"Windows.UI.Notifications.IUserNotification";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotification* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotification* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotification* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotification* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotification* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotification* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Notification)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotification* This,
        __x_ABI_CWindows_CUI_CNotifications_CINotification** value);
    HRESULT (STDMETHODCALLTYPE* get_AppInfo)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotification* This,
        __x_ABI_CWindows_CApplicationModel_CIAppInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotification* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_CreationTime)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotification* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIUserNotification
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotification_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotification_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotification_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotification_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotification_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotification_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotification_get_Notification(This, value) \
    ((This)->lpVtbl->get_Notification(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotification_get_AppInfo(This, value) \
    ((This)->lpVtbl->get_AppInfo(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotification_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotification_get_CreationTime(This, value) \
    ((This)->lpVtbl->get_CreationTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIUserNotification;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIUserNotification_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.UI.Notifications.IUserNotificationChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.UI.Notifications.UserNotificationChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_UI_Notifications_IUserNotificationChangedEventArgs[] = L"Windows.UI.Notifications.IUserNotificationChangedEventArgs";
typedef struct __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChangeKind)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs* This,
        enum __x_ABI_CWindows_CUI_CNotifications_CUserNotificationChangedKind* value);
    HRESULT (STDMETHODCALLTYPE* get_UserNotificationId)(__x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgsVtbl;

interface __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_get_ChangeKind(This, value) \
    ((This)->lpVtbl->get_ChangeKind(This, value))

#define __x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_get_UserNotificationId(This, value) \
    ((This)->lpVtbl->get_UserNotificationId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CUI_CNotifications_CIUserNotificationChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.AdaptiveNotificationText
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IAdaptiveNotificationText ** Default Interface **
 *    Windows.UI.Notifications.IAdaptiveNotificationContent
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_AdaptiveNotificationText_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_AdaptiveNotificationText_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_AdaptiveNotificationText[] = L"Windows.UI.Notifications.AdaptiveNotificationText";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.BadgeNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Notifications.IBadgeNotificationFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IBadgeNotification ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_BadgeNotification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_BadgeNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_BadgeNotification[] = L"Windows.UI.Notifications.BadgeNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.BadgeUpdateManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.IBadgeUpdateManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Notifications.IBadgeUpdateManagerStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_BadgeUpdateManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_BadgeUpdateManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_BadgeUpdateManager[] = L"Windows.UI.Notifications.BadgeUpdateManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.BadgeUpdateManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IBadgeUpdateManagerForUser ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_BadgeUpdateManagerForUser_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_BadgeUpdateManagerForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_BadgeUpdateManagerForUser[] = L"Windows.UI.Notifications.BadgeUpdateManagerForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.BadgeUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IBadgeUpdater ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_BadgeUpdater_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_BadgeUpdater_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_BadgeUpdater[] = L"Windows.UI.Notifications.BadgeUpdater";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.KnownAdaptiveNotificationHints
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.IKnownAdaptiveNotificationHintsStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_KnownAdaptiveNotificationHints_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_KnownAdaptiveNotificationHints_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_KnownAdaptiveNotificationHints[] = L"Windows.UI.Notifications.KnownAdaptiveNotificationHints";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.KnownAdaptiveNotificationTextStyles
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.IKnownAdaptiveNotificationTextStylesStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_KnownAdaptiveNotificationTextStyles_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_KnownAdaptiveNotificationTextStyles_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_KnownAdaptiveNotificationTextStyles[] = L"Windows.UI.Notifications.KnownAdaptiveNotificationTextStyles";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.KnownNotificationBindings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.IKnownNotificationBindingsStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_KnownNotificationBindings_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_KnownNotificationBindings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_KnownNotificationBindings[] = L"Windows.UI.Notifications.KnownNotificationBindings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.Notification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.INotification ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_Notification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_Notification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_Notification[] = L"Windows.UI.Notifications.Notification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.NotificationBinding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.INotificationBinding ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_NotificationBinding_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_NotificationBinding_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_NotificationBinding[] = L"Windows.UI.Notifications.NotificationBinding";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.NotificationData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Notifications.INotificationDataFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.INotificationData ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_NotificationData_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_NotificationData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_NotificationData[] = L"Windows.UI.Notifications.NotificationData";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Notifications.NotificationVisual
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.INotificationVisual ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_NotificationVisual_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_NotificationVisual_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_NotificationVisual[] = L"Windows.UI.Notifications.NotificationVisual";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.ScheduledTileNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Notifications.IScheduledTileNotificationFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IScheduledTileNotification ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ScheduledTileNotification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ScheduledTileNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ScheduledTileNotification[] = L"Windows.UI.Notifications.ScheduledTileNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ScheduledToastNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Notifications.IScheduledToastNotificationFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IScheduledToastNotification ** Default Interface **
 *    Windows.UI.Notifications.IScheduledToastNotification2
 *    Windows.UI.Notifications.IScheduledToastNotification3
 *    Windows.UI.Notifications.IScheduledToastNotification4
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ScheduledToastNotification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ScheduledToastNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ScheduledToastNotification[] = L"Windows.UI.Notifications.ScheduledToastNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ScheduledToastNotificationShowingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IScheduledToastNotificationShowingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ScheduledToastNotificationShowingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ScheduledToastNotificationShowingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ScheduledToastNotificationShowingEventArgs[] = L"Windows.UI.Notifications.ScheduledToastNotificationShowingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.UI.Notifications.ShownTileNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IShownTileNotification ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ShownTileNotification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ShownTileNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ShownTileNotification[] = L"Windows.UI.Notifications.ShownTileNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.TileFlyoutNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Notifications.ITileFlyoutNotificationFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.ITileFlyoutNotification ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_TileFlyoutNotification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_TileFlyoutNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_TileFlyoutNotification[] = L"Windows.UI.Notifications.TileFlyoutNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.TileFlyoutUpdateManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.ITileFlyoutUpdateManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_TileFlyoutUpdateManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_TileFlyoutUpdateManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_TileFlyoutUpdateManager[] = L"Windows.UI.Notifications.TileFlyoutUpdateManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.TileFlyoutUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.ITileFlyoutUpdater ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_TileFlyoutUpdater_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_TileFlyoutUpdater_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_TileFlyoutUpdater[] = L"Windows.UI.Notifications.TileFlyoutUpdater";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.TileNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Notifications.ITileNotificationFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.ITileNotification ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_TileNotification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_TileNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_TileNotification[] = L"Windows.UI.Notifications.TileNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.TileUpdateManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.ITileUpdateManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Notifications.ITileUpdateManagerStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_TileUpdateManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_TileUpdateManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_TileUpdateManager[] = L"Windows.UI.Notifications.TileUpdateManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.TileUpdateManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.ITileUpdateManagerForUser ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_TileUpdateManagerForUser_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_TileUpdateManagerForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_TileUpdateManagerForUser[] = L"Windows.UI.Notifications.TileUpdateManagerForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.TileUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.ITileUpdater ** Default Interface **
 *    Windows.UI.Notifications.ITileUpdater2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_TileUpdater_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_TileUpdater_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_TileUpdater[] = L"Windows.UI.Notifications.TileUpdater";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastActivatedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastActivatedEventArgs ** Default Interface **
 *    Windows.UI.Notifications.IToastActivatedEventArgs2
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastActivatedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastActivatedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastActivatedEventArgs[] = L"Windows.UI.Notifications.ToastActivatedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastCollection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Notifications.IToastCollectionFactory interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastCollection ** Default Interface **
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastCollection_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastCollection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastCollection[] = L"Windows.UI.Notifications.ToastCollection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Notifications.ToastCollectionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastCollectionManager ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastCollectionManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastCollectionManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastCollectionManager[] = L"Windows.UI.Notifications.ToastCollectionManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.UI.Notifications.ToastDismissedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastDismissedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastDismissedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastDismissedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastDismissedEventArgs[] = L"Windows.UI.Notifications.ToastDismissedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastFailedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastFailedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastFailedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastFailedEventArgs[] = L"Windows.UI.Notifications.ToastFailedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.UI.Notifications.IToastNotificationFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastNotification ** Default Interface **
 *    Windows.UI.Notifications.IToastNotification2
 *    Windows.UI.Notifications.IToastNotification3
 *    Windows.UI.Notifications.IToastNotification4
 *    Windows.UI.Notifications.IToastNotification6
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastNotification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastNotification[] = L"Windows.UI.Notifications.ToastNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastNotificationActionTriggerDetail
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastNotificationActionTriggerDetail ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationActionTriggerDetail_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationActionTriggerDetail_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastNotificationActionTriggerDetail[] = L"Windows.UI.Notifications.ToastNotificationActionTriggerDetail";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastNotificationHistory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastNotificationHistory2
 *    Windows.UI.Notifications.IToastNotificationHistory ** Default Interface **
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationHistory_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationHistory_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastNotificationHistory[] = L"Windows.UI.Notifications.ToastNotificationHistory";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastNotificationHistoryChangedTriggerDetail ** Default Interface **
 *    Windows.UI.Notifications.IToastNotificationHistoryChangedTriggerDetail2
 *
 * Class Marshaling Behavior:  Standard - Class marshals using the standard marshaler
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationHistoryChangedTriggerDetail_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationHistoryChangedTriggerDetail_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastNotificationHistoryChangedTriggerDetail[] = L"Windows.UI.Notifications.ToastNotificationHistoryChangedTriggerDetail";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastNotificationManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.UI.Notifications.IToastNotificationManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Notifications.IToastNotificationManagerStatics4 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Notifications.IToastNotificationManagerStatics5 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.UI.Notifications.IToastNotificationManagerStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationManager_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastNotificationManager[] = L"Windows.UI.Notifications.ToastNotificationManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.ToastNotificationManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastNotificationManagerForUser ** Default Interface **
 *    Windows.UI.Notifications.IToastNotificationManagerForUser2
 *    Windows.UI.Notifications.IToastNotificationManagerForUser3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationManagerForUser_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastNotificationManagerForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastNotificationManagerForUser[] = L"Windows.UI.Notifications.ToastNotificationManagerForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.ToastNotifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IToastNotifier ** Default Interface **
 *    Windows.UI.Notifications.IToastNotifier2
 *    Windows.UI.Notifications.IToastNotifier3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_ToastNotifier_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_ToastNotifier_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_ToastNotifier[] = L"Windows.UI.Notifications.ToastNotifier";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.UI.Notifications.UserNotification
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IUserNotification ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_UserNotification_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_UserNotification_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_UserNotification[] = L"Windows.UI.Notifications.UserNotification";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.UI.Notifications.UserNotificationChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.UI.Notifications.IUserNotificationChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_UI_Notifications_UserNotificationChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_UI_Notifications_UserNotificationChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_UI_Notifications_UserNotificationChangedEventArgs[] = L"Windows.UI.Notifications.UserNotificationChangedEventArgs";
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
#endif // __windows2Eui2Enotifications_p_h__

#endif // __windows2Eui2Enotifications_h__
