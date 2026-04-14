
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
#ifndef __windows2Eapplicationmodel2Eappointments_h__
#define __windows2Eapplicationmodel2Eappointments_h__
#ifndef __windows2Eapplicationmodel2Eappointments_p_h__
#define __windows2Eapplicationmodel2Eappointments_p_h__


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
#include "Windows.System.h"
#include "Windows.UI.h"
#include "Windows.UI.Popups.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
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

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointment2;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2 ABI::Windows::ApplicationModel::Appointments::IAppointment2

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointment3;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3 ABI::Windows::ApplicationModel::Appointments::IAppointment3

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentCalendar;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendar

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentCalendar2;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2 ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendar2

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentCalendar3;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3 ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendar3

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentCalendarSyncManager;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendarSyncManager

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentCalendarSyncManager2;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2 ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendarSyncManager2

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentConflictResult;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult ABI::Windows::ApplicationModel::Appointments::IAppointmentConflictResult

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentException;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException ABI::Windows::ApplicationModel::Appointments::IAppointmentException

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentInvitee;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee ABI::Windows::ApplicationModel::Appointments::IAppointmentInvitee

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentManagerForUser;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser ABI::Windows::ApplicationModel::Appointments::IAppointmentManagerForUser

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentManagerStatics;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics ABI::Windows::ApplicationModel::Appointments::IAppointmentManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentManagerStatics2;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2 ABI::Windows::ApplicationModel::Appointments::IAppointmentManagerStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentManagerStatics3;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3 ABI::Windows::ApplicationModel::Appointments::IAppointmentManagerStatics3

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentParticipant;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant ABI::Windows::ApplicationModel::Appointments::IAppointmentParticipant

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentPropertiesStatics;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics ABI::Windows::ApplicationModel::Appointments::IAppointmentPropertiesStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentPropertiesStatics2;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2 ABI::Windows::ApplicationModel::Appointments::IAppointmentPropertiesStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentRecurrence;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence ABI::Windows::ApplicationModel::Appointments::IAppointmentRecurrence

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentRecurrence2;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2 ABI::Windows::ApplicationModel::Appointments::IAppointmentRecurrence2

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentRecurrence3;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3 ABI::Windows::ApplicationModel::Appointments::IAppointmentRecurrence3

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentStore;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore ABI::Windows::ApplicationModel::Appointments::IAppointmentStore

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentStore2;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2 ABI::Windows::ApplicationModel::Appointments::IAppointmentStore2

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentStore3;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3 ABI::Windows::ApplicationModel::Appointments::IAppointmentStore3

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentStoreChange;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreChange

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentStoreChange2;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2 ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreChange2

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentStoreChangeReader;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreChangeReader

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentStoreChangeTracker;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreChangeTracker

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentStoreChangeTracker2;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2 ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreChangeTracker2

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentStoreChangedDeferral;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreChangedDeferral

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentStoreChangedEventArgs;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreChangedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IAppointmentStoreNotificationTriggerDetails;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreNotificationTriggerDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                interface IFindAppointmentsOptions;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions ABI::Windows::ApplicationModel::Appointments::IFindAppointmentsOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncOperation_1_boolean_USE
#define DEF___FIAsyncOperation_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cdb5efb3-5788-509d-9be1-71ccb8a3362a"))
IAsyncOperation<bool> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<bool> __FIAsyncOperation_1_boolean_t;
#define __FIAsyncOperation_1_boolean ABI::Windows::Foundation::__FIAsyncOperation_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_boolean_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_boolean_USE
#define DEF___FIAsyncOperationCompletedHandler_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c1d3d1a2-ae17-5a5f-b5a2-bdcc8844889a"))
IAsyncOperationCompletedHandler<bool> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<bool> __FIAsyncOperationCompletedHandler_1_boolean_t;
#define __FIAsyncOperationCompletedHandler_1_boolean ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_boolean_USE */



#ifndef DEF___FIAsyncOperation_1_HSTRING_USE
#define DEF___FIAsyncOperation_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3e1fe603-f897-5263-b328-0806426b8a79"))
IAsyncOperation<HSTRING> : IAsyncOperation_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<HSTRING> __FIAsyncOperation_1_HSTRING_t;
#define __FIAsyncOperation_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b79a741f-7fb5-50ae-9e99-911201ec3d41"))
IAsyncOperationCompletedHandler<HSTRING> : IAsyncOperationCompletedHandler_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<HSTRING> __FIAsyncOperationCompletedHandler_1_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_HSTRING_USE */


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class Appointment;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0c5732f8-5bb9-5bb3-93e5-b87e43e0cd6a"))
IAsyncOperation<ABI::Windows::ApplicationModel::Appointments::Appointment*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::Appointment*, ABI::Windows::ApplicationModel::Appointments::IAppointment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Appointments.Appointment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Appointments::Appointment*> __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b640ed04-9331-5b28-9247-0146bcf5b72a"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Appointments::Appointment*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::Appointment*, ABI::Windows::ApplicationModel::Appointments::IAppointment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Appointments.Appointment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Appointments::Appointment*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class AppointmentCalendar;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6bb17a95-918e-5ad0-bbc2-bcc5fa1ff936"))
IAsyncOperation<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*, ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendar*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Appointments.AppointmentCalendar>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*> __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6d9cb651-5af6-51b0-9cd3-45dd51f17949"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*, ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendar*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Appointments.AppointmentCalendar>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class AppointmentConflictResult;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b376080f-e5b2-5ae2-9901-86cf77ba5d00"))
IAsyncOperation<ABI::Windows::ApplicationModel::Appointments::AppointmentConflictResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentConflictResult*, ABI::Windows::ApplicationModel::Appointments::IAppointmentConflictResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Appointments.AppointmentConflictResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Appointments::AppointmentConflictResult*> __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("82fb40fe-05b1-523c-9b53-b3dd759c9f75"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Appointments::AppointmentConflictResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentConflictResult*, ABI::Windows::ApplicationModel::Appointments::IAppointmentConflictResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Appointments.AppointmentConflictResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Appointments::AppointmentConflictResult*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class AppointmentStore;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("63798415-4d1f-5fc7-8729-79a282bceca4"))
IAsyncOperation<ABI::Windows::ApplicationModel::Appointments::AppointmentStore*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentStore*, ABI::Windows::ApplicationModel::Appointments::IAppointmentStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Appointments.AppointmentStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Appointments::AppointmentStore*> __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bccf6d2a-ab72-5f23-b7d5-4c2c9bd45b79"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Appointments::AppointmentStore*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentStore*, ABI::Windows::ApplicationModel::Appointments::IAppointmentStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Appointments.AppointmentStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Appointments::AppointmentStore*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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



#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_HSTRING_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2f92b529-119b-575a-a419-3904b4e41af2"))
IAsyncOperation<__FIVectorView_1_HSTRING*> : IAsyncOperation_impl<__FIVectorView_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_HSTRING*> __FIAsyncOperation_1___FIVectorView_1_HSTRING_t;
#define __FIAsyncOperation_1___FIVectorView_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_HSTRING_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7c7899be-5f2e-5bf3-ade5-ad98b772c7cd"))
IAsyncOperationCompletedHandler<__FIVectorView_1_HSTRING*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_HSTRING*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("386a5922-49fc-53b6-8bed-4c9ff9fe6e01"))
IIterator<ABI::Windows::ApplicationModel::Appointments::Appointment*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::Appointment*, ABI::Windows::ApplicationModel::Appointments::IAppointment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Appointments.Appointment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Appointments::Appointment*> __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_t;
#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b9802bba-ff53-5d37-8cd7-e56162f12156"))
IIterable<ABI::Windows::ApplicationModel::Appointments::Appointment*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::Appointment*, ABI::Windows::ApplicationModel::Appointments::IAppointment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Appointments.Appointment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Appointments::Appointment*> __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment_t;
#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("61021758-9e37-5a86-a832-aab31f32692b"))
IVectorView<ABI::Windows::ApplicationModel::Appointments::Appointment*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::Appointment*, ABI::Windows::ApplicationModel::Appointments::IAppointment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Appointments.Appointment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Appointments::Appointment*> __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_t;
#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a6bb6962-e2c1-5da2-9938-15ef82cbd1cc"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Appointments.Appointment>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f626345f-7bfc-5418-9f47-f1d86306e06b"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Appointments.Appointment>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f40a879d-b6b7-5f92-beb1-6a8e7ce54120"))
IIterator<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*, ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendar*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Appointments.AppointmentCalendar>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*> __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t;
#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d3fb010b-b692-5130-9d16-2cfdabcb6dec"))
IIterable<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*, ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendar*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Appointments.AppointmentCalendar>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*> __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t;
#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("774512d3-a564-5f8d-b104-3ec8f2a1104f"))
IVectorView<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*, ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendar*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Appointments.AppointmentCalendar>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendar*> __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t;
#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d26021ac-99b4-5c40-b0d2-d6835d2e9202"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Appointments.AppointmentCalendar>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f478469b-9777-553d-beab-1bb5eee1ca8b"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Appointments.AppointmentCalendar>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class AppointmentException;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e58c63db-d9d5-5fa5-8790-29846de54fa4"))
IIterator<ABI::Windows::ApplicationModel::Appointments::AppointmentException*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentException*, ABI::Windows::ApplicationModel::Appointments::IAppointmentException*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Appointments.AppointmentException>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Appointments::AppointmentException*> __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_t;
#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("caac41d6-6c65-5fd8-b783-eb9d9a4272b8"))
IIterable<ABI::Windows::ApplicationModel::Appointments::AppointmentException*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentException*, ABI::Windows::ApplicationModel::Appointments::IAppointmentException*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Appointments.AppointmentException>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Appointments::AppointmentException*> __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException_t;
#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e5030866-08d8-56be-a21b-c5bf80d70360"))
IVectorView<ABI::Windows::ApplicationModel::Appointments::AppointmentException*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentException*, ABI::Windows::ApplicationModel::Appointments::IAppointmentException*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Appointments.AppointmentException>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Appointments::AppointmentException*> __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_t;
#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("87e171ac-53fe-50ea-beb3-0589993ac984"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Appointments.AppointmentException>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("60e2023c-c2a9-5d3c-86b1-cd221c308a5e"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Appointments.AppointmentException>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class AppointmentStoreChange;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("16a427bf-e5b0-5662-9279-caa8ed8481a6"))
IIterator<ABI::Windows::ApplicationModel::Appointments::AppointmentStoreChange*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentStoreChange*, ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreChange*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Appointments.AppointmentStoreChange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Appointments::AppointmentStoreChange*> __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_t;
#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7b657ca2-e02e-5026-a032-9905e49682fd"))
IIterable<ABI::Windows::ApplicationModel::Appointments::AppointmentStoreChange*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentStoreChange*, ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreChange*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Appointments.AppointmentStoreChange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Appointments::AppointmentStoreChange*> __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_t;
#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4f9dd4e5-c3ae-5269-aff5-fe35cd50c3bb"))
IVectorView<ABI::Windows::ApplicationModel::Appointments::AppointmentStoreChange*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentStoreChange*, ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreChange*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Appointments.AppointmentStoreChange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Appointments::AppointmentStoreChange*> __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_t;
#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d54944b9-b72e-50bc-a64a-19396e0d1bcc"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Appointments.AppointmentStoreChange>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3fe84fc8-da80-5fab-8b46-3720f7646345"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Appointments.AppointmentStoreChange>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class AppointmentInvitee;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("57e35198-1b41-54da-adaf-50ef1e93fded"))
IIterator<ABI::Windows::ApplicationModel::Appointments::AppointmentInvitee*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentInvitee*, ABI::Windows::ApplicationModel::Appointments::IAppointmentInvitee*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Appointments.AppointmentInvitee>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Appointments::AppointmentInvitee*> __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_t;
#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fb9718a9-f059-52b0-a904-1a65e4281e40"))
IIterable<ABI::Windows::ApplicationModel::Appointments::AppointmentInvitee*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentInvitee*, ABI::Windows::ApplicationModel::Appointments::IAppointmentInvitee*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Appointments.AppointmentInvitee>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Appointments::AppointmentInvitee*> __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_t;
#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ba2f633a-5182-5eda-8e2e-a66e55b320ce"))
IVectorView<ABI::Windows::ApplicationModel::Appointments::AppointmentInvitee*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentInvitee*, ABI::Windows::ApplicationModel::Appointments::IAppointmentInvitee*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Appointments.AppointmentInvitee>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Appointments::AppointmentInvitee*> __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_t;
#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIVector_1_HSTRING_USE
#define DEF___FIVector_1_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("98b9acc1-4b56-532e-ac73-03d5291cca90"))
IVector<HSTRING> : IVector_impl<HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<HSTRING> __FIVector_1_HSTRING_t;
#define __FIVector_1_HSTRING ABI::Windows::Foundation::Collections::__FIVector_1_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_USE
#define DEF___FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("70f4b32e-f91e-55bb-9a92-0246da734bb0"))
IVector<ABI::Windows::ApplicationModel::Appointments::AppointmentInvitee*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentInvitee*, ABI::Windows::ApplicationModel::Appointments::IAppointmentInvitee*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.ApplicationModel.Appointments.AppointmentInvitee>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::ApplicationModel::Appointments::AppointmentInvitee*> __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_t;
#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIReference_1_UINT32_USE
#define DEF___FIReference_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("513ef3af-e784-5325-a91e-97c2b8111cf3"))
IReference<UINT32> : IReference_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<UINT32> __FIReference_1_UINT32_t;
#define __FIReference_1_UINT32 ABI::Windows::Foundation::__FIReference_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_UINT32_USE */


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
        namespace ApplicationModel {
            namespace Appointments {
                class AppointmentCalendarSyncManager;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bd1308de-6d2e-5541-b254-bdb61839bac1"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendarSyncManager*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendarSyncManager*, ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendarSyncManager*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Appointments::AppointmentCalendarSyncManager*, IInspectable*> __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class AppointmentStoreChangedEventArgs;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9e628351-c639-5cef-ab1d-8beae9d75d52"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Appointments::AppointmentStore*, ABI::Windows::ApplicationModel::Appointments::AppointmentStoreChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentStore*, ABI::Windows::ApplicationModel::Appointments::IAppointmentStore*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::AppointmentStoreChangedEventArgs*, ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Appointments.AppointmentStore, Windows.ApplicationModel.Appointments.AppointmentStoreChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Appointments::AppointmentStore*, ABI::Windows::ApplicationModel::Appointments::AppointmentStoreChangedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
            typedef struct Color Color;
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Popups {
                typedef enum Placement : int Placement;
            } /* Popups */
        } /* UI */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum AppointmentBusyStatus : int AppointmentBusyStatus;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum AppointmentCalendarOtherAppReadAccess : int AppointmentCalendarOtherAppReadAccess;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum AppointmentCalendarOtherAppWriteAccess : int AppointmentCalendarOtherAppWriteAccess;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum AppointmentCalendarSyncStatus : int AppointmentCalendarSyncStatus;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum AppointmentConflictType : int AppointmentConflictType;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum AppointmentDaysOfWeek : unsigned int AppointmentDaysOfWeek;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum AppointmentDetailsKind : int AppointmentDetailsKind;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum AppointmentParticipantResponse : int AppointmentParticipantResponse;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum AppointmentParticipantRole : int AppointmentParticipantRole;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum AppointmentRecurrenceUnit : int AppointmentRecurrenceUnit;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum AppointmentSensitivity : int AppointmentSensitivity;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum AppointmentStoreAccessType : int AppointmentStoreAccessType;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum AppointmentStoreChangeType : int AppointmentStoreChangeType;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum AppointmentSummaryCardView : int AppointmentSummaryCardView;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum AppointmentWeekOfMonth : int AppointmentWeekOfMonth;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum FindAppointmentCalendarsOptions : unsigned int FindAppointmentCalendarsOptions;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                typedef enum RecurrenceType : int RecurrenceType;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class AppointmentManagerForUser;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class AppointmentOrganizer;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class AppointmentRecurrence;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class AppointmentStoreChangeReader;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class AppointmentStoreChangeTracker;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class AppointmentStoreChangedDeferral;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                class FindAppointmentsOptions;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentBusyStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum AppointmentBusyStatus : int
                {
                    AppointmentBusyStatus_Busy = 0,
                    AppointmentBusyStatus_Tentative = 1,
                    AppointmentBusyStatus_Free = 2,
                    AppointmentBusyStatus_OutOfOffice = 3,
                    AppointmentBusyStatus_WorkingElsewhere = 4,
                };
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentCalendarOtherAppReadAccess
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum AppointmentCalendarOtherAppReadAccess : int
                {
                    AppointmentCalendarOtherAppReadAccess_SystemOnly = 0,
                    AppointmentCalendarOtherAppReadAccess_Limited = 1,
                    AppointmentCalendarOtherAppReadAccess_Full = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    AppointmentCalendarOtherAppReadAccess_None = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                };
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentCalendarOtherAppWriteAccess
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum AppointmentCalendarOtherAppWriteAccess : int
                {
                    AppointmentCalendarOtherAppWriteAccess_None = 0,
                    AppointmentCalendarOtherAppWriteAccess_SystemOnly = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    AppointmentCalendarOtherAppWriteAccess_Limited = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                };
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentCalendarSyncStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum AppointmentCalendarSyncStatus : int
                {
                    AppointmentCalendarSyncStatus_Idle = 0,
                    AppointmentCalendarSyncStatus_Syncing = 1,
                    AppointmentCalendarSyncStatus_UpToDate = 2,
                    AppointmentCalendarSyncStatus_AuthenticationError = 3,
                    AppointmentCalendarSyncStatus_PolicyError = 4,
                    AppointmentCalendarSyncStatus_UnknownError = 5,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
                    AppointmentCalendarSyncStatus_ManualAccountRemovalRequired = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
                };
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentConflictType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum AppointmentConflictType : int
                {
                    AppointmentConflictType_None = 0,
                    AppointmentConflictType_Adjacent = 1,
                    AppointmentConflictType_Overlap = 2,
                };
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentDaysOfWeek
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum AppointmentDaysOfWeek : unsigned int
                {
                    AppointmentDaysOfWeek_None = 0,
                    AppointmentDaysOfWeek_Sunday = 0x1,
                    AppointmentDaysOfWeek_Monday = 0x2,
                    AppointmentDaysOfWeek_Tuesday = 0x4,
                    AppointmentDaysOfWeek_Wednesday = 0x8,
                    AppointmentDaysOfWeek_Thursday = 0x10,
                    AppointmentDaysOfWeek_Friday = 0x20,
                    AppointmentDaysOfWeek_Saturday = 0x40,
                };

                DEFINE_ENUM_FLAG_OPERATORS(AppointmentDaysOfWeek)
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentDetailsKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum AppointmentDetailsKind : int
                {
                    AppointmentDetailsKind_PlainText = 0,
                    AppointmentDetailsKind_Html = 1,
                };
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentParticipantResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum AppointmentParticipantResponse : int
                {
                    AppointmentParticipantResponse_None = 0,
                    AppointmentParticipantResponse_Tentative = 1,
                    AppointmentParticipantResponse_Accepted = 2,
                    AppointmentParticipantResponse_Declined = 3,
                    AppointmentParticipantResponse_Unknown = 4,
                };
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentParticipantRole
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum AppointmentParticipantRole : int
                {
                    AppointmentParticipantRole_RequiredAttendee = 0,
                    AppointmentParticipantRole_OptionalAttendee = 1,
                    AppointmentParticipantRole_Resource = 2,
                };
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentRecurrenceUnit
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum AppointmentRecurrenceUnit : int
                {
                    AppointmentRecurrenceUnit_Daily = 0,
                    AppointmentRecurrenceUnit_Weekly = 1,
                    AppointmentRecurrenceUnit_Monthly = 2,
                    AppointmentRecurrenceUnit_MonthlyOnDay = 3,
                    AppointmentRecurrenceUnit_Yearly = 4,
                    AppointmentRecurrenceUnit_YearlyOnDay = 5,
                };
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentSensitivity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum AppointmentSensitivity : int
                {
                    AppointmentSensitivity_Public = 0,
                    AppointmentSensitivity_Private = 1,
                };
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentStoreAccessType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum AppointmentStoreAccessType : int
                {
                    AppointmentStoreAccessType_AppCalendarsReadWrite = 0,
                    AppointmentStoreAccessType_AllCalendarsReadOnly = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    AppointmentStoreAccessType_AllCalendarsReadWrite = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentStoreChangeType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum AppointmentStoreChangeType : int
                {
                    AppointmentStoreChangeType_AppointmentCreated = 0,
                    AppointmentStoreChangeType_AppointmentModified = 1,
                    AppointmentStoreChangeType_AppointmentDeleted = 2,
                    AppointmentStoreChangeType_ChangeTrackingLost = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    AppointmentStoreChangeType_CalendarCreated = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    AppointmentStoreChangeType_CalendarModified = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    AppointmentStoreChangeType_CalendarDeleted = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentSummaryCardView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum AppointmentSummaryCardView : int
                {
                    AppointmentSummaryCardView_System = 0,
                    AppointmentSummaryCardView_App = 1,
                };
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentWeekOfMonth
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum AppointmentWeekOfMonth : int
                {
                    AppointmentWeekOfMonth_First = 0,
                    AppointmentWeekOfMonth_Second = 1,
                    AppointmentWeekOfMonth_Third = 2,
                    AppointmentWeekOfMonth_Fourth = 3,
                    AppointmentWeekOfMonth_Last = 4,
                };
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.FindAppointmentCalendarsOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum FindAppointmentCalendarsOptions : unsigned int
                {
                    FindAppointmentCalendarsOptions_None = 0,
                    FindAppointmentCalendarsOptions_IncludeHidden = 0x1,
                };

                DEFINE_ENUM_FLAG_OPERATORS(FindAppointmentCalendarsOptions)
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.RecurrenceType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                enum RecurrenceType : int
                {
                    RecurrenceType_Master = 0,
                    RecurrenceType_Instance = 1,
                    RecurrenceType_ExceptionInstance = 2,
                };
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.Appointment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointment[] = L"Windows.ApplicationModel.Appointments.IAppointment";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("dd002f2f-2bdd-4076-90a3-22c275312965")
                IAppointment : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_StartTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_StartTime(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Duration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Duration(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Location(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Location(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Subject(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Subject(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Details(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Details(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Reminder(
                        __FIReference_1_Windows__CFoundation__CTimeSpan** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Reminder(
                        __FIReference_1_Windows__CFoundation__CTimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Organizer(
                        ABI::Windows::ApplicationModel::Appointments::IAppointmentParticipant** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Organizer(
                        ABI::Windows::ApplicationModel::Appointments::IAppointmentParticipant* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Invitees(
                        __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Recurrence(
                        ABI::Windows::ApplicationModel::Appointments::IAppointmentRecurrence** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Recurrence(
                        ABI::Windows::ApplicationModel::Appointments::IAppointmentRecurrence* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BusyStatus(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentBusyStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BusyStatus(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentBusyStatus value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllDay(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AllDay(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Sensitivity(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentSensitivity* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Sensitivity(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentSensitivity value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Uri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointment = __uuidof(IAppointment);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointment2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.Appointment
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointment2[] = L"Windows.ApplicationModel.Appointments.IAppointment2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("5e85983c-540f-3452-9b5c-0dd7ad4c65a2")
                IAppointment2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LocalId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CalendarId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RoamingId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RoamingId(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OriginalStartTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsResponseRequested(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsResponseRequested(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllowNewTimeProposal(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AllowNewTimeProposal(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OnlineMeetingLink(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OnlineMeetingLink(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReplyTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ReplyTime(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserResponse(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentParticipantResponse* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_UserResponse(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentParticipantResponse value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasInvitees(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsCanceledMeeting(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsCanceledMeeting(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsOrganizedByUser(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsOrganizedByUser(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointment2 = __uuidof(IAppointment2);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointment3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.Appointment
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointment2
 *     Windows.ApplicationModel.Appointments.IAppointment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointment3[] = L"Windows.ApplicationModel.Appointments.IAppointment3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("bfcc45a9-8961-4991-934b-c48768e5a96c")
                IAppointment3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ChangeNumber(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteChangeNumber(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RemoteChangeNumber(
                        UINT64 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DetailsKind(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentDetailsKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DetailsKind(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentDetailsKind value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointment3 = __uuidof(IAppointment3);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentCalendar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentCalendar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentCalendar[] = L"Windows.ApplicationModel.Appointments.IAppointmentCalendar";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("5273819d-8339-3d4f-a02f-64084452bb5d")
                IAppointmentCalendar : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocalId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsHidden(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OtherAppReadAccess(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentCalendarOtherAppReadAccess* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OtherAppReadAccess(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentCalendarOtherAppReadAccess value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OtherAppWriteAccess(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentCalendarOtherAppWriteAccess* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OtherAppWriteAccess(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentCalendarOtherAppWriteAccess value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SourceDisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SummaryCardView(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentSummaryCardView* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SummaryCardView(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentSummaryCardView value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAppointmentsAsync(
                        ABI::Windows::Foundation::DateTime rangeStart,
                        ABI::Windows::Foundation::TimeSpan rangeLength,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAppointmentsAsyncWithOptions(
                        ABI::Windows::Foundation::DateTime rangeStart,
                        ABI::Windows::Foundation::TimeSpan rangeLength,
                        ABI::Windows::ApplicationModel::Appointments::IFindAppointmentsOptions* options,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindExceptionsFromMasterAsync(
                        HSTRING masterLocalId,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllInstancesAsync(
                        HSTRING masterLocalId,
                        ABI::Windows::Foundation::DateTime rangeStart,
                        ABI::Windows::Foundation::TimeSpan rangeLength,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAllInstancesAsyncWithOptions(
                        HSTRING masterLocalId,
                        ABI::Windows::Foundation::DateTime rangeStart,
                        ABI::Windows::Foundation::TimeSpan rangeLength,
                        ABI::Windows::ApplicationModel::Appointments::IFindAppointmentsOptions* pOptions,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAppointmentAsync(
                        HSTRING localId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAppointmentInstanceAsync(
                        HSTRING localId,
                        ABI::Windows::Foundation::DateTime instanceStartTime,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindUnexpandedAppointmentsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindUnexpandedAppointmentsAsyncWithOptions(
                        ABI::Windows::ApplicationModel::Appointments::IFindAppointmentsOptions* options,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteAsync(
                        ABI::Windows::Foundation::IAsyncAction** asyncAction
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SaveAsync(
                        ABI::Windows::Foundation::IAsyncAction** asyncAction
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteAppointmentAsync(
                        HSTRING localId,
                        ABI::Windows::Foundation::IAsyncAction** asyncAction
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteAppointmentInstanceAsync(
                        HSTRING localId,
                        ABI::Windows::Foundation::DateTime instanceStartTime,
                        ABI::Windows::Foundation::IAsyncAction** asyncAction
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SaveAppointmentAsync(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* pAppointment,
                        ABI::Windows::Foundation::IAsyncAction** asyncAction
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentCalendar = __uuidof(IAppointmentCalendar);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentCalendar2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentCalendar
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointmentCalendar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentCalendar2[] = L"Windows.ApplicationModel.Appointments.IAppointmentCalendar2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("18e7e422-2467-4e1c-a459-d8a29303d092")
                IAppointmentCalendar2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SyncManager(
                        ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendarSyncManager** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RemoteId(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayColor(
                        ABI::Windows::UI::Color value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsHidden(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserDataAccountId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanCreateOrUpdateAppointments(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CanCreateOrUpdateAppointments(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanCancelMeetings(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CanCancelMeetings(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanForwardMeetings(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CanForwardMeetings(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanProposeNewTimeForMeetings(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CanProposeNewTimeForMeetings(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanUpdateMeetingResponses(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CanUpdateMeetingResponses(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanNotifyInvitees(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CanNotifyInvitees(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MustNofityInvitees(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MustNofityInvitees(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryCreateOrUpdateAppointmentAsync(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        boolean notifyInvitees,
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryCancelMeetingAsync(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* meeting,
                        HSTRING subject,
                        HSTRING comment,
                        boolean notifyInvitees,
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryForwardMeetingAsync(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* meeting,
                        __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* invitees,
                        HSTRING subject,
                        HSTRING forwardHeader,
                        HSTRING comment,
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryProposeNewTimeForMeetingAsync(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* meeting,
                        ABI::Windows::Foundation::DateTime newStartTime,
                        ABI::Windows::Foundation::TimeSpan newDuration,
                        HSTRING subject,
                        HSTRING comment,
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryUpdateMeetingResponseAsync(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* meeting,
                        ABI::Windows::ApplicationModel::Appointments::AppointmentParticipantResponse response,
                        HSTRING subject,
                        HSTRING comment,
                        boolean sendUpdate,
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentCalendar2 = __uuidof(IAppointmentCalendar2);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentCalendar3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentCalendar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentCalendar3[] = L"Windows.ApplicationModel.Appointments.IAppointmentCalendar3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("eb23d22b-a685-42ae-8495-b3119adb4167")
                IAppointmentCalendar3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RegisterSyncManagerAsync(
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentCalendar3 = __uuidof(IAppointmentCalendar3);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentCalendarSyncManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentCalendarSyncManager[] = L"Windows.ApplicationModel.Appointments.IAppointmentCalendarSyncManager";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("2b21b3a0-4aff-4392-bc5f-5645ffcffb17")
                IAppointmentCalendarSyncManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentCalendarSyncStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LastSuccessfulSyncTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LastAttemptedSyncTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SyncAsync(
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_SyncStatusChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SyncStatusChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentCalendarSyncManager = __uuidof(IAppointmentCalendarSyncManager);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentCalendarSyncManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentCalendarSyncManager2[] = L"Windows.ApplicationModel.Appointments.IAppointmentCalendarSyncManager2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("647528ad-0d29-4c7c-aaa7-bf996805537c")
                IAppointmentCalendarSyncManager2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Status(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentCalendarSyncStatus value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LastSuccessfulSyncTime(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LastAttemptedSyncTime(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentCalendarSyncManager2 = __uuidof(IAppointmentCalendarSyncManager2);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentConflictResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentConflictResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentConflictResult[] = L"Windows.ApplicationModel.Appointments.IAppointmentConflictResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("d5cdf0be-2f2f-3b7d-af0a-a7e20f3a46e3")
                IAppointmentConflictResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Type(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentConflictType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Date(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentConflictResult = __uuidof(IAppointmentConflictResult);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentException
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentException
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentException[] = L"Windows.ApplicationModel.Appointments.IAppointmentException";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("a2076767-16f6-4bce-9f5a-8600b8019fcb")
                IAppointmentException : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Appointment(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExceptionProperties(
                        __FIVectorView_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsDeleted(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentException = __uuidof(IAppointmentException);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentInvitee
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentInvitee
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointmentParticipant
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentInvitee[] = L"Windows.ApplicationModel.Appointments.IAppointmentInvitee";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("13bf0796-9842-495b-b0e7-ef8f79c0701d")
                IAppointmentInvitee : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Role(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentParticipantRole* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Role(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentParticipantRole value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Response(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentParticipantResponse* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Response(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentParticipantResponse value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentInvitee = __uuidof(IAppointmentInvitee);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentManagerForUser[] = L"Windows.ApplicationModel.Appointments.IAppointmentManagerForUser";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("70261423-73cc-4660-b318-b01365302a03")
                IAppointmentManagerForUser : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ShowAddAppointmentAsync(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        ABI::Windows::Foundation::Rect selection,
                        __FIAsyncOperation_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAddAppointmentWithPlacementAsync(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement,
                        __FIAsyncOperation_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowReplaceAppointmentAsync(
                        HSTRING appointmentId,
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        ABI::Windows::Foundation::Rect selection,
                        __FIAsyncOperation_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowReplaceAppointmentWithPlacementAsync(
                        HSTRING appointmentId,
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement,
                        __FIAsyncOperation_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowReplaceAppointmentWithPlacementAndDateAsync(
                        HSTRING appointmentId,
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement,
                        ABI::Windows::Foundation::DateTime instanceStartDate,
                        __FIAsyncOperation_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowRemoveAppointmentAsync(
                        HSTRING appointmentId,
                        ABI::Windows::Foundation::Rect selection,
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowRemoveAppointmentWithPlacementAsync(
                        HSTRING appointmentId,
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement,
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowRemoveAppointmentWithPlacementAndDateAsync(
                        HSTRING appointmentId,
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement,
                        ABI::Windows::Foundation::DateTime instanceStartDate,
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowTimeFrameAsync(
                        ABI::Windows::Foundation::DateTime timeToShow,
                        ABI::Windows::Foundation::TimeSpan duration,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAppointmentDetailsAsync(
                        HSTRING appointmentId,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAppointmentDetailsWithDateAsync(
                        HSTRING appointmentId,
                        ABI::Windows::Foundation::DateTime instanceStartDate,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowEditNewAppointmentAsync(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        __FIAsyncOperation_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestStoreAsync(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentStoreAccessType options,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentManagerForUser = __uuidof(IAppointmentManagerForUser);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentManagerStatics[] = L"Windows.ApplicationModel.Appointments.IAppointmentManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("3a30fa01-5c40-499d-b33f-a43050f74fc4")
                IAppointmentManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ShowAddAppointmentAsync(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        ABI::Windows::Foundation::Rect selection,
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAddAppointmentWithPlacementAsync(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement,
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowReplaceAppointmentAsync(
                        HSTRING appointmentId,
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        ABI::Windows::Foundation::Rect selection,
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowReplaceAppointmentWithPlacementAsync(
                        HSTRING appointmentId,
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement,
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowReplaceAppointmentWithPlacementAndDateAsync(
                        HSTRING appointmentId,
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement,
                        ABI::Windows::Foundation::DateTime instanceStartDate,
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowRemoveAppointmentAsync(
                        HSTRING appointmentId,
                        ABI::Windows::Foundation::Rect selection,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowRemoveAppointmentWithPlacementAsync(
                        HSTRING appointmentId,
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowRemoveAppointmentWithPlacementAndDateAsync(
                        HSTRING appointmentId,
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement,
                        ABI::Windows::Foundation::DateTime instanceStartDate,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowTimeFrameAsync(
                        ABI::Windows::Foundation::DateTime timeToShow,
                        ABI::Windows::Foundation::TimeSpan duration,
                        ABI::Windows::Foundation::IAsyncAction** asyncAction
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentManagerStatics = __uuidof(IAppointmentManagerStatics);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentManagerStatics2[] = L"Windows.ApplicationModel.Appointments.IAppointmentManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("0a81f60d-d04f-4034-af72-a36573b45ff0")
                IAppointmentManagerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ShowAppointmentDetailsAsync(
                        HSTRING appointmentId,
                        ABI::Windows::Foundation::IAsyncAction** asyncAction
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAppointmentDetailsWithDateAsync(
                        HSTRING appointmentId,
                        ABI::Windows::Foundation::DateTime instanceStartDate,
                        ABI::Windows::Foundation::IAsyncAction** asyncAction
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowEditNewAppointmentAsync(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestStoreAsync(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentStoreAccessType options,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentManagerStatics2 = __uuidof(IAppointmentManagerStatics2);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentManagerStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentManagerStatics3[] = L"Windows.ApplicationModel.Appointments.IAppointmentManagerStatics3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("2f9ae09c-b34c-4dc7-a35d-cafd88ae3ec6")
                IAppointmentManagerStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::ApplicationModel::Appointments::IAppointmentManagerForUser** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentManagerStatics3 = __uuidof(IAppointmentManagerStatics3);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentParticipant
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentParticipant[] = L"Windows.ApplicationModel.Appointments.IAppointmentParticipant";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("615e2902-9718-467b-83fb-b293a19121de")
                IAppointmentParticipant : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Address(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Address(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentParticipant = __uuidof(IAppointmentParticipant);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentPropertiesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentPropertiesStatics[] = L"Windows.ApplicationModel.Appointments.IAppointmentPropertiesStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("25141fe9-68ae-3aae-855f-bc4441caa234")
                IAppointmentPropertiesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Subject(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Location(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StartTime(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Duration(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Reminder(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BusyStatus(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Sensitivity(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OriginalStartTime(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsResponseRequested(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllowNewTimeProposal(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllDay(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Details(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OnlineMeetingLink(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReplyTime(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Organizer(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UserResponse(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HasInvitees(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsCanceledMeeting(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsOrganizedByUser(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Recurrence(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Uri(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Invitees(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DefaultProperties(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentPropertiesStatics = __uuidof(IAppointmentPropertiesStatics);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentPropertiesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointmentPropertiesStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentPropertiesStatics2[] = L"Windows.ApplicationModel.Appointments.IAppointmentPropertiesStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("dffc434b-b017-45dd-8af5-d163d10801bb")
                IAppointmentPropertiesStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ChangeNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteChangeNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DetailsKind(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentPropertiesStatics2 = __uuidof(IAppointmentPropertiesStatics2);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentRecurrence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentRecurrence
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentRecurrence[] = L"Windows.ApplicationModel.Appointments.IAppointmentRecurrence";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("d87b3e83-15a6-487b-b959-0c361e60e954")
                IAppointmentRecurrence : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Unit(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentRecurrenceUnit* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Unit(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentRecurrenceUnit value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Occurrences(
                        __FIReference_1_UINT32** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Occurrences(
                        __FIReference_1_UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Until(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Until(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Interval(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Interval(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DaysOfWeek(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentDaysOfWeek* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DaysOfWeek(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentDaysOfWeek value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WeekOfMonth(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentWeekOfMonth* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_WeekOfMonth(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentWeekOfMonth value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Month(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Month(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Day(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Day(
                        UINT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentRecurrence = __uuidof(IAppointmentRecurrence);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentRecurrence2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentRecurrence
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointmentRecurrence
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentRecurrence2[] = L"Windows.ApplicationModel.Appointments.IAppointmentRecurrence2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("3df3a2e0-05a7-4f50-9f86-b03f9436254d")
                IAppointmentRecurrence2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RecurrenceType(
                        ABI::Windows::ApplicationModel::Appointments::RecurrenceType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TimeZone(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TimeZone(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentRecurrence2 = __uuidof(IAppointmentRecurrence2);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentRecurrence3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentRecurrence
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointmentRecurrence2
 *     Windows.ApplicationModel.Appointments.IAppointmentRecurrence
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentRecurrence3[] = L"Windows.ApplicationModel.Appointments.IAppointmentRecurrence3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("89ff96d9-da4d-4a17-8dd2-1cebc2b5ff9d")
                IAppointmentRecurrence3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CalendarIdentifier(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentRecurrence3 = __uuidof(IAppointmentRecurrence3);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStore[] = L"Windows.ApplicationModel.Appointments.IAppointmentStore";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("a461918c-7a47-4d96-96c9-15cd8a05a735")
                IAppointmentStore : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ChangeTracker(
                        ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreChangeTracker** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAppointmentCalendarAsync(
                        HSTRING name,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAppointmentCalendarAsync(
                        HSTRING calendarId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAppointmentAsync(
                        HSTRING localId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAppointmentInstanceAsync(
                        HSTRING localId,
                        ABI::Windows::Foundation::DateTime instanceStartTime,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAppointmentCalendarsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAppointmentCalendarsAsyncWithOptions(
                        ABI::Windows::ApplicationModel::Appointments::FindAppointmentCalendarsOptions options,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAppointmentsAsync(
                        ABI::Windows::Foundation::DateTime rangeStart,
                        ABI::Windows::Foundation::TimeSpan rangeLength,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindAppointmentsAsyncWithOptions(
                        ABI::Windows::Foundation::DateTime rangeStart,
                        ABI::Windows::Foundation::TimeSpan rangeLength,
                        ABI::Windows::ApplicationModel::Appointments::IFindAppointmentsOptions* options,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindConflictAsync(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindConflictAsyncWithInstanceStart(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        ABI::Windows::Foundation::DateTime instanceStartTime,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE MoveAppointmentAsync(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendar* destinationCalendar,
                        ABI::Windows::Foundation::IAsyncAction** asyncAction
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAddAppointmentAsync(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        ABI::Windows::Foundation::Rect selection,
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowReplaceAppointmentAsync(
                        HSTRING localId,
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        ABI::Windows::Foundation::Rect selection,
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowReplaceAppointmentWithPlacementAndDateAsync(
                        HSTRING localId,
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement,
                        ABI::Windows::Foundation::DateTime instanceStartDate,
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowRemoveAppointmentAsync(
                        HSTRING localId,
                        ABI::Windows::Foundation::Rect selection,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowRemoveAppointmentWithPlacementAndDateAsync(
                        HSTRING localId,
                        ABI::Windows::Foundation::Rect selection,
                        ABI::Windows::UI::Popups::Placement preferredPlacement,
                        ABI::Windows::Foundation::DateTime instanceStartDate,
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAppointmentDetailsAsync(
                        HSTRING localId,
                        ABI::Windows::Foundation::IAsyncAction** asyncAction
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowAppointmentDetailsWithDateAsync(
                        HSTRING localId,
                        ABI::Windows::Foundation::DateTime instanceStartDate,
                        ABI::Windows::Foundation::IAsyncAction** asyncAction
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowEditNewAppointmentAsync(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment* appointment,
                        __FIAsyncOperation_1_HSTRING** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FindLocalIdsFromRoamingIdAsync(
                        HSTRING roamingId,
                        __FIAsyncOperation_1___FIVectorView_1_HSTRING** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentStore = __uuidof(IAppointmentStore);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStore2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStore
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointmentStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStore2[] = L"Windows.ApplicationModel.Appointments.IAppointmentStore2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("25c48c20-1c41-424f-8084-67c1cfe0a854")
                IAppointmentStore2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_StoreChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs* pHandler,
                        EventRegistrationToken* pToken
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StoreChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAppointmentCalendarInAccountAsync(
                        HSTRING name,
                        HSTRING userDataAccountId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentStore2 = __uuidof(IAppointmentStore2);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStore3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStore3[] = L"Windows.ApplicationModel.Appointments.IAppointmentStore3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("4251940b-b078-470a-9a40-c2e01761f72f")
                IAppointmentStore3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetChangeTracker(
                        HSTRING identity,
                        ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreChangeTracker** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentStore3 = __uuidof(IAppointmentStore3);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStoreChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStoreChange
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStoreChange[] = L"Windows.ApplicationModel.Appointments.IAppointmentStoreChange";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("a5a6e035-0a33-3654-8463-b543e90c3b79")
                IAppointmentStoreChange : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Appointment(
                        ABI::Windows::ApplicationModel::Appointments::IAppointment** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ChangeType(
                        ABI::Windows::ApplicationModel::Appointments::AppointmentStoreChangeType* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentStoreChange = __uuidof(IAppointmentStoreChange);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStoreChange2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStoreChange
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointmentStoreChange
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStoreChange2[] = L"Windows.ApplicationModel.Appointments.IAppointmentStoreChange2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("b37d0dce-5211-4402-a608-a96fe70b8ee2")
                IAppointmentStoreChange2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AppointmentCalendar(
                        ABI::Windows::ApplicationModel::Appointments::IAppointmentCalendar** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentStoreChange2 = __uuidof(IAppointmentStoreChange2);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStoreChangeReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStoreChangeReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStoreChangeReader[] = L"Windows.ApplicationModel.Appointments.IAppointmentStoreChangeReader";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("8b2409f1-65f3-42a0-961d-4c209bf30370")
                IAppointmentStoreChangeReader : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReadBatchAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AcceptChanges(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AcceptChangesThrough(
                        ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreChange* lastChangeToAccept
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentStoreChangeReader = __uuidof(IAppointmentStoreChangeReader);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStoreChangeTracker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStoreChangeTracker[] = L"Windows.ApplicationModel.Appointments.IAppointmentStoreChangeTracker";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("1b25f4b1-8ece-4f17-93c8-e6412458fd5c")
                IAppointmentStoreChangeTracker : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetChangeReader(
                        ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreChangeReader** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Enable(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Reset(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentStoreChangeTracker = __uuidof(IAppointmentStoreChangeTracker);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStoreChangeTracker2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStoreChangeTracker2[] = L"Windows.ApplicationModel.Appointments.IAppointmentStoreChangeTracker2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("b66aaf45-9542-4cf7-8550-eb370e0c08d3")
                IAppointmentStoreChangeTracker2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsTracking(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentStoreChangeTracker2 = __uuidof(IAppointmentStoreChangeTracker2);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStoreChangedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStoreChangedDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStoreChangedDeferral[] = L"Windows.ApplicationModel.Appointments.IAppointmentStoreChangedDeferral";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("4cb82026-fedb-4bc3-9662-95a9befdf4df")
                IAppointmentStoreChangedDeferral : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentStoreChangedDeferral = __uuidof(IAppointmentStoreChangedDeferral);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStoreChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStoreChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStoreChangedEventArgs[] = L"Windows.ApplicationModel.Appointments.IAppointmentStoreChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("2285f8b9-0791-417e-bfea-cc6d41636c8c")
                IAppointmentStoreChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::ApplicationModel::Appointments::IAppointmentStoreChangedDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppointmentStoreChangedEventArgs = __uuidof(IAppointmentStoreChangedEventArgs);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStoreNotificationTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStoreNotificationTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStoreNotificationTriggerDetails[] = L"Windows.ApplicationModel.Appointments.IAppointmentStoreNotificationTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("9b33cb11-c301-421e-afef-047ecfa76adb")
                IAppointmentStoreNotificationTriggerDetails : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IAppointmentStoreNotificationTriggerDetails = __uuidof(IAppointmentStoreNotificationTriggerDetails);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IFindAppointmentsOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.FindAppointmentsOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IFindAppointmentsOptions[] = L"Windows.ApplicationModel.Appointments.IFindAppointmentsOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                MIDL_INTERFACE("55f7dc55-9942-3086-82b5-2cb29f64d5f5")
                IFindAppointmentsOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CalendarIds(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FetchProperties(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IncludeHidden(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IncludeHidden(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MaxCount(
                        UINT32 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFindAppointmentsOptions = __uuidof(IFindAppointmentsOptions);
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.Appointment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointment ** Default Interface **
 *    Windows.ApplicationModel.Appointments.IAppointment2
 *    Windows.ApplicationModel.Appointments.IAppointment3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_Appointment_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_Appointment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_Appointment[] = L"Windows.ApplicationModel.Appointments.Appointment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentCalendar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentCalendar ** Default Interface **
 *    Windows.ApplicationModel.Appointments.IAppointmentCalendar2
 *    Windows.ApplicationModel.Appointments.IAppointmentCalendar3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentCalendar_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentCalendar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentCalendar[] = L"Windows.ApplicationModel.Appointments.AppointmentCalendar";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentCalendarSyncManager ** Default Interface **
 *    Windows.ApplicationModel.Appointments.IAppointmentCalendarSyncManager2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentCalendarSyncManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentCalendarSyncManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentCalendarSyncManager[] = L"Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentConflictResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentConflictResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentConflictResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentConflictResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentConflictResult[] = L"Windows.ApplicationModel.Appointments.AppointmentConflictResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentException
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentException ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentException_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentException_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentException[] = L"Windows.ApplicationModel.Appointments.AppointmentException";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentInvitee
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentInvitee ** Default Interface **
 *    Windows.ApplicationModel.Appointments.IAppointmentParticipant
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentInvitee_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentInvitee_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentInvitee[] = L"Windows.ApplicationModel.Appointments.AppointmentInvitee";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Appointments.IAppointmentManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Appointments.IAppointmentManagerStatics3 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Appointments.IAppointmentManagerStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentManager[] = L"Windows.ApplicationModel.Appointments.AppointmentManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentManagerForUser ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentManagerForUser_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentManagerForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentManagerForUser[] = L"Windows.ApplicationModel.Appointments.AppointmentManagerForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentOrganizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentParticipant ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentOrganizer_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentOrganizer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentOrganizer[] = L"Windows.ApplicationModel.Appointments.AppointmentOrganizer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Appointments.IAppointmentPropertiesStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Appointments.IAppointmentPropertiesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentProperties_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentProperties[] = L"Windows.ApplicationModel.Appointments.AppointmentProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentRecurrence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentRecurrence ** Default Interface **
 *    Windows.ApplicationModel.Appointments.IAppointmentRecurrence2
 *    Windows.ApplicationModel.Appointments.IAppointmentRecurrence3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentRecurrence_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentRecurrence_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentRecurrence[] = L"Windows.ApplicationModel.Appointments.AppointmentRecurrence";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentStore ** Default Interface **
 *    Windows.ApplicationModel.Appointments.IAppointmentStore2
 *    Windows.ApplicationModel.Appointments.IAppointmentStore3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStore_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentStore[] = L"Windows.ApplicationModel.Appointments.AppointmentStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentStoreChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentStoreChange ** Default Interface **
 *    Windows.ApplicationModel.Appointments.IAppointmentStoreChange2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChange_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChange_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentStoreChange[] = L"Windows.ApplicationModel.Appointments.AppointmentStoreChange";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentStoreChangeReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentStoreChangeReader ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChangeReader_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChangeReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentStoreChangeReader[] = L"Windows.ApplicationModel.Appointments.AppointmentStoreChangeReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentStoreChangeTracker ** Default Interface **
 *    Windows.ApplicationModel.Appointments.IAppointmentStoreChangeTracker2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChangeTracker_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChangeTracker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentStoreChangeTracker[] = L"Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentStoreChangedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentStoreChangedDeferral ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChangedDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChangedDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentStoreChangedDeferral[] = L"Windows.ApplicationModel.Appointments.AppointmentStoreChangedDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentStoreChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentStoreChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentStoreChangedEventArgs[] = L"Windows.ApplicationModel.Appointments.AppointmentStoreChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentStoreNotificationTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentStoreNotificationTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreNotificationTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreNotificationTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentStoreNotificationTriggerDetails[] = L"Windows.ApplicationModel.Appointments.AppointmentStoreNotificationTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.FindAppointmentsOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IFindAppointmentsOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_FindAppointmentsOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_FindAppointmentsOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_FindAppointmentsOptions[] = L"Windows.ApplicationModel.Appointments.FindAppointmentsOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2 __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3 __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2 __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3 __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2 __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2 __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3 __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2 __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2 __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3 __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2 __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3 __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2 __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2 __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_boolean __FIAsyncOperationCompletedHandler_1_boolean;

#if !defined(____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_boolean __FIAsyncOperation_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_boolean;

typedef struct __FIAsyncOperation_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_boolean* This,
        __FIAsyncOperationCompletedHandler_1_boolean* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_boolean* This,
        __FIAsyncOperationCompletedHandler_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_boolean* This,
        boolean* result);

    END_INTERFACE
} __FIAsyncOperation_1_booleanVtbl;

interface __FIAsyncOperation_1_boolean
{
    CONST_VTBL struct __FIAsyncOperation_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_boolean_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_boolean_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_boolean_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_boolean __FIAsyncOperationCompletedHandler_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_boolean;

typedef struct __FIAsyncOperationCompletedHandler_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_boolean* This,
        __FIAsyncOperation_1_boolean* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_booleanVtbl;

interface __FIAsyncOperationCompletedHandler_1_boolean
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_boolean_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

#if !defined(____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_HSTRING __FIAsyncOperation_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_HSTRING;

typedef struct __FIAsyncOperation_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIAsyncOperation_1_HSTRINGVtbl;

interface __FIAsyncOperation_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_HSTRING __FIAsyncOperationCompletedHandler_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_HSTRING* This,
        __FIAsyncOperation_1_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CAppointments__CAppointmentStore_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING;

#if !defined(____FIAsyncOperation_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_HSTRING __FIAsyncOperation_1___FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_HSTRING;

typedef struct __FIAsyncOperation_1___FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_HSTRING* This,
        __FIVectorView_1_HSTRING** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_HSTRINGVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_HSTRING_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING* This,
        __FIAsyncOperation_1___FIVectorView_1_HSTRING* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRINGVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment;

typedef struct __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment;

typedef struct __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointment** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

typedef struct __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

typedef struct __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException;

typedef struct __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentExceptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentExceptionVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentExceptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException;

typedef struct __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentExceptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentException** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentExceptionVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentExceptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentException_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentExceptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentExceptionVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentExceptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentExceptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentExceptionVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentExceptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentExceptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentExceptionVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentExceptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange;

typedef struct __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangeVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange;

typedef struct __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangeVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangeVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangeVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangeVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee;

typedef struct __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInviteeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInviteeVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInviteeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee;

typedef struct __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInviteeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        __FIIterator_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInviteeVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInviteeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInviteeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInviteeVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInviteeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIVector_1_HSTRING_INTERFACE_DEFINED__)
#define ____FIVector_1_HSTRING_INTERFACE_DEFINED__

typedef interface __FIVector_1_HSTRING __FIVector_1_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_HSTRING;

typedef struct __FIVector_1_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_HSTRING* This,
        __FIVectorView_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_HSTRING* This,
        HSTRING value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_HSTRING* This,
        UINT32 index,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_HSTRING* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_HSTRING* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_HSTRING* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        HSTRING* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_HSTRING* This,
        UINT32 itemsLength,
        HSTRING* items);

    END_INTERFACE
} __FIVector_1_HSTRINGVtbl;

interface __FIVector_1_HSTRING
{
    CONST_VTBL struct __FIVector_1_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_HSTRING_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_HSTRING_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_HSTRING_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_HSTRING_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_HSTRING_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_HSTRING_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_HSTRING_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_HSTRING_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_HSTRING_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_HSTRING_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_HSTRING_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee;

typedef struct __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInviteeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee** items);

    END_INTERFACE
} __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInviteeVtbl;

interface __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee
{
    CONST_VTBL struct __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInviteeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIReference_1_UINT32_INTERFACE_DEFINED__)
#define ____FIReference_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIReference_1_UINT32 __FIReference_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_UINT32;

typedef struct __FIReference_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_UINT32* This,
        UINT32* result);

    END_INTERFACE
} __FIReference_1_UINT32Vtbl;

interface __FIReference_1_UINT32
{
    CONST_VTBL struct __FIReference_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_UINT32_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_UINT32_INTERFACE_DEFINED__

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
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* sender,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CRect __x_ABI_CWindows_CFoundation_CRect;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

typedef enum __x_ABI_CWindows_CUI_CPopups_CPlacement __x_ABI_CWindows_CUI_CPopups_CPlacement;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentBusyStatus __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentBusyStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentCalendarOtherAppReadAccess __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentCalendarOtherAppReadAccess;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentCalendarOtherAppWriteAccess __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentCalendarOtherAppWriteAccess;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentCalendarSyncStatus __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentCalendarSyncStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentConflictType __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentConflictType;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentDaysOfWeek __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentDaysOfWeek;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentDetailsKind __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentDetailsKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentParticipantResponse __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentParticipantResponse;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentParticipantRole __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentParticipantRole;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentRecurrenceUnit __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentRecurrenceUnit;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentSensitivity __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentSensitivity;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentStoreAccessType __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentStoreAccessType;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentStoreChangeType __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentStoreChangeType;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentSummaryCardView __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentSummaryCardView;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentWeekOfMonth __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentWeekOfMonth;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CFindAppointmentCalendarsOptions __x_ABI_CWindows_CApplicationModel_CAppointments_CFindAppointmentCalendarsOptions;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CRecurrenceType __x_ABI_CWindows_CApplicationModel_CAppointments_CRecurrenceType;

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentBusyStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentBusyStatus
{
    AppointmentBusyStatus_Busy = 0,
    AppointmentBusyStatus_Tentative = 1,
    AppointmentBusyStatus_Free = 2,
    AppointmentBusyStatus_OutOfOffice = 3,
    AppointmentBusyStatus_WorkingElsewhere = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentCalendarOtherAppReadAccess
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentCalendarOtherAppReadAccess
{
    AppointmentCalendarOtherAppReadAccess_SystemOnly = 0,
    AppointmentCalendarOtherAppReadAccess_Limited = 1,
    AppointmentCalendarOtherAppReadAccess_Full = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AppointmentCalendarOtherAppReadAccess_None = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentCalendarOtherAppWriteAccess
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentCalendarOtherAppWriteAccess
{
    AppointmentCalendarOtherAppWriteAccess_None = 0,
    AppointmentCalendarOtherAppWriteAccess_SystemOnly = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    AppointmentCalendarOtherAppWriteAccess_Limited = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentCalendarSyncStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentCalendarSyncStatus
{
    AppointmentCalendarSyncStatus_Idle = 0,
    AppointmentCalendarSyncStatus_Syncing = 1,
    AppointmentCalendarSyncStatus_UpToDate = 2,
    AppointmentCalendarSyncStatus_AuthenticationError = 3,
    AppointmentCalendarSyncStatus_PolicyError = 4,
    AppointmentCalendarSyncStatus_UnknownError = 5,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
    AppointmentCalendarSyncStatus_ManualAccountRemovalRequired = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentConflictType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentConflictType
{
    AppointmentConflictType_None = 0,
    AppointmentConflictType_Adjacent = 1,
    AppointmentConflictType_Overlap = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentDaysOfWeek
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentDaysOfWeek
{
    AppointmentDaysOfWeek_None = 0,
    AppointmentDaysOfWeek_Sunday = 0x1,
    AppointmentDaysOfWeek_Monday = 0x2,
    AppointmentDaysOfWeek_Tuesday = 0x4,
    AppointmentDaysOfWeek_Wednesday = 0x8,
    AppointmentDaysOfWeek_Thursday = 0x10,
    AppointmentDaysOfWeek_Friday = 0x20,
    AppointmentDaysOfWeek_Saturday = 0x40,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentDetailsKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentDetailsKind
{
    AppointmentDetailsKind_PlainText = 0,
    AppointmentDetailsKind_Html = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentParticipantResponse
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentParticipantResponse
{
    AppointmentParticipantResponse_None = 0,
    AppointmentParticipantResponse_Tentative = 1,
    AppointmentParticipantResponse_Accepted = 2,
    AppointmentParticipantResponse_Declined = 3,
    AppointmentParticipantResponse_Unknown = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentParticipantRole
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentParticipantRole
{
    AppointmentParticipantRole_RequiredAttendee = 0,
    AppointmentParticipantRole_OptionalAttendee = 1,
    AppointmentParticipantRole_Resource = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentRecurrenceUnit
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentRecurrenceUnit
{
    AppointmentRecurrenceUnit_Daily = 0,
    AppointmentRecurrenceUnit_Weekly = 1,
    AppointmentRecurrenceUnit_Monthly = 2,
    AppointmentRecurrenceUnit_MonthlyOnDay = 3,
    AppointmentRecurrenceUnit_Yearly = 4,
    AppointmentRecurrenceUnit_YearlyOnDay = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentSensitivity
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentSensitivity
{
    AppointmentSensitivity_Public = 0,
    AppointmentSensitivity_Private = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentStoreAccessType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentStoreAccessType
{
    AppointmentStoreAccessType_AppCalendarsReadWrite = 0,
    AppointmentStoreAccessType_AllCalendarsReadOnly = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    AppointmentStoreAccessType_AllCalendarsReadWrite = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentStoreChangeType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentStoreChangeType
{
    AppointmentStoreChangeType_AppointmentCreated = 0,
    AppointmentStoreChangeType_AppointmentModified = 1,
    AppointmentStoreChangeType_AppointmentDeleted = 2,
    AppointmentStoreChangeType_ChangeTrackingLost = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    AppointmentStoreChangeType_CalendarCreated = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    AppointmentStoreChangeType_CalendarModified = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    AppointmentStoreChangeType_CalendarDeleted = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentSummaryCardView
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentSummaryCardView
{
    AppointmentSummaryCardView_System = 0,
    AppointmentSummaryCardView_App = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.AppointmentWeekOfMonth
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentWeekOfMonth
{
    AppointmentWeekOfMonth_First = 0,
    AppointmentWeekOfMonth_Second = 1,
    AppointmentWeekOfMonth_Third = 2,
    AppointmentWeekOfMonth_Fourth = 3,
    AppointmentWeekOfMonth_Last = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.FindAppointmentCalendarsOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CFindAppointmentCalendarsOptions
{
    FindAppointmentCalendarsOptions_None = 0,
    FindAppointmentCalendarsOptions_IncludeHidden = 0x1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Appointments.RecurrenceType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CAppointments_CRecurrenceType
{
    RecurrenceType_Master = 0,
    RecurrenceType_Instance = 1,
    RecurrenceType_ExceptionInstance = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.Appointment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointment[] = L"Windows.ApplicationModel.Appointments.IAppointment";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_StartTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* put_StartTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_Duration)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_Location)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Location)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Subject)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Subject)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Details)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Details)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Reminder)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* put_Reminder)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Organizer)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant** value);
    HRESULT (STDMETHODCALLTYPE* put_Organizer)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant* value);
    HRESULT (STDMETHODCALLTYPE* get_Invitees)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        __FIVector_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee** value);
    HRESULT (STDMETHODCALLTYPE* get_Recurrence)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence** value);
    HRESULT (STDMETHODCALLTYPE* put_Recurrence)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* value);
    HRESULT (STDMETHODCALLTYPE* get_BusyStatus)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentBusyStatus* value);
    HRESULT (STDMETHODCALLTYPE* put_BusyStatus)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentBusyStatus value);
    HRESULT (STDMETHODCALLTYPE* get_AllDay)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllDay)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Sensitivity)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentSensitivity* value);
    HRESULT (STDMETHODCALLTYPE* put_Sensitivity)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentSensitivity value);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_Uri)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_get_StartTime(This, value) \
    ((This)->lpVtbl->get_StartTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_put_StartTime(This, value) \
    ((This)->lpVtbl->put_StartTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_put_Duration(This, value) \
    ((This)->lpVtbl->put_Duration(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_get_Location(This, value) \
    ((This)->lpVtbl->get_Location(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_put_Location(This, value) \
    ((This)->lpVtbl->put_Location(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_get_Subject(This, value) \
    ((This)->lpVtbl->get_Subject(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_put_Subject(This, value) \
    ((This)->lpVtbl->put_Subject(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_get_Details(This, value) \
    ((This)->lpVtbl->get_Details(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_put_Details(This, value) \
    ((This)->lpVtbl->put_Details(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_get_Reminder(This, value) \
    ((This)->lpVtbl->get_Reminder(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_put_Reminder(This, value) \
    ((This)->lpVtbl->put_Reminder(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_get_Organizer(This, value) \
    ((This)->lpVtbl->get_Organizer(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_put_Organizer(This, value) \
    ((This)->lpVtbl->put_Organizer(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_get_Invitees(This, value) \
    ((This)->lpVtbl->get_Invitees(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_get_Recurrence(This, value) \
    ((This)->lpVtbl->get_Recurrence(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_put_Recurrence(This, value) \
    ((This)->lpVtbl->put_Recurrence(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_get_BusyStatus(This, value) \
    ((This)->lpVtbl->get_BusyStatus(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_put_BusyStatus(This, value) \
    ((This)->lpVtbl->put_BusyStatus(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_get_AllDay(This, value) \
    ((This)->lpVtbl->get_AllDay(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_put_AllDay(This, value) \
    ((This)->lpVtbl->put_AllDay(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_get_Sensitivity(This, value) \
    ((This)->lpVtbl->get_Sensitivity(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_put_Sensitivity(This, value) \
    ((This)->lpVtbl->put_Sensitivity(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_put_Uri(This, value) \
    ((This)->lpVtbl->put_Uri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointment2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.Appointment
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointment2[] = L"Windows.ApplicationModel.Appointments.IAppointment2";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LocalId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CalendarId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RoamingId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_RoamingId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_OriginalStartTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_IsResponseRequested)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsResponseRequested)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AllowNewTimeProposal)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowNewTimeProposal)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_OnlineMeetingLink)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_OnlineMeetingLink)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ReplyTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_ReplyTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_UserResponse)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentParticipantResponse* value);
    HRESULT (STDMETHODCALLTYPE* put_UserResponse)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentParticipantResponse value);
    HRESULT (STDMETHODCALLTYPE* get_HasInvitees)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsCanceledMeeting)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsCanceledMeeting)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsOrganizedByUser)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsOrganizedByUser)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_get_LocalId(This, value) \
    ((This)->lpVtbl->get_LocalId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_get_CalendarId(This, value) \
    ((This)->lpVtbl->get_CalendarId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_get_RoamingId(This, value) \
    ((This)->lpVtbl->get_RoamingId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_put_RoamingId(This, value) \
    ((This)->lpVtbl->put_RoamingId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_get_OriginalStartTime(This, value) \
    ((This)->lpVtbl->get_OriginalStartTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_get_IsResponseRequested(This, value) \
    ((This)->lpVtbl->get_IsResponseRequested(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_put_IsResponseRequested(This, value) \
    ((This)->lpVtbl->put_IsResponseRequested(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_get_AllowNewTimeProposal(This, value) \
    ((This)->lpVtbl->get_AllowNewTimeProposal(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_put_AllowNewTimeProposal(This, value) \
    ((This)->lpVtbl->put_AllowNewTimeProposal(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_get_OnlineMeetingLink(This, value) \
    ((This)->lpVtbl->get_OnlineMeetingLink(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_put_OnlineMeetingLink(This, value) \
    ((This)->lpVtbl->put_OnlineMeetingLink(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_get_ReplyTime(This, value) \
    ((This)->lpVtbl->get_ReplyTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_put_ReplyTime(This, value) \
    ((This)->lpVtbl->put_ReplyTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_get_UserResponse(This, value) \
    ((This)->lpVtbl->get_UserResponse(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_put_UserResponse(This, value) \
    ((This)->lpVtbl->put_UserResponse(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_get_HasInvitees(This, value) \
    ((This)->lpVtbl->get_HasInvitees(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_get_IsCanceledMeeting(This, value) \
    ((This)->lpVtbl->get_IsCanceledMeeting(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_put_IsCanceledMeeting(This, value) \
    ((This)->lpVtbl->put_IsCanceledMeeting(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_get_IsOrganizedByUser(This, value) \
    ((This)->lpVtbl->get_IsOrganizedByUser(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_put_IsOrganizedByUser(This, value) \
    ((This)->lpVtbl->put_IsOrganizedByUser(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointment3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.Appointment
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointment2
 *     Windows.ApplicationModel.Appointments.IAppointment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointment3[] = L"Windows.ApplicationModel.Appointments.IAppointment3";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChangeNumber)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteChangeNumber)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* put_RemoteChangeNumber)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3* This,
        UINT64 value);
    HRESULT (STDMETHODCALLTYPE* get_DetailsKind)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentDetailsKind* value);
    HRESULT (STDMETHODCALLTYPE* put_DetailsKind)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentDetailsKind value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_get_ChangeNumber(This, value) \
    ((This)->lpVtbl->get_ChangeNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_get_RemoteChangeNumber(This, value) \
    ((This)->lpVtbl->get_RemoteChangeNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_put_RemoteChangeNumber(This, value) \
    ((This)->lpVtbl->put_RemoteChangeNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_get_DetailsKind(This, value) \
    ((This)->lpVtbl->get_DetailsKind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_put_DetailsKind(This, value) \
    ((This)->lpVtbl->put_DetailsKind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentCalendar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentCalendar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentCalendar[] = L"Windows.ApplicationModel.Appointments.IAppointmentCalendar";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayColor)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_LocalId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsHidden)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_OtherAppReadAccess)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentCalendarOtherAppReadAccess* value);
    HRESULT (STDMETHODCALLTYPE* put_OtherAppReadAccess)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentCalendarOtherAppReadAccess value);
    HRESULT (STDMETHODCALLTYPE* get_OtherAppWriteAccess)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentCalendarOtherAppWriteAccess* value);
    HRESULT (STDMETHODCALLTYPE* put_OtherAppWriteAccess)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentCalendarOtherAppWriteAccess value);
    HRESULT (STDMETHODCALLTYPE* get_SourceDisplayName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SummaryCardView)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentSummaryCardView* value);
    HRESULT (STDMETHODCALLTYPE* put_SummaryCardView)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentSummaryCardView value);
    HRESULT (STDMETHODCALLTYPE* FindAppointmentsAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime rangeStart,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan rangeLength,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** result);
    HRESULT (STDMETHODCALLTYPE* FindAppointmentsAsyncWithOptions)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime rangeStart,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan rangeLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions* options,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** result);
    HRESULT (STDMETHODCALLTYPE* FindExceptionsFromMasterAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        HSTRING masterLocalId,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentException** value);
    HRESULT (STDMETHODCALLTYPE* FindAllInstancesAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        HSTRING masterLocalId,
        struct __x_ABI_CWindows_CFoundation_CDateTime rangeStart,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan rangeLength,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** value);
    HRESULT (STDMETHODCALLTYPE* FindAllInstancesAsyncWithOptions)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        HSTRING masterLocalId,
        struct __x_ABI_CWindows_CFoundation_CDateTime rangeStart,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan rangeLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions* pOptions,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** value);
    HRESULT (STDMETHODCALLTYPE* GetAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        HSTRING localId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment** result);
    HRESULT (STDMETHODCALLTYPE* GetAppointmentInstanceAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        HSTRING localId,
        struct __x_ABI_CWindows_CFoundation_CDateTime instanceStartTime,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment** result);
    HRESULT (STDMETHODCALLTYPE* FindUnexpandedAppointmentsAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** result);
    HRESULT (STDMETHODCALLTYPE* FindUnexpandedAppointmentsAsyncWithOptions)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions* options,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** result);
    HRESULT (STDMETHODCALLTYPE* DeleteAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);
    HRESULT (STDMETHODCALLTYPE* SaveAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);
    HRESULT (STDMETHODCALLTYPE* DeleteAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        HSTRING localId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);
    HRESULT (STDMETHODCALLTYPE* DeleteAppointmentInstanceAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        HSTRING localId,
        struct __x_ABI_CWindows_CFoundation_CDateTime instanceStartTime,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);
    HRESULT (STDMETHODCALLTYPE* SaveAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* pAppointment,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_get_DisplayColor(This, value) \
    ((This)->lpVtbl->get_DisplayColor(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_get_LocalId(This, value) \
    ((This)->lpVtbl->get_LocalId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_get_IsHidden(This, value) \
    ((This)->lpVtbl->get_IsHidden(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_get_OtherAppReadAccess(This, value) \
    ((This)->lpVtbl->get_OtherAppReadAccess(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_put_OtherAppReadAccess(This, value) \
    ((This)->lpVtbl->put_OtherAppReadAccess(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_get_OtherAppWriteAccess(This, value) \
    ((This)->lpVtbl->get_OtherAppWriteAccess(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_put_OtherAppWriteAccess(This, value) \
    ((This)->lpVtbl->put_OtherAppWriteAccess(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_get_SourceDisplayName(This, value) \
    ((This)->lpVtbl->get_SourceDisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_get_SummaryCardView(This, value) \
    ((This)->lpVtbl->get_SummaryCardView(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_put_SummaryCardView(This, value) \
    ((This)->lpVtbl->put_SummaryCardView(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FindAppointmentsAsync(This, rangeStart, rangeLength, result) \
    ((This)->lpVtbl->FindAppointmentsAsync(This, rangeStart, rangeLength, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FindAppointmentsAsyncWithOptions(This, rangeStart, rangeLength, options, result) \
    ((This)->lpVtbl->FindAppointmentsAsyncWithOptions(This, rangeStart, rangeLength, options, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FindExceptionsFromMasterAsync(This, masterLocalId, value) \
    ((This)->lpVtbl->FindExceptionsFromMasterAsync(This, masterLocalId, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FindAllInstancesAsync(This, masterLocalId, rangeStart, rangeLength, value) \
    ((This)->lpVtbl->FindAllInstancesAsync(This, masterLocalId, rangeStart, rangeLength, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FindAllInstancesAsyncWithOptions(This, masterLocalId, rangeStart, rangeLength, pOptions, value) \
    ((This)->lpVtbl->FindAllInstancesAsyncWithOptions(This, masterLocalId, rangeStart, rangeLength, pOptions, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_GetAppointmentAsync(This, localId, result) \
    ((This)->lpVtbl->GetAppointmentAsync(This, localId, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_GetAppointmentInstanceAsync(This, localId, instanceStartTime, result) \
    ((This)->lpVtbl->GetAppointmentInstanceAsync(This, localId, instanceStartTime, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FindUnexpandedAppointmentsAsync(This, result) \
    ((This)->lpVtbl->FindUnexpandedAppointmentsAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_FindUnexpandedAppointmentsAsyncWithOptions(This, options, result) \
    ((This)->lpVtbl->FindUnexpandedAppointmentsAsyncWithOptions(This, options, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_DeleteAsync(This, asyncAction) \
    ((This)->lpVtbl->DeleteAsync(This, asyncAction))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_SaveAsync(This, asyncAction) \
    ((This)->lpVtbl->SaveAsync(This, asyncAction))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_DeleteAppointmentAsync(This, localId, asyncAction) \
    ((This)->lpVtbl->DeleteAppointmentAsync(This, localId, asyncAction))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_DeleteAppointmentInstanceAsync(This, localId, instanceStartTime, asyncAction) \
    ((This)->lpVtbl->DeleteAppointmentInstanceAsync(This, localId, instanceStartTime, asyncAction))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_SaveAppointmentAsync(This, pAppointment, asyncAction) \
    ((This)->lpVtbl->SaveAppointmentAsync(This, pAppointment, asyncAction))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentCalendar2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentCalendar
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointmentCalendar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentCalendar2[] = L"Windows.ApplicationModel.Appointments.IAppointmentCalendar2";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SyncManager)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager** value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_RemoteId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayColor)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        struct __x_ABI_CWindows_CUI_CColor value);
    HRESULT (STDMETHODCALLTYPE* put_IsHidden)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_UserDataAccountId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CanCreateOrUpdateAppointments)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CanCreateOrUpdateAppointments)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CanCancelMeetings)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CanCancelMeetings)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CanForwardMeetings)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CanForwardMeetings)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CanProposeNewTimeForMeetings)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CanProposeNewTimeForMeetings)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CanUpdateMeetingResponses)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CanUpdateMeetingResponses)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_CanNotifyInvitees)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_CanNotifyInvitees)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_MustNofityInvitees)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_MustNofityInvitees)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* TryCreateOrUpdateAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        boolean notifyInvitees,
        __FIAsyncOperation_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* TryCancelMeetingAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* meeting,
        HSTRING subject,
        HSTRING comment,
        boolean notifyInvitees,
        __FIAsyncOperation_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* TryForwardMeetingAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* meeting,
        __FIIterable_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee* invitees,
        HSTRING subject,
        HSTRING forwardHeader,
        HSTRING comment,
        __FIAsyncOperation_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* TryProposeNewTimeForMeetingAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* meeting,
        struct __x_ABI_CWindows_CFoundation_CDateTime newStartTime,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan newDuration,
        HSTRING subject,
        HSTRING comment,
        __FIAsyncOperation_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* TryUpdateMeetingResponseAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* meeting,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentParticipantResponse response,
        HSTRING subject,
        HSTRING comment,
        boolean sendUpdate,
        __FIAsyncOperation_1_boolean** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_get_SyncManager(This, value) \
    ((This)->lpVtbl->get_SyncManager(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_get_RemoteId(This, value) \
    ((This)->lpVtbl->get_RemoteId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_put_RemoteId(This, value) \
    ((This)->lpVtbl->put_RemoteId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_put_DisplayColor(This, value) \
    ((This)->lpVtbl->put_DisplayColor(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_put_IsHidden(This, value) \
    ((This)->lpVtbl->put_IsHidden(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_get_UserDataAccountId(This, value) \
    ((This)->lpVtbl->get_UserDataAccountId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_get_CanCreateOrUpdateAppointments(This, value) \
    ((This)->lpVtbl->get_CanCreateOrUpdateAppointments(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_put_CanCreateOrUpdateAppointments(This, value) \
    ((This)->lpVtbl->put_CanCreateOrUpdateAppointments(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_get_CanCancelMeetings(This, value) \
    ((This)->lpVtbl->get_CanCancelMeetings(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_put_CanCancelMeetings(This, value) \
    ((This)->lpVtbl->put_CanCancelMeetings(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_get_CanForwardMeetings(This, value) \
    ((This)->lpVtbl->get_CanForwardMeetings(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_put_CanForwardMeetings(This, value) \
    ((This)->lpVtbl->put_CanForwardMeetings(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_get_CanProposeNewTimeForMeetings(This, value) \
    ((This)->lpVtbl->get_CanProposeNewTimeForMeetings(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_put_CanProposeNewTimeForMeetings(This, value) \
    ((This)->lpVtbl->put_CanProposeNewTimeForMeetings(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_get_CanUpdateMeetingResponses(This, value) \
    ((This)->lpVtbl->get_CanUpdateMeetingResponses(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_put_CanUpdateMeetingResponses(This, value) \
    ((This)->lpVtbl->put_CanUpdateMeetingResponses(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_get_CanNotifyInvitees(This, value) \
    ((This)->lpVtbl->get_CanNotifyInvitees(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_put_CanNotifyInvitees(This, value) \
    ((This)->lpVtbl->put_CanNotifyInvitees(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_get_MustNofityInvitees(This, value) \
    ((This)->lpVtbl->get_MustNofityInvitees(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_put_MustNofityInvitees(This, value) \
    ((This)->lpVtbl->put_MustNofityInvitees(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_TryCreateOrUpdateAppointmentAsync(This, appointment, notifyInvitees, result) \
    ((This)->lpVtbl->TryCreateOrUpdateAppointmentAsync(This, appointment, notifyInvitees, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_TryCancelMeetingAsync(This, meeting, subject, comment, notifyInvitees, result) \
    ((This)->lpVtbl->TryCancelMeetingAsync(This, meeting, subject, comment, notifyInvitees, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_TryForwardMeetingAsync(This, meeting, invitees, subject, forwardHeader, comment, result) \
    ((This)->lpVtbl->TryForwardMeetingAsync(This, meeting, invitees, subject, forwardHeader, comment, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_TryProposeNewTimeForMeetingAsync(This, meeting, newStartTime, newDuration, subject, comment, result) \
    ((This)->lpVtbl->TryProposeNewTimeForMeetingAsync(This, meeting, newStartTime, newDuration, subject, comment, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_TryUpdateMeetingResponseAsync(This, meeting, response, subject, comment, sendUpdate, result) \
    ((This)->lpVtbl->TryUpdateMeetingResponseAsync(This, meeting, response, subject, comment, sendUpdate, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentCalendar3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentCalendar
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentCalendar3[] = L"Windows.ApplicationModel.Appointments.IAppointmentCalendar3";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RegisterSyncManagerAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_RegisterSyncManagerAsync(This, result) \
    ((This)->lpVtbl->RegisterSyncManagerAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentCalendarSyncManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentCalendarSyncManager[] = L"Windows.ApplicationModel.Appointments.IAppointmentCalendarSyncManager";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentCalendarSyncStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_LastSuccessfulSyncTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_LastAttemptedSyncTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* SyncAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager* This,
        __FIAsyncOperation_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* add_SyncStatusChanged)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentCalendarSyncManager_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SyncStatusChanged)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManagerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_get_LastSuccessfulSyncTime(This, value) \
    ((This)->lpVtbl->get_LastSuccessfulSyncTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_get_LastAttemptedSyncTime(This, value) \
    ((This)->lpVtbl->get_LastAttemptedSyncTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_SyncAsync(This, result) \
    ((This)->lpVtbl->SyncAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_add_SyncStatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_SyncStatusChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_remove_SyncStatusChanged(This, token) \
    ((This)->lpVtbl->remove_SyncStatusChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentCalendarSyncManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentCalendarSyncManager2[] = L"Windows.ApplicationModel.Appointments.IAppointmentCalendarSyncManager2";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Status)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentCalendarSyncStatus value);
    HRESULT (STDMETHODCALLTYPE* put_LastSuccessfulSyncTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* put_LastAttemptedSyncTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_put_Status(This, value) \
    ((This)->lpVtbl->put_Status(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_put_LastSuccessfulSyncTime(This, value) \
    ((This)->lpVtbl->put_LastSuccessfulSyncTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_put_LastAttemptedSyncTime(This, value) \
    ((This)->lpVtbl->put_LastAttemptedSyncTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendarSyncManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentConflictResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentConflictResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentConflictResult[] = L"Windows.ApplicationModel.Appointments.IAppointmentConflictResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentConflictType* value);
    HRESULT (STDMETHODCALLTYPE* get_Date)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_get_Date(This, value) \
    ((This)->lpVtbl->get_Date(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentConflictResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentException
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentException
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentException[] = L"Windows.ApplicationModel.Appointments.IAppointmentException";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentExceptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Appointment)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment** value);
    HRESULT (STDMETHODCALLTYPE* get_ExceptionProperties)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_IsDeleted)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentExceptionVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentExceptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_get_Appointment(This, value) \
    ((This)->lpVtbl->get_Appointment(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_get_ExceptionProperties(This, value) \
    ((This)->lpVtbl->get_ExceptionProperties(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_get_IsDeleted(This, value) \
    ((This)->lpVtbl->get_IsDeleted(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentException_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentInvitee
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentInvitee
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointmentParticipant
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentInvitee[] = L"Windows.ApplicationModel.Appointments.IAppointmentInvitee";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInviteeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Role)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentParticipantRole* value);
    HRESULT (STDMETHODCALLTYPE* put_Role)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentParticipantRole value);
    HRESULT (STDMETHODCALLTYPE* get_Response)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentParticipantResponse* value);
    HRESULT (STDMETHODCALLTYPE* put_Response)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentParticipantResponse value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInviteeVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInviteeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_get_Role(This, value) \
    ((This)->lpVtbl->get_Role(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_put_Role(This, value) \
    ((This)->lpVtbl->put_Role(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_get_Response(This, value) \
    ((This)->lpVtbl->get_Response(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_put_Response(This, value) \
    ((This)->lpVtbl->put_Response(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentManagerForUser[] = L"Windows.ApplicationModel.Appointments.IAppointmentManagerForUser";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUserVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ShowAddAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        __FIAsyncOperation_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* ShowAddAppointmentWithPlacementAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement,
        __FIAsyncOperation_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* ShowReplaceAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        HSTRING appointmentId,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        __FIAsyncOperation_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* ShowReplaceAppointmentWithPlacementAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        HSTRING appointmentId,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement,
        __FIAsyncOperation_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* ShowReplaceAppointmentWithPlacementAndDateAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        HSTRING appointmentId,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement,
        struct __x_ABI_CWindows_CFoundation_CDateTime instanceStartDate,
        __FIAsyncOperation_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* ShowRemoveAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        HSTRING appointmentId,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        __FIAsyncOperation_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* ShowRemoveAppointmentWithPlacementAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        HSTRING appointmentId,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement,
        __FIAsyncOperation_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* ShowRemoveAppointmentWithPlacementAndDateAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        HSTRING appointmentId,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement,
        struct __x_ABI_CWindows_CFoundation_CDateTime instanceStartDate,
        __FIAsyncOperation_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* ShowTimeFrameAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime timeToShow,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan duration,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ShowAppointmentDetailsAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        HSTRING appointmentId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ShowAppointmentDetailsWithDateAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        HSTRING appointmentId,
        struct __x_ABI_CWindows_CFoundation_CDateTime instanceStartDate,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ShowEditNewAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        __FIAsyncOperation_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* RequestStoreAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentStoreAccessType options,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore** result);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUserVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUserVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_ShowAddAppointmentAsync(This, appointment, selection, result) \
    ((This)->lpVtbl->ShowAddAppointmentAsync(This, appointment, selection, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_ShowAddAppointmentWithPlacementAsync(This, appointment, selection, preferredPlacement, result) \
    ((This)->lpVtbl->ShowAddAppointmentWithPlacementAsync(This, appointment, selection, preferredPlacement, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_ShowReplaceAppointmentAsync(This, appointmentId, appointment, selection, result) \
    ((This)->lpVtbl->ShowReplaceAppointmentAsync(This, appointmentId, appointment, selection, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_ShowReplaceAppointmentWithPlacementAsync(This, appointmentId, appointment, selection, preferredPlacement, result) \
    ((This)->lpVtbl->ShowReplaceAppointmentWithPlacementAsync(This, appointmentId, appointment, selection, preferredPlacement, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_ShowReplaceAppointmentWithPlacementAndDateAsync(This, appointmentId, appointment, selection, preferredPlacement, instanceStartDate, result) \
    ((This)->lpVtbl->ShowReplaceAppointmentWithPlacementAndDateAsync(This, appointmentId, appointment, selection, preferredPlacement, instanceStartDate, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_ShowRemoveAppointmentAsync(This, appointmentId, selection, result) \
    ((This)->lpVtbl->ShowRemoveAppointmentAsync(This, appointmentId, selection, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_ShowRemoveAppointmentWithPlacementAsync(This, appointmentId, selection, preferredPlacement, result) \
    ((This)->lpVtbl->ShowRemoveAppointmentWithPlacementAsync(This, appointmentId, selection, preferredPlacement, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_ShowRemoveAppointmentWithPlacementAndDateAsync(This, appointmentId, selection, preferredPlacement, instanceStartDate, result) \
    ((This)->lpVtbl->ShowRemoveAppointmentWithPlacementAndDateAsync(This, appointmentId, selection, preferredPlacement, instanceStartDate, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_ShowTimeFrameAsync(This, timeToShow, duration, result) \
    ((This)->lpVtbl->ShowTimeFrameAsync(This, timeToShow, duration, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_ShowAppointmentDetailsAsync(This, appointmentId, result) \
    ((This)->lpVtbl->ShowAppointmentDetailsAsync(This, appointmentId, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_ShowAppointmentDetailsWithDateAsync(This, appointmentId, instanceStartDate, result) \
    ((This)->lpVtbl->ShowAppointmentDetailsWithDateAsync(This, appointmentId, instanceStartDate, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_ShowEditNewAppointmentAsync(This, appointment, result) \
    ((This)->lpVtbl->ShowEditNewAppointmentAsync(This, appointment, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_RequestStoreAsync(This, options, result) \
    ((This)->lpVtbl->RequestStoreAsync(This, options, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentManagerStatics[] = L"Windows.ApplicationModel.Appointments.IAppointmentManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ShowAddAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* ShowAddAppointmentWithPlacementAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* ShowReplaceAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics* This,
        HSTRING appointmentId,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* ShowReplaceAppointmentWithPlacementAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics* This,
        HSTRING appointmentId,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* ShowReplaceAppointmentWithPlacementAndDateAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics* This,
        HSTRING appointmentId,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement,
        struct __x_ABI_CWindows_CFoundation_CDateTime instanceStartDate,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* ShowRemoveAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics* This,
        HSTRING appointmentId,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* ShowRemoveAppointmentWithPlacementAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics* This,
        HSTRING appointmentId,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* ShowRemoveAppointmentWithPlacementAndDateAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics* This,
        HSTRING appointmentId,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement,
        struct __x_ABI_CWindows_CFoundation_CDateTime instanceStartDate,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* ShowTimeFrameAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime timeToShow,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan duration,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_ShowAddAppointmentAsync(This, appointment, selection, operation) \
    ((This)->lpVtbl->ShowAddAppointmentAsync(This, appointment, selection, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_ShowAddAppointmentWithPlacementAsync(This, appointment, selection, preferredPlacement, operation) \
    ((This)->lpVtbl->ShowAddAppointmentWithPlacementAsync(This, appointment, selection, preferredPlacement, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_ShowReplaceAppointmentAsync(This, appointmentId, appointment, selection, operation) \
    ((This)->lpVtbl->ShowReplaceAppointmentAsync(This, appointmentId, appointment, selection, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_ShowReplaceAppointmentWithPlacementAsync(This, appointmentId, appointment, selection, preferredPlacement, operation) \
    ((This)->lpVtbl->ShowReplaceAppointmentWithPlacementAsync(This, appointmentId, appointment, selection, preferredPlacement, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_ShowReplaceAppointmentWithPlacementAndDateAsync(This, appointmentId, appointment, selection, preferredPlacement, instanceStartDate, operation) \
    ((This)->lpVtbl->ShowReplaceAppointmentWithPlacementAndDateAsync(This, appointmentId, appointment, selection, preferredPlacement, instanceStartDate, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_ShowRemoveAppointmentAsync(This, appointmentId, selection, operation) \
    ((This)->lpVtbl->ShowRemoveAppointmentAsync(This, appointmentId, selection, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_ShowRemoveAppointmentWithPlacementAsync(This, appointmentId, selection, preferredPlacement, operation) \
    ((This)->lpVtbl->ShowRemoveAppointmentWithPlacementAsync(This, appointmentId, selection, preferredPlacement, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_ShowRemoveAppointmentWithPlacementAndDateAsync(This, appointmentId, selection, preferredPlacement, instanceStartDate, operation) \
    ((This)->lpVtbl->ShowRemoveAppointmentWithPlacementAndDateAsync(This, appointmentId, selection, preferredPlacement, instanceStartDate, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_ShowTimeFrameAsync(This, timeToShow, duration, asyncAction) \
    ((This)->lpVtbl->ShowTimeFrameAsync(This, timeToShow, duration, asyncAction))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentManagerStatics2[] = L"Windows.ApplicationModel.Appointments.IAppointmentManagerStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ShowAppointmentDetailsAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2* This,
        HSTRING appointmentId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);
    HRESULT (STDMETHODCALLTYPE* ShowAppointmentDetailsWithDateAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2* This,
        HSTRING appointmentId,
        struct __x_ABI_CWindows_CFoundation_CDateTime instanceStartDate,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);
    HRESULT (STDMETHODCALLTYPE* ShowEditNewAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* RequestStoreAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentStoreAccessType options,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentStore** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_ShowAppointmentDetailsAsync(This, appointmentId, asyncAction) \
    ((This)->lpVtbl->ShowAppointmentDetailsAsync(This, appointmentId, asyncAction))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_ShowAppointmentDetailsWithDateAsync(This, appointmentId, instanceStartDate, asyncAction) \
    ((This)->lpVtbl->ShowAppointmentDetailsWithDateAsync(This, appointmentId, instanceStartDate, asyncAction))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_ShowEditNewAppointmentAsync(This, appointment, operation) \
    ((This)->lpVtbl->ShowEditNewAppointmentAsync(This, appointment, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_RequestStoreAsync(This, options, operation) \
    ((This)->lpVtbl->RequestStoreAsync(This, options, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentManagerStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentManagerStatics3[] = L"Windows.ApplicationModel.Appointments.IAppointmentManagerStatics3";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerForUser** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_GetForUser(This, user, result) \
    ((This)->lpVtbl->GetForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentParticipant
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentParticipant[] = L"Windows.ApplicationModel.Appointments.IAppointmentParticipant";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipantVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Address)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Address)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipantVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipantVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_get_Address(This, value) \
    ((This)->lpVtbl->get_Address(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_put_Address(This, value) \
    ((This)->lpVtbl->put_Address(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentParticipant_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentPropertiesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentPropertiesStatics[] = L"Windows.ApplicationModel.Appointments.IAppointmentPropertiesStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Subject)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Location)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_StartTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Reminder)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BusyStatus)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Sensitivity)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_OriginalStartTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsResponseRequested)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AllowNewTimeProposal)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AllDay)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Details)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_OnlineMeetingLink)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ReplyTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Organizer)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_UserResponse)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HasInvitees)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsCanceledMeeting)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsOrganizedByUser)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Recurrence)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Invitees)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DefaultProperties)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics* This,
        __FIVector_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_Subject(This, value) \
    ((This)->lpVtbl->get_Subject(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_Location(This, value) \
    ((This)->lpVtbl->get_Location(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_StartTime(This, value) \
    ((This)->lpVtbl->get_StartTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_Reminder(This, value) \
    ((This)->lpVtbl->get_Reminder(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_BusyStatus(This, value) \
    ((This)->lpVtbl->get_BusyStatus(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_Sensitivity(This, value) \
    ((This)->lpVtbl->get_Sensitivity(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_OriginalStartTime(This, value) \
    ((This)->lpVtbl->get_OriginalStartTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_IsResponseRequested(This, value) \
    ((This)->lpVtbl->get_IsResponseRequested(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_AllowNewTimeProposal(This, value) \
    ((This)->lpVtbl->get_AllowNewTimeProposal(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_AllDay(This, value) \
    ((This)->lpVtbl->get_AllDay(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_Details(This, value) \
    ((This)->lpVtbl->get_Details(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_OnlineMeetingLink(This, value) \
    ((This)->lpVtbl->get_OnlineMeetingLink(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_ReplyTime(This, value) \
    ((This)->lpVtbl->get_ReplyTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_Organizer(This, value) \
    ((This)->lpVtbl->get_Organizer(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_UserResponse(This, value) \
    ((This)->lpVtbl->get_UserResponse(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_HasInvitees(This, value) \
    ((This)->lpVtbl->get_HasInvitees(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_IsCanceledMeeting(This, value) \
    ((This)->lpVtbl->get_IsCanceledMeeting(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_IsOrganizedByUser(This, value) \
    ((This)->lpVtbl->get_IsOrganizedByUser(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_Recurrence(This, value) \
    ((This)->lpVtbl->get_Recurrence(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_Invitees(This, value) \
    ((This)->lpVtbl->get_Invitees(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_get_DefaultProperties(This, value) \
    ((This)->lpVtbl->get_DefaultProperties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentPropertiesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointmentPropertiesStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentPropertiesStatics2[] = L"Windows.ApplicationModel.Appointments.IAppointmentPropertiesStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChangeNumber)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteChangeNumber)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DetailsKind)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_get_ChangeNumber(This, value) \
    ((This)->lpVtbl->get_ChangeNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_get_RemoteChangeNumber(This, value) \
    ((This)->lpVtbl->get_RemoteChangeNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_get_DetailsKind(This, value) \
    ((This)->lpVtbl->get_DetailsKind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentPropertiesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentRecurrence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentRecurrence
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentRecurrence[] = L"Windows.ApplicationModel.Appointments.IAppointmentRecurrence";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Unit)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentRecurrenceUnit* value);
    HRESULT (STDMETHODCALLTYPE* put_Unit)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentRecurrenceUnit value);
    HRESULT (STDMETHODCALLTYPE* get_Occurrences)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* put_Occurrences)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        __FIReference_1_UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Until)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* put_Until)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Interval)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Interval)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_DaysOfWeek)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentDaysOfWeek* value);
    HRESULT (STDMETHODCALLTYPE* put_DaysOfWeek)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentDaysOfWeek value);
    HRESULT (STDMETHODCALLTYPE* get_WeekOfMonth)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentWeekOfMonth* value);
    HRESULT (STDMETHODCALLTYPE* put_WeekOfMonth)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentWeekOfMonth value);
    HRESULT (STDMETHODCALLTYPE* get_Month)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Month)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Day)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Day)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrenceVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_get_Unit(This, value) \
    ((This)->lpVtbl->get_Unit(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_put_Unit(This, value) \
    ((This)->lpVtbl->put_Unit(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_get_Occurrences(This, value) \
    ((This)->lpVtbl->get_Occurrences(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_put_Occurrences(This, value) \
    ((This)->lpVtbl->put_Occurrences(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_get_Until(This, value) \
    ((This)->lpVtbl->get_Until(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_put_Until(This, value) \
    ((This)->lpVtbl->put_Until(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_get_Interval(This, value) \
    ((This)->lpVtbl->get_Interval(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_put_Interval(This, value) \
    ((This)->lpVtbl->put_Interval(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_get_DaysOfWeek(This, value) \
    ((This)->lpVtbl->get_DaysOfWeek(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_put_DaysOfWeek(This, value) \
    ((This)->lpVtbl->put_DaysOfWeek(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_get_WeekOfMonth(This, value) \
    ((This)->lpVtbl->get_WeekOfMonth(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_put_WeekOfMonth(This, value) \
    ((This)->lpVtbl->put_WeekOfMonth(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_get_Month(This, value) \
    ((This)->lpVtbl->get_Month(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_put_Month(This, value) \
    ((This)->lpVtbl->put_Month(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_get_Day(This, value) \
    ((This)->lpVtbl->get_Day(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_put_Day(This, value) \
    ((This)->lpVtbl->put_Day(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentRecurrence2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentRecurrence
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointmentRecurrence
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentRecurrence2[] = L"Windows.ApplicationModel.Appointments.IAppointmentRecurrence2";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RecurrenceType)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CRecurrenceType* value);
    HRESULT (STDMETHODCALLTYPE* get_TimeZone)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_TimeZone)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_get_RecurrenceType(This, value) \
    ((This)->lpVtbl->get_RecurrenceType(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_get_TimeZone(This, value) \
    ((This)->lpVtbl->get_TimeZone(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_put_TimeZone(This, value) \
    ((This)->lpVtbl->put_TimeZone(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentRecurrence3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentRecurrence
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointmentRecurrence2
 *     Windows.ApplicationModel.Appointments.IAppointmentRecurrence
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentRecurrence3[] = L"Windows.ApplicationModel.Appointments.IAppointmentRecurrence3";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CalendarIdentifier)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_get_CalendarIdentifier(This, value) \
    ((This)->lpVtbl->get_CalendarIdentifier(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentRecurrence3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStore[] = L"Windows.ApplicationModel.Appointments.IAppointmentStore";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChangeTracker)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker** value);
    HRESULT (STDMETHODCALLTYPE* CreateAppointmentCalendarAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        HSTRING name,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** operation);
    HRESULT (STDMETHODCALLTYPE* GetAppointmentCalendarAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        HSTRING calendarId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** result);
    HRESULT (STDMETHODCALLTYPE* GetAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        HSTRING localId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment** result);
    HRESULT (STDMETHODCALLTYPE* GetAppointmentInstanceAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        HSTRING localId,
        struct __x_ABI_CWindows_CFoundation_CDateTime instanceStartTime,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointment** result);
    HRESULT (STDMETHODCALLTYPE* FindAppointmentCalendarsAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** result);
    HRESULT (STDMETHODCALLTYPE* FindAppointmentCalendarsAsyncWithOptions)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CFindAppointmentCalendarsOptions options,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** result);
    HRESULT (STDMETHODCALLTYPE* FindAppointmentsAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime rangeStart,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan rangeLength,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** result);
    HRESULT (STDMETHODCALLTYPE* FindAppointmentsAsyncWithOptions)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime rangeStart,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan rangeLength,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions* options,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointment** result);
    HRESULT (STDMETHODCALLTYPE* FindConflictAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult** result);
    HRESULT (STDMETHODCALLTYPE* FindConflictAsyncWithInstanceStart)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        struct __x_ABI_CWindows_CFoundation_CDateTime instanceStartTime,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentConflictResult** result);
    HRESULT (STDMETHODCALLTYPE* MoveAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar* destinationCalendar,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);
    HRESULT (STDMETHODCALLTYPE* ShowAddAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* ShowReplaceAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        HSTRING localId,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* ShowReplaceAppointmentWithPlacementAndDateAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        HSTRING localId,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement,
        struct __x_ABI_CWindows_CFoundation_CDateTime instanceStartDate,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* ShowRemoveAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        HSTRING localId,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* ShowRemoveAppointmentWithPlacementAndDateAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        HSTRING localId,
        struct __x_ABI_CWindows_CFoundation_CRect selection,
        enum __x_ABI_CWindows_CUI_CPopups_CPlacement preferredPlacement,
        struct __x_ABI_CWindows_CFoundation_CDateTime instanceStartDate,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* ShowAppointmentDetailsAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        HSTRING localId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);
    HRESULT (STDMETHODCALLTYPE* ShowAppointmentDetailsWithDateAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        HSTRING localId,
        struct __x_ABI_CWindows_CFoundation_CDateTime instanceStartDate,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);
    HRESULT (STDMETHODCALLTYPE* ShowEditNewAppointmentAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* appointment,
        __FIAsyncOperation_1_HSTRING** operation);
    HRESULT (STDMETHODCALLTYPE* FindLocalIdsFromRoamingIdAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore* This,
        HSTRING roamingId,
        __FIAsyncOperation_1___FIVectorView_1_HSTRING** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_get_ChangeTracker(This, value) \
    ((This)->lpVtbl->get_ChangeTracker(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_CreateAppointmentCalendarAsync(This, name, operation) \
    ((This)->lpVtbl->CreateAppointmentCalendarAsync(This, name, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_GetAppointmentCalendarAsync(This, calendarId, result) \
    ((This)->lpVtbl->GetAppointmentCalendarAsync(This, calendarId, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_GetAppointmentAsync(This, localId, result) \
    ((This)->lpVtbl->GetAppointmentAsync(This, localId, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_GetAppointmentInstanceAsync(This, localId, instanceStartTime, result) \
    ((This)->lpVtbl->GetAppointmentInstanceAsync(This, localId, instanceStartTime, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_FindAppointmentCalendarsAsync(This, result) \
    ((This)->lpVtbl->FindAppointmentCalendarsAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_FindAppointmentCalendarsAsyncWithOptions(This, options, result) \
    ((This)->lpVtbl->FindAppointmentCalendarsAsyncWithOptions(This, options, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_FindAppointmentsAsync(This, rangeStart, rangeLength, result) \
    ((This)->lpVtbl->FindAppointmentsAsync(This, rangeStart, rangeLength, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_FindAppointmentsAsyncWithOptions(This, rangeStart, rangeLength, options, result) \
    ((This)->lpVtbl->FindAppointmentsAsyncWithOptions(This, rangeStart, rangeLength, options, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_FindConflictAsync(This, appointment, result) \
    ((This)->lpVtbl->FindConflictAsync(This, appointment, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_FindConflictAsyncWithInstanceStart(This, appointment, instanceStartTime, result) \
    ((This)->lpVtbl->FindConflictAsyncWithInstanceStart(This, appointment, instanceStartTime, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_MoveAppointmentAsync(This, appointment, destinationCalendar, asyncAction) \
    ((This)->lpVtbl->MoveAppointmentAsync(This, appointment, destinationCalendar, asyncAction))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_ShowAddAppointmentAsync(This, appointment, selection, operation) \
    ((This)->lpVtbl->ShowAddAppointmentAsync(This, appointment, selection, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_ShowReplaceAppointmentAsync(This, localId, appointment, selection, operation) \
    ((This)->lpVtbl->ShowReplaceAppointmentAsync(This, localId, appointment, selection, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_ShowReplaceAppointmentWithPlacementAndDateAsync(This, localId, appointment, selection, preferredPlacement, instanceStartDate, operation) \
    ((This)->lpVtbl->ShowReplaceAppointmentWithPlacementAndDateAsync(This, localId, appointment, selection, preferredPlacement, instanceStartDate, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_ShowRemoveAppointmentAsync(This, localId, selection, operation) \
    ((This)->lpVtbl->ShowRemoveAppointmentAsync(This, localId, selection, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_ShowRemoveAppointmentWithPlacementAndDateAsync(This, localId, selection, preferredPlacement, instanceStartDate, operation) \
    ((This)->lpVtbl->ShowRemoveAppointmentWithPlacementAndDateAsync(This, localId, selection, preferredPlacement, instanceStartDate, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_ShowAppointmentDetailsAsync(This, localId, asyncAction) \
    ((This)->lpVtbl->ShowAppointmentDetailsAsync(This, localId, asyncAction))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_ShowAppointmentDetailsWithDateAsync(This, localId, instanceStartDate, asyncAction) \
    ((This)->lpVtbl->ShowAppointmentDetailsWithDateAsync(This, localId, instanceStartDate, asyncAction))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_ShowEditNewAppointmentAsync(This, appointment, operation) \
    ((This)->lpVtbl->ShowEditNewAppointmentAsync(This, appointment, operation))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_FindLocalIdsFromRoamingIdAsync(This, roamingId, operation) \
    ((This)->lpVtbl->FindLocalIdsFromRoamingIdAsync(This, roamingId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStore2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStore
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointmentStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStore2[] = L"Windows.ApplicationModel.Appointments.IAppointmentStore2";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_StoreChanged)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CAppointmentStore_Windows__CApplicationModel__CAppointments__CAppointmentStoreChangedEventArgs* pHandler,
        EventRegistrationToken* pToken);
    HRESULT (STDMETHODCALLTYPE* remove_StoreChanged)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* CreateAppointmentCalendarInAccountAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2* This,
        HSTRING name,
        HSTRING userDataAccountId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CAppointments__CAppointmentCalendar** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_add_StoreChanged(This, pHandler, pToken) \
    ((This)->lpVtbl->add_StoreChanged(This, pHandler, pToken))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_remove_StoreChanged(This, token) \
    ((This)->lpVtbl->remove_StoreChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_CreateAppointmentCalendarInAccountAsync(This, name, userDataAccountId, operation) \
    ((This)->lpVtbl->CreateAppointmentCalendarInAccountAsync(This, name, userDataAccountId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStore3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStore3[] = L"Windows.ApplicationModel.Appointments.IAppointmentStore3";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetChangeTracker)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3* This,
        HSTRING identity,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_GetChangeTracker(This, identity, result) \
    ((This)->lpVtbl->GetChangeTracker(This, identity, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStore3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStoreChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStoreChange
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStoreChange[] = L"Windows.ApplicationModel.Appointments.IAppointmentStoreChange";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Appointment)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment** value);
    HRESULT (STDMETHODCALLTYPE* get_ChangeType)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentStoreChangeType* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_get_Appointment(This, value) \
    ((This)->lpVtbl->get_Appointment(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_get_ChangeType(This, value) \
    ((This)->lpVtbl->get_ChangeType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStoreChange2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStoreChange
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Appointments.IAppointmentStoreChange
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStoreChange2[] = L"Windows.ApplicationModel.Appointments.IAppointmentStoreChange2";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppointmentCalendar)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentCalendar** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_get_AppointmentCalendar(This, value) \
    ((This)->lpVtbl->get_AppointmentCalendar(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStoreChangeReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStoreChangeReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStoreChangeReader[] = L"Windows.ApplicationModel.Appointments.IAppointmentStoreChangeReader";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReadBatchAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentStoreChange** result);
    HRESULT (STDMETHODCALLTYPE* AcceptChanges)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader* This);
    HRESULT (STDMETHODCALLTYPE* AcceptChangesThrough)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChange* lastChangeToAccept);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReaderVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_ReadBatchAsync(This, result) \
    ((This)->lpVtbl->ReadBatchAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_AcceptChanges(This) \
    ((This)->lpVtbl->AcceptChanges(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_AcceptChangesThrough(This, lastChangeToAccept) \
    ((This)->lpVtbl->AcceptChangesThrough(This, lastChangeToAccept))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStoreChangeTracker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStoreChangeTracker[] = L"Windows.ApplicationModel.Appointments.IAppointmentStoreChangeTracker";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTrackerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetChangeReader)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeReader** value);
    HRESULT (STDMETHODCALLTYPE* Enable)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker* This);
    HRESULT (STDMETHODCALLTYPE* Reset)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTrackerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTrackerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_GetChangeReader(This, value) \
    ((This)->lpVtbl->GetChangeReader(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_Enable(This) \
    ((This)->lpVtbl->Enable(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_Reset(This) \
    ((This)->lpVtbl->Reset(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStoreChangeTracker2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStoreChangeTracker2[] = L"Windows.ApplicationModel.Appointments.IAppointmentStoreChangeTracker2";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsTracking)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_get_IsTracking(This, value) \
    ((This)->lpVtbl->get_IsTracking(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangeTracker2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStoreChangedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStoreChangedDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStoreChangedDeferral[] = L"Windows.ApplicationModel.Appointments.IAppointmentStoreChangedDeferral";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferralVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStoreChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStoreChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStoreChangedEventArgs[] = L"Windows.ApplicationModel.Appointments.IAppointmentStoreChangedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IAppointmentStoreNotificationTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.AppointmentStoreNotificationTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IAppointmentStoreNotificationTriggerDetails[] = L"Windows.ApplicationModel.Appointments.IAppointmentStoreNotificationTriggerDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentStoreNotificationTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.IFindAppointmentsOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.FindAppointmentsOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_IFindAppointmentsOptions[] = L"Windows.ApplicationModel.Appointments.IFindAppointmentsOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CalendarIds)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_FetchProperties)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_IncludeHidden)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IncludeHidden)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_MaxCount)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_MaxCount)(__x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_get_CalendarIds(This, value) \
    ((This)->lpVtbl->get_CalendarIds(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_get_FetchProperties(This, value) \
    ((This)->lpVtbl->get_FetchProperties(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_get_IncludeHidden(This, value) \
    ((This)->lpVtbl->get_IncludeHidden(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_put_IncludeHidden(This, value) \
    ((This)->lpVtbl->put_IncludeHidden(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_get_MaxCount(This, value) \
    ((This)->lpVtbl->get_MaxCount(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_put_MaxCount(This, value) \
    ((This)->lpVtbl->put_MaxCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CIFindAppointmentsOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.Appointment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointment ** Default Interface **
 *    Windows.ApplicationModel.Appointments.IAppointment2
 *    Windows.ApplicationModel.Appointments.IAppointment3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_Appointment_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_Appointment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_Appointment[] = L"Windows.ApplicationModel.Appointments.Appointment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentCalendar
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentCalendar ** Default Interface **
 *    Windows.ApplicationModel.Appointments.IAppointmentCalendar2
 *    Windows.ApplicationModel.Appointments.IAppointmentCalendar3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentCalendar_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentCalendar_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentCalendar[] = L"Windows.ApplicationModel.Appointments.AppointmentCalendar";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentCalendarSyncManager ** Default Interface **
 *    Windows.ApplicationModel.Appointments.IAppointmentCalendarSyncManager2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentCalendarSyncManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentCalendarSyncManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentCalendarSyncManager[] = L"Windows.ApplicationModel.Appointments.AppointmentCalendarSyncManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentConflictResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentConflictResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentConflictResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentConflictResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentConflictResult[] = L"Windows.ApplicationModel.Appointments.AppointmentConflictResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentException
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentException ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentException_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentException_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentException[] = L"Windows.ApplicationModel.Appointments.AppointmentException";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentInvitee
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentInvitee ** Default Interface **
 *    Windows.ApplicationModel.Appointments.IAppointmentParticipant
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentInvitee_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentInvitee_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentInvitee[] = L"Windows.ApplicationModel.Appointments.AppointmentInvitee";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Appointments.IAppointmentManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Appointments.IAppointmentManagerStatics3 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Appointments.IAppointmentManagerStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentManager[] = L"Windows.ApplicationModel.Appointments.AppointmentManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentManagerForUser ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentManagerForUser_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentManagerForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentManagerForUser[] = L"Windows.ApplicationModel.Appointments.AppointmentManagerForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentOrganizer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentParticipant ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentOrganizer_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentOrganizer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentOrganizer[] = L"Windows.ApplicationModel.Appointments.AppointmentOrganizer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Appointments.IAppointmentPropertiesStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Appointments.IAppointmentPropertiesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentProperties_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentProperties[] = L"Windows.ApplicationModel.Appointments.AppointmentProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentRecurrence
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentRecurrence ** Default Interface **
 *    Windows.ApplicationModel.Appointments.IAppointmentRecurrence2
 *    Windows.ApplicationModel.Appointments.IAppointmentRecurrence3
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentRecurrence_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentRecurrence_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentRecurrence[] = L"Windows.ApplicationModel.Appointments.AppointmentRecurrence";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentStore ** Default Interface **
 *    Windows.ApplicationModel.Appointments.IAppointmentStore2
 *    Windows.ApplicationModel.Appointments.IAppointmentStore3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStore_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentStore[] = L"Windows.ApplicationModel.Appointments.AppointmentStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentStoreChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentStoreChange ** Default Interface **
 *    Windows.ApplicationModel.Appointments.IAppointmentStoreChange2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChange_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChange_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentStoreChange[] = L"Windows.ApplicationModel.Appointments.AppointmentStoreChange";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentStoreChangeReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentStoreChangeReader ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChangeReader_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChangeReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentStoreChangeReader[] = L"Windows.ApplicationModel.Appointments.AppointmentStoreChangeReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentStoreChangeTracker ** Default Interface **
 *    Windows.ApplicationModel.Appointments.IAppointmentStoreChangeTracker2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChangeTracker_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChangeTracker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentStoreChangeTracker[] = L"Windows.ApplicationModel.Appointments.AppointmentStoreChangeTracker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentStoreChangedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentStoreChangedDeferral ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChangedDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChangedDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentStoreChangedDeferral[] = L"Windows.ApplicationModel.Appointments.AppointmentStoreChangedDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentStoreChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentStoreChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentStoreChangedEventArgs[] = L"Windows.ApplicationModel.Appointments.AppointmentStoreChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.AppointmentStoreNotificationTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IAppointmentStoreNotificationTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreNotificationTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_AppointmentStoreNotificationTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_AppointmentStoreNotificationTriggerDetails[] = L"Windows.ApplicationModel.Appointments.AppointmentStoreNotificationTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Appointments.FindAppointmentsOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.IFindAppointmentsOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_FindAppointmentsOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_FindAppointmentsOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_FindAppointmentsOptions[] = L"Windows.ApplicationModel.Appointments.FindAppointmentsOptions";
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
#endif // __windows2Eapplicationmodel2Eappointments_p_h__

#endif // __windows2Eapplicationmodel2Eappointments_h__
