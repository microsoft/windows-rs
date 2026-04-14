
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
#ifndef __windows2Eapplicationmodel2Ecalls_h__
#define __windows2Eapplicationmodel2Ecalls_h__
#ifndef __windows2Eapplicationmodel2Ecalls_p_h__
#define __windows2Eapplicationmodel2Ecalls_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION 0x50000
#endif // defined(WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION)

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
#include "Windows.ApplicationModel.Contacts.h"
#include "Windows.Devices.Enumeration.h"
#include "Windows.System.h"
#include "Windows.UI.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IAcceptedVoipPhoneCallOptions;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions ABI::Windows::ApplicationModel::Calls::IAcceptedVoipPhoneCallOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IAcceptedVoipPhoneCallOptionsFactory;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory ABI::Windows::ApplicationModel::Calls::IAcceptedVoipPhoneCallOptionsFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IAppInitiatedVoipPhoneCallOptions;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions ABI::Windows::ApplicationModel::Calls::IAppInitiatedVoipPhoneCallOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IAppInitiatedVoipPhoneCallOptionsFactory;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory ABI::Windows::ApplicationModel::Calls::IAppInitiatedVoipPhoneCallOptionsFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface ICallAnswerEventArgs;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs ABI::Windows::ApplicationModel::Calls::ICallAnswerEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface ICallAnswerEventArgs2;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2 ABI::Windows::ApplicationModel::Calls::ICallAnswerEventArgs2

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface ICallRejectEventArgs;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs ABI::Windows::ApplicationModel::Calls::ICallRejectEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface ICallStateChangeEventArgs;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs ABI::Windows::ApplicationModel::Calls::ICallStateChangeEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IIncomingVoipPhoneCallOptions;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions ABI::Windows::ApplicationModel::Calls::IIncomingVoipPhoneCallOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IIncomingVoipPhoneCallOptionsFactory;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory ABI::Windows::ApplicationModel::Calls::IIncomingVoipPhoneCallOptionsFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface ILockScreenCallEndCallDeferral;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral ABI::Windows::ApplicationModel::Calls::ILockScreenCallEndCallDeferral

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface ILockScreenCallEndRequestedEventArgs;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs ABI::Windows::ApplicationModel::Calls::ILockScreenCallEndRequestedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface ILockScreenCallUI;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI ABI::Windows::ApplicationModel::Calls::ILockScreenCallUI

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IMuteChangeEventArgs;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs ABI::Windows::ApplicationModel::Calls::IMuteChangeEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IOutgoingVoipPhoneCallOptions;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions ABI::Windows::ApplicationModel::Calls::IOutgoingVoipPhoneCallOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IOutgoingVoipPhoneCallOptionsFactory;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory ABI::Windows::ApplicationModel::Calls::IOutgoingVoipPhoneCallOptionsFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCall;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall ABI::Windows::ApplicationModel::Calls::IPhoneCall

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallBlockingStatics;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics ABI::Windows::ApplicationModel::Calls::IPhoneCallBlockingStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallHistoryEntry;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntry

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallHistoryEntryAddress;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntryAddress

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallHistoryEntryAddressFactory;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntryAddressFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallHistoryEntryQueryOptions;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntryQueryOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallHistoryEntryReader;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntryReader

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallHistoryManagerForUser;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryManagerForUser

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallHistoryManagerStatics;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallHistoryManagerStatics2;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2 ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryManagerStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallHistoryStore;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryStore

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallInfo;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo ABI::Windows::ApplicationModel::Calls::IPhoneCallInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallManagerStatics;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics ABI::Windows::ApplicationModel::Calls::IPhoneCallManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallManagerStatics2;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2 ABI::Windows::ApplicationModel::Calls::IPhoneCallManagerStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallStatics;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics ABI::Windows::ApplicationModel::Calls::IPhoneCallStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallStore;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore ABI::Windows::ApplicationModel::Calls::IPhoneCallStore

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallVideoCapabilities;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities ABI::Windows::ApplicationModel::Calls::IPhoneCallVideoCapabilities

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallVideoCapabilitiesManagerStatics;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics ABI::Windows::ApplicationModel::Calls::IPhoneCallVideoCapabilitiesManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneCallsResult;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult ABI::Windows::ApplicationModel::Calls::IPhoneCallsResult

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneDialOptions;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions ABI::Windows::ApplicationModel::Calls::IPhoneDialOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneLine;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine ABI::Windows::ApplicationModel::Calls::IPhoneLine

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneLine2;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2 ABI::Windows::ApplicationModel::Calls::IPhoneLine2

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneLine3;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3 ABI::Windows::ApplicationModel::Calls::IPhoneLine3

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneLineCellularDetails;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails ABI::Windows::ApplicationModel::Calls::IPhoneLineCellularDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneLineConfiguration;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration ABI::Windows::ApplicationModel::Calls::IPhoneLineConfiguration

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneLineDialResult;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult ABI::Windows::ApplicationModel::Calls::IPhoneLineDialResult

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneLineStatics;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics ABI::Windows::ApplicationModel::Calls::IPhoneLineStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneLineTransportDevice;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice ABI::Windows::ApplicationModel::Calls::IPhoneLineTransportDevice

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneLineTransportDevice2;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2 ABI::Windows::ApplicationModel::Calls::IPhoneLineTransportDevice2

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneLineTransportDeviceStatics;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics ABI::Windows::ApplicationModel::Calls::IPhoneLineTransportDeviceStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneLineWatcher;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher ABI::Windows::ApplicationModel::Calls::IPhoneLineWatcher

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneLineWatcherEventArgs;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs ABI::Windows::ApplicationModel::Calls::IPhoneLineWatcherEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IPhoneVoicemail;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail ABI::Windows::ApplicationModel::Calls::IPhoneVoicemail

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IVoipCallCoordinator;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator ABI::Windows::ApplicationModel::Calls::IVoipCallCoordinator

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IVoipCallCoordinator2;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2 ABI::Windows::ApplicationModel::Calls::IVoipCallCoordinator2

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IVoipCallCoordinator3;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3 ABI::Windows::ApplicationModel::Calls::IVoipCallCoordinator3

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IVoipCallCoordinator4;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4 ABI::Windows::ApplicationModel::Calls::IVoipCallCoordinator4

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IVoipCallCoordinator5;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5 ABI::Windows::ApplicationModel::Calls::IVoipCallCoordinator5

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IVoipCallCoordinatorStatics;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics ABI::Windows::ApplicationModel::Calls::IVoipCallCoordinatorStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IVoipCallCoordinatorStatics2;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2 ABI::Windows::ApplicationModel::Calls::IVoipCallCoordinatorStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IVoipPhoneCall;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IVoipPhoneCall2;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2 ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall2

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IVoipPhoneCall3;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3 ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall3

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                interface IVoipPhoneCall4;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4 ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall4

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_FWD_DEFINED__

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



#ifndef DEF___FIAsyncOperation_1_GUID_USE
#define DEF___FIAsyncOperation_1_GUID_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6607bc41-294b-5975-9c3f-4b49836d0916"))
IAsyncOperation<GUID> : IAsyncOperation_impl<GUID>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Guid>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<GUID> __FIAsyncOperation_1_GUID_t;
#define __FIAsyncOperation_1_GUID ABI::Windows::Foundation::__FIAsyncOperation_1_GUID_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_GUID_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_GUID_USE
#define DEF___FIAsyncOperationCompletedHandler_1_GUID_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5233899b-ba7e-504f-bb83-ceebac62decf"))
IAsyncOperationCompletedHandler<GUID> : IAsyncOperationCompletedHandler_impl<GUID>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Guid>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<GUID> __FIAsyncOperationCompletedHandler_1_GUID_t;
#define __FIAsyncOperationCompletedHandler_1_GUID ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_GUID_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_GUID_USE */



#ifndef DEF___FIAsyncOperation_1_UINT32_USE
#define DEF___FIAsyncOperation_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ef60385f-be78-584b-aaef-7829ada2b0de"))
IAsyncOperation<UINT32> : IAsyncOperation_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<UINT32> __FIAsyncOperation_1_UINT32_t;
#define __FIAsyncOperation_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperation_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_UINT32_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE
#define DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9343b6e7-e3d2-5e4a-ab2d-2bce4919a6a4"))
IAsyncOperationCompletedHandler<UINT32> : IAsyncOperationCompletedHandler_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<UINT32> __FIAsyncOperationCompletedHandler_1_UINT32_t;
#define __FIAsyncOperationCompletedHandler_1_UINT32 ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_UINT32_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_UINT32_USE */


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneCallHistoryEntry;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("785e7cca-90e2-5d03-8f23-b3358d09c951"))
IAsyncOperation<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntry*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntry*, ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntry*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Calls.PhoneCallHistoryEntry>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntry*> __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3234244b-abee-561d-b247-79b832822055"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntry*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntry*, ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntry*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Calls.PhoneCallHistoryEntry>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntry*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneCallHistoryStore;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0d9a97b0-8796-52bf-80da-b1435fe64a26"))
IAsyncOperation<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryStore*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryStore*, ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Calls.PhoneCallHistoryStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryStore*> __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("226a138b-79ea-56d3-adc2-a40db8d8c9b0"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryStore*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryStore*, ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Calls.PhoneCallHistoryStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryStore*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneCallInfo;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5b8546d0-f662-55f1-bd47-bf33a7f63fa9"))
IAsyncOperation<ABI::Windows::ApplicationModel::Calls::PhoneCallInfo*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCallInfo*, ABI::Windows::ApplicationModel::Calls::IPhoneCallInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Calls.PhoneCallInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Calls::PhoneCallInfo*> __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("29a6debd-bfe0-5e5d-958d-e77db3079a79"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Calls::PhoneCallInfo*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCallInfo*, ABI::Windows::ApplicationModel::Calls::IPhoneCallInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Calls.PhoneCallInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Calls::PhoneCallInfo*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneCallOperationStatus : int PhoneCallOperationStatus;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("286df3a1-2f43-5b7a-984f-66b8a43b9876"))
IAsyncOperation<enum ABI::Windows::ApplicationModel::Calls::PhoneCallOperationStatus> : IAsyncOperation_impl<enum ABI::Windows::ApplicationModel::Calls::PhoneCallOperationStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Calls.PhoneCallOperationStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::ApplicationModel::Calls::PhoneCallOperationStatus> __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6e80ae96-2731-51f9-8375-53e6c5e2ad8a"))
IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::Calls::PhoneCallOperationStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::ApplicationModel::Calls::PhoneCallOperationStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Calls.PhoneCallOperationStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::Calls::PhoneCallOperationStatus> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneCallStore;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("871cad28-01e8-53b5-a14b-30316df65907"))
IAsyncOperation<ABI::Windows::ApplicationModel::Calls::PhoneCallStore*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCallStore*, ABI::Windows::ApplicationModel::Calls::IPhoneCallStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Calls.PhoneCallStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Calls::PhoneCallStore*> __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("27b63bb3-d008-58f5-854d-ddae65a020b9"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Calls::PhoneCallStore*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCallStore*, ABI::Windows::ApplicationModel::Calls::IPhoneCallStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Calls.PhoneCallStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Calls::PhoneCallStore*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneCallVideoCapabilities;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7b4b280a-e312-5f06-b953-7e482b67cfcf"))
IAsyncOperation<ABI::Windows::ApplicationModel::Calls::PhoneCallVideoCapabilities*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCallVideoCapabilities*, ABI::Windows::ApplicationModel::Calls::IPhoneCallVideoCapabilities*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Calls::PhoneCallVideoCapabilities*> __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e50fc826-3ef3-5669-aa14-eb95903793a5"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Calls::PhoneCallVideoCapabilities*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCallVideoCapabilities*, ABI::Windows::ApplicationModel::Calls::IPhoneCallVideoCapabilities*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Calls::PhoneCallVideoCapabilities*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneCallsResult;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ca0b576f-ee68-5b56-92e0-e4ee9b563462"))
IAsyncOperation<ABI::Windows::ApplicationModel::Calls::PhoneCallsResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCallsResult*, ABI::Windows::ApplicationModel::Calls::IPhoneCallsResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Calls.PhoneCallsResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Calls::PhoneCallsResult*> __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e024b1ab-29e6-569a-a26e-d1ca9474271a"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Calls::PhoneCallsResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCallsResult*, ABI::Windows::ApplicationModel::Calls::IPhoneCallsResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Calls.PhoneCallsResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Calls::PhoneCallsResult*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneLine;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d8712730-aa68-5614-a408-b2012463120b"))
IAsyncOperation<ABI::Windows::ApplicationModel::Calls::PhoneLine*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneLine*, ABI::Windows::ApplicationModel::Calls::IPhoneLine*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Calls.PhoneLine>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Calls::PhoneLine*> __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("92ce5bfd-1417-55ee-b0b6-298ae78cb179"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Calls::PhoneLine*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneLine*, ABI::Windows::ApplicationModel::Calls::IPhoneLine*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Calls.PhoneLine>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Calls::PhoneLine*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneLineDialResult;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("95a1388c-ca33-5530-bcc0-3fc16394f548"))
IAsyncOperation<ABI::Windows::ApplicationModel::Calls::PhoneLineDialResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneLineDialResult*, ABI::Windows::ApplicationModel::Calls::IPhoneLineDialResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Calls.PhoneLineDialResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Calls::PhoneLineDialResult*> __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7a3ba455-27c8-5dad-b8fd-3fe9d8746a15"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Calls::PhoneLineDialResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneLineDialResult*, ABI::Windows::ApplicationModel::Calls::IPhoneLineDialResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Calls.PhoneLineDialResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Calls::PhoneLineDialResult*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum VoipPhoneCallResourceReservationStatus : int VoipPhoneCallResourceReservationStatus;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8528be80-7ce9-5668-8e48-469ae5ba9ead"))
IAsyncOperation<enum ABI::Windows::ApplicationModel::Calls::VoipPhoneCallResourceReservationStatus> : IAsyncOperation_impl<enum ABI::Windows::ApplicationModel::Calls::VoipPhoneCallResourceReservationStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Calls.VoipPhoneCallResourceReservationStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::ApplicationModel::Calls::VoipPhoneCallResourceReservationStatus> __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7a27b20f-647a-53fc-80f0-a79d083ce531"))
IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::Calls::VoipPhoneCallResourceReservationStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::ApplicationModel::Calls::VoipPhoneCallResourceReservationStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Calls.VoipPhoneCallResourceReservationStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::Calls::VoipPhoneCallResourceReservationStatus> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Enumeration {
                typedef enum DeviceAccessStatus : int DeviceAccessStatus;
            } /* Enumeration */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c00bc2f2-a7f8-5f3f-80d1-2808ef6bca10"))
IAsyncOperation<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus> : IAsyncOperation_impl<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Enumeration.DeviceAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus> __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_t;
#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ee154d83-805b-53e8-8469-90715036d013"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Enumeration.DeviceAccessStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Devices::Enumeration::DeviceAccessStatus> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c1cf3870-064a-54d5-afab-d1dc4ee26ccb"))
IIterator<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntry*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntry*, ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntry*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Calls.PhoneCallHistoryEntry>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntry*> __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_t;
#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a3f93eea-c846-52c7-aa5a-3306707f6369"))
IIterable<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntry*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntry*, ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntry*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Calls.PhoneCallHistoryEntry>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntry*> __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_t;
#define __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("452ce6ed-a06d-58fb-be06-cb4330b7f5c7"))
IVectorView<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntry*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntry*, ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntry*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Calls.PhoneCallHistoryEntry>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntry*> __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_t;
#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2258b912-eb70-5361-b20a-731e15bb9097"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Calls.PhoneCallHistoryEntry>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1ef6a805-fd84-5756-a180-353dd72db275"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Calls.PhoneCallHistoryEntry>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_USE */

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


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneCall;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("24587ec4-36b5-50e6-9b18-b69b1550c84a"))
IIterator<ABI::Windows::ApplicationModel::Calls::PhoneCall*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCall*, ABI::Windows::ApplicationModel::Calls::IPhoneCall*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Calls.PhoneCall>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Calls::PhoneCall*> __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_t;
#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c43448ad-3660-5f7f-9270-cc340ebc32c2"))
IIterable<ABI::Windows::ApplicationModel::Calls::PhoneCall*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCall*, ABI::Windows::ApplicationModel::Calls::IPhoneCall*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Calls.PhoneCall>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Calls::PhoneCall*> __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall_t;
#define __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000


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


#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2c1d6d88-1ba7-5459-9d9c-eb835c55cc4c"))
IVectorView<ABI::Windows::ApplicationModel::Calls::PhoneCall*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCall*, ABI::Windows::ApplicationModel::Calls::IPhoneCall*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Calls.PhoneCall>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Calls::PhoneCall*> __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_t;
#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000


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



#ifndef DEF___FIEventHandler_1_IInspectable_USE
#define DEF___FIEventHandler_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c50898f6-c536-5f47-8583-8b2c2438a13b"))
IEventHandler<IInspectable*> : IEventHandler_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.EventHandler`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IEventHandler<IInspectable*> __FIEventHandler_1_IInspectable_t;
#define __FIEventHandler_1_IInspectable ABI::Windows::Foundation::__FIEventHandler_1_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIEventHandler_1_IInspectable_USE */


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
            namespace Calls {
                class LockScreenCallUI;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("addada2a-e5a7-5921-b7e0-17323adf7382"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::LockScreenCallUI*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::LockScreenCallUI*, ABI::Windows::ApplicationModel::Calls::ILockScreenCallUI*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Calls.LockScreenCallUI, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::LockScreenCallUI*, IInspectable*> __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class LockScreenCallEndRequestedEventArgs;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("92f7c40e-e7b9-5f68-98f0-56fb89015806"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::LockScreenCallUI*, ABI::Windows::ApplicationModel::Calls::LockScreenCallEndRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::LockScreenCallUI*, ABI::Windows::ApplicationModel::Calls::ILockScreenCallUI*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::LockScreenCallEndRequestedEventArgs*, ABI::Windows::ApplicationModel::Calls::ILockScreenCallEndRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Calls.LockScreenCallUI, Windows.ApplicationModel.Calls.LockScreenCallEndRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::LockScreenCallUI*, ABI::Windows::ApplicationModel::Calls::LockScreenCallEndRequestedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6dde6f30-654a-5fae-994d-8200049bd6da"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::PhoneCall*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneCall*, ABI::Windows::ApplicationModel::Calls::IPhoneCall*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Calls.PhoneCall, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::PhoneCall*, IInspectable*> __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7c5f5192-9fc0-5543-9bc4-411482e4ea93"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::PhoneLine*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneLine*, ABI::Windows::ApplicationModel::Calls::IPhoneLine*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Calls.PhoneLine, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::PhoneLine*, IInspectable*> __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneLineTransportDevice;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b20415a7-ef40-50db-9340-ef10915d76f9"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::PhoneLineTransportDevice*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneLineTransportDevice*, ABI::Windows::ApplicationModel::Calls::IPhoneLineTransportDevice*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Calls.PhoneLineTransportDevice, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::PhoneLineTransportDevice*, IInspectable*> __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneLineWatcher;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d55ce56a-23ac-5185-bf76-2808ec83c78b"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::PhoneLineWatcher*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneLineWatcher*, ABI::Windows::ApplicationModel::Calls::IPhoneLineWatcher*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Calls.PhoneLineWatcher, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::PhoneLineWatcher*, IInspectable*> __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneLineWatcherEventArgs;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("727cff26-a887-5361-8924-95f7bab4e25d"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::PhoneLineWatcher*, ABI::Windows::ApplicationModel::Calls::PhoneLineWatcherEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneLineWatcher*, ABI::Windows::ApplicationModel::Calls::IPhoneLineWatcher*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::PhoneLineWatcherEventArgs*, ABI::Windows::ApplicationModel::Calls::IPhoneLineWatcherEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Calls.PhoneLineWatcher, Windows.ApplicationModel.Calls.PhoneLineWatcherEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::PhoneLineWatcher*, ABI::Windows::ApplicationModel::Calls::PhoneLineWatcherEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class VoipCallCoordinator;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class MuteChangeEventArgs;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ecafec77-4bf6-57b7-86c6-e2feca5b5aee"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::VoipCallCoordinator*, ABI::Windows::ApplicationModel::Calls::MuteChangeEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::VoipCallCoordinator*, ABI::Windows::ApplicationModel::Calls::IVoipCallCoordinator*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::MuteChangeEventArgs*, ABI::Windows::ApplicationModel::Calls::IMuteChangeEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Calls.VoipCallCoordinator, Windows.ApplicationModel.Calls.MuteChangeEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::VoipCallCoordinator*, ABI::Windows::ApplicationModel::Calls::MuteChangeEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class VoipPhoneCall;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class CallAnswerEventArgs;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d47be4da-c00c-5faa-bfa5-1b11e0c3ccc1"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::VoipPhoneCall*, ABI::Windows::ApplicationModel::Calls::CallAnswerEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::VoipPhoneCall*, ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::CallAnswerEventArgs*, ABI::Windows::ApplicationModel::Calls::ICallAnswerEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Calls.VoipPhoneCall, Windows.ApplicationModel.Calls.CallAnswerEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::VoipPhoneCall*, ABI::Windows::ApplicationModel::Calls::CallAnswerEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class CallRejectEventArgs;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d06255ce-0967-5441-8fe6-ed2e7008197e"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::VoipPhoneCall*, ABI::Windows::ApplicationModel::Calls::CallRejectEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::VoipPhoneCall*, ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::CallRejectEventArgs*, ABI::Windows::ApplicationModel::Calls::ICallRejectEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Calls.VoipPhoneCall, Windows.ApplicationModel.Calls.CallRejectEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::VoipPhoneCall*, ABI::Windows::ApplicationModel::Calls::CallRejectEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class CallStateChangeEventArgs;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1e00c6cc-e14c-51ce-93f3-0a0a9a3f3eec"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::VoipPhoneCall*, ABI::Windows::ApplicationModel::Calls::CallStateChangeEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::VoipPhoneCall*, ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Calls::CallStateChangeEventArgs*, ABI::Windows::ApplicationModel::Calls::ICallStateChangeEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Calls.VoipPhoneCall, Windows.ApplicationModel.Calls.CallStateChangeEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Calls::VoipPhoneCall*, ABI::Windows::ApplicationModel::Calls::CallStateChangeEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs_USE */

#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

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

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                class ContactPhone;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactPhone_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactPhone_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                interface IContactPhone;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CIContactPhone ABI::Windows::ApplicationModel::Contacts::IContactPhone

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactPhone_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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
        namespace ApplicationModel {
            namespace Calls {
                typedef enum DtmfKey : int DtmfKey;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum DtmfToneAudioPlayback : int DtmfToneAudioPlayback;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneAudioRoutingEndpoint : int PhoneAudioRoutingEndpoint;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneCallAudioDevice : int PhoneCallAudioDevice;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneCallDirection : int PhoneCallDirection;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneCallHistoryEntryMedia : int PhoneCallHistoryEntryMedia;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneCallHistoryEntryOtherAppReadAccess : int PhoneCallHistoryEntryOtherAppReadAccess;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneCallHistoryEntryQueryDesiredMedia : unsigned int PhoneCallHistoryEntryQueryDesiredMedia;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneCallHistoryEntryRawAddressKind : int PhoneCallHistoryEntryRawAddressKind;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneCallHistorySourceIdKind : int PhoneCallHistorySourceIdKind;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneCallHistoryStoreAccessType : int PhoneCallHistoryStoreAccessType;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneCallMedia : int PhoneCallMedia;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneCallStatus : int PhoneCallStatus;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneLineNetworkOperatorDisplayTextLocation : int PhoneLineNetworkOperatorDisplayTextLocation;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneLineOperationStatus : int PhoneLineOperationStatus;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneLineTransport : int PhoneLineTransport;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneLineWatcherStatus : int PhoneLineWatcherStatus;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneNetworkState : int PhoneNetworkState;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneSimState : int PhoneSimState;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum PhoneVoicemailType : int PhoneVoicemailType;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum TransportDeviceAudioRoutingStatus : int TransportDeviceAudioRoutingStatus;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum VoipCallControlDeviceKind : int VoipCallControlDeviceKind;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum VoipPhoneCallMedia : unsigned int VoipPhoneCallMedia;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum VoipPhoneCallRejectReason : int VoipPhoneCallRejectReason;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                typedef enum VoipPhoneCallState : int VoipPhoneCallState;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class AcceptedVoipPhoneCallOptions;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class AppInitiatedVoipPhoneCallOptions;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class IncomingVoipPhoneCallOptions;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class LockScreenCallEndCallDeferral;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class OutgoingVoipPhoneCallOptions;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneCallHistoryEntryAddress;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneCallHistoryEntryQueryOptions;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneCallHistoryEntryReader;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneCallHistoryManagerForUser;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneDialOptions;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneLineCellularDetails;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneLineConfiguration;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                class PhoneVoicemail;
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Calls.CellularDtmfMode
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum CellularDtmfMode : int
                {
                    CellularDtmfMode_Continuous = 0,
                    CellularDtmfMode_Burst = 1,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.DtmfKey
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum DtmfKey : int
                {
                    DtmfKey_D0 = 0,
                    DtmfKey_D1 = 1,
                    DtmfKey_D2 = 2,
                    DtmfKey_D3 = 3,
                    DtmfKey_D4 = 4,
                    DtmfKey_D5 = 5,
                    DtmfKey_D6 = 6,
                    DtmfKey_D7 = 7,
                    DtmfKey_D8 = 8,
                    DtmfKey_D9 = 9,
                    DtmfKey_Star = 10,
                    DtmfKey_Pound = 11,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.Calls.DtmfToneAudioPlayback
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum DtmfToneAudioPlayback : int
                {
                    DtmfToneAudioPlayback_Play = 0,
                    DtmfToneAudioPlayback_DoNotPlay = 1,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneAudioRoutingEndpoint
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneAudioRoutingEndpoint : int
                {
                    PhoneAudioRoutingEndpoint_Default = 0,
                    PhoneAudioRoutingEndpoint_Bluetooth = 1,
                    PhoneAudioRoutingEndpoint_Speakerphone = 2,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallAudioDevice
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneCallAudioDevice : int
                {
                    PhoneCallAudioDevice_Unknown = 0,
                    PhoneCallAudioDevice_LocalDevice = 1,
                    PhoneCallAudioDevice_RemoteDevice = 2,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallDirection
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneCallDirection : int
                {
                    PhoneCallDirection_Unknown = 0,
                    PhoneCallDirection_Incoming = 1,
                    PhoneCallDirection_Outgoing = 2,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallHistoryEntryMedia
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneCallHistoryEntryMedia : int
                {
                    PhoneCallHistoryEntryMedia_Audio = 0,
                    PhoneCallHistoryEntryMedia_Video = 1,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallHistoryEntryOtherAppReadAccess
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneCallHistoryEntryOtherAppReadAccess : int
                {
                    PhoneCallHistoryEntryOtherAppReadAccess_Full = 0,
                    PhoneCallHistoryEntryOtherAppReadAccess_SystemOnly = 1,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryDesiredMedia
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneCallHistoryEntryQueryDesiredMedia : unsigned int
                {
                    PhoneCallHistoryEntryQueryDesiredMedia_None = 0,
                    PhoneCallHistoryEntryQueryDesiredMedia_Audio = 0x1,
                    PhoneCallHistoryEntryQueryDesiredMedia_Video = 0x2,
                    PhoneCallHistoryEntryQueryDesiredMedia_All = 0xffffffff,
                };

                DEFINE_ENUM_FLAG_OPERATORS(PhoneCallHistoryEntryQueryDesiredMedia)
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallHistoryEntryRawAddressKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneCallHistoryEntryRawAddressKind : int
                {
                    PhoneCallHistoryEntryRawAddressKind_PhoneNumber = 0,
                    PhoneCallHistoryEntryRawAddressKind_Custom = 1,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallHistorySourceIdKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneCallHistorySourceIdKind : int
                {
                    PhoneCallHistorySourceIdKind_CellularPhoneLineId = 0,
                    PhoneCallHistorySourceIdKind_PackageFamilyName = 1,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallHistoryStoreAccessType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneCallHistoryStoreAccessType : int
                {
                    PhoneCallHistoryStoreAccessType_AppEntriesReadWrite = 0,
                    PhoneCallHistoryStoreAccessType_AllEntriesLimitedReadWrite = 1,
                    PhoneCallHistoryStoreAccessType_AllEntriesReadWrite = 2,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallMedia
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneCallMedia : int
                {
                    PhoneCallMedia_Audio = 0,
                    PhoneCallMedia_AudioAndVideo = 1,
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x40000
                    PhoneCallMedia_AudioAndRealTimeText = 2,
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x40000
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallOperationStatus
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneCallOperationStatus : int
                {
                    PhoneCallOperationStatus_Succeeded = 0,
                    PhoneCallOperationStatus_OtherFailure = 1,
                    PhoneCallOperationStatus_TimedOut = 2,
                    PhoneCallOperationStatus_ConnectionLost = 3,
                    PhoneCallOperationStatus_InvalidCallState = 4,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallStatus
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneCallStatus : int
                {
                    PhoneCallStatus_Lost = 0,
                    PhoneCallStatus_Incoming = 1,
                    PhoneCallStatus_Dialing = 2,
                    PhoneCallStatus_Talking = 3,
                    PhoneCallStatus_Held = 4,
                    PhoneCallStatus_Ended = 5,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneLineNetworkOperatorDisplayTextLocation
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneLineNetworkOperatorDisplayTextLocation : int
                {
                    PhoneLineNetworkOperatorDisplayTextLocation_Default = 0,
                    PhoneLineNetworkOperatorDisplayTextLocation_Tile = 1,
                    PhoneLineNetworkOperatorDisplayTextLocation_Dialer = 2,
                    PhoneLineNetworkOperatorDisplayTextLocation_InCallUI = 3,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneLineOperationStatus
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneLineOperationStatus : int
                {
                    PhoneLineOperationStatus_Succeeded = 0,
                    PhoneLineOperationStatus_OtherFailure = 1,
                    PhoneLineOperationStatus_TimedOut = 2,
                    PhoneLineOperationStatus_ConnectionLost = 3,
                    PhoneLineOperationStatus_InvalidCallState = 4,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneLineTransport
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneLineTransport : int
                {
                    PhoneLineTransport_Cellular = 0,
                    PhoneLineTransport_VoipApp = 1,
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000
                    PhoneLineTransport_Bluetooth = 2,
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneLineWatcherStatus
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneLineWatcherStatus : int
                {
                    PhoneLineWatcherStatus_Created = 0,
                    PhoneLineWatcherStatus_Started = 1,
                    PhoneLineWatcherStatus_EnumerationCompleted = 2,
                    PhoneLineWatcherStatus_Stopped = 3,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneNetworkState
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneNetworkState : int
                {
                    PhoneNetworkState_Unknown = 0,
                    PhoneNetworkState_NoSignal = 1,
                    PhoneNetworkState_Deregistered = 2,
                    PhoneNetworkState_Denied = 3,
                    PhoneNetworkState_Searching = 4,
                    PhoneNetworkState_Home = 5,
                    PhoneNetworkState_RoamingInternational = 6,
                    PhoneNetworkState_RoamingDomestic = 7,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneSimState
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneSimState : int
                {
                    PhoneSimState_Unknown = 0,
                    PhoneSimState_PinNotRequired = 1,
                    PhoneSimState_PinUnlocked = 2,
                    PhoneSimState_PinLocked = 3,
                    PhoneSimState_PukLocked = 4,
                    PhoneSimState_NotInserted = 5,
                    PhoneSimState_Invalid = 6,
                    PhoneSimState_Disabled = 7,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneVoicemailType
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum PhoneVoicemailType : int
                {
                    PhoneVoicemailType_None = 0,
                    PhoneVoicemailType_Traditional = 1,
                    PhoneVoicemailType_Visual = 2,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.TransportDeviceAudioRoutingStatus
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum TransportDeviceAudioRoutingStatus : int
                {
                    TransportDeviceAudioRoutingStatus_Unknown = 0,
                    TransportDeviceAudioRoutingStatus_CanRouteToLocalDevice = 1,
                    TransportDeviceAudioRoutingStatus_CannotRouteToLocalDevice = 2,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.Calls.VoipCallControlDeviceKind
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum VoipCallControlDeviceKind : int
                {
                    VoipCallControlDeviceKind_Bluetooth = 0,
                    VoipCallControlDeviceKind_Usb = 1,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.ApplicationModel.Calls.VoipPhoneCallMedia
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum VoipPhoneCallMedia : unsigned int
                {
                    VoipPhoneCallMedia_None = 0,
                    VoipPhoneCallMedia_Audio = 0x1,
                    VoipPhoneCallMedia_Video = 0x2,
                };

                DEFINE_ENUM_FLAG_OPERATORS(VoipPhoneCallMedia)
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.VoipPhoneCallRejectReason
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum VoipPhoneCallRejectReason : int
                {
                    VoipPhoneCallRejectReason_UserIgnored = 0,
                    VoipPhoneCallRejectReason_TimedOut = 1,
                    VoipPhoneCallRejectReason_OtherIncomingCall = 2,
                    VoipPhoneCallRejectReason_EmergencyCallExists = 3,
                    VoipPhoneCallRejectReason_InvalidCallState = 4,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.VoipPhoneCallResourceReservationStatus
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum VoipPhoneCallResourceReservationStatus : int
                {
                    VoipPhoneCallResourceReservationStatus_Success = 0,
                    VoipPhoneCallResourceReservationStatus_ResourcesNotAvailable = 1,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.VoipPhoneCallState
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                enum VoipPhoneCallState : int
                {
                    VoipPhoneCallState_Ended = 0,
                    VoipPhoneCallState_Held = 1,
                    VoipPhoneCallState_Active = 2,
                    VoipPhoneCallState_Incoming = 3,
                    VoipPhoneCallState_Outgoing = 4,
                };
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IAcceptedVoipPhoneCallOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.AcceptedVoipPhoneCallOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IAcceptedVoipPhoneCallOptions[] = L"Windows.ApplicationModel.Calls.IAcceptedVoipPhoneCallOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("e519c726-b86f-5add-8ae2-0f46acd9232d")
                IAcceptedVoipPhoneCallOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Context(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Context(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContactName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ContactName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContactNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ContactNumber(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ServiceName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Media(
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Media(
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AssociatedDeviceIds(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAcceptedVoipPhoneCallOptions = __uuidof(IAcceptedVoipPhoneCallOptions);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IAcceptedVoipPhoneCallOptionsFactory
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.AcceptedVoipPhoneCallOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IAcceptedVoipPhoneCallOptionsFactory[] = L"Windows.ApplicationModel.Calls.IAcceptedVoipPhoneCallOptionsFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("6cf8a79b-acc1-54ce-a75d-cc78d17690c8")
                IAcceptedVoipPhoneCallOptionsFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        __FIIterable_1_HSTRING* associatedDeviceIds,
                        ABI::Windows::ApplicationModel::Calls::IAcceptedVoipPhoneCallOptions** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAcceptedVoipPhoneCallOptionsFactory = __uuidof(IAcceptedVoipPhoneCallOptionsFactory);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IAppInitiatedVoipPhoneCallOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.AppInitiatedVoipPhoneCallOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IAppInitiatedVoipPhoneCallOptions[] = L"Windows.ApplicationModel.Calls.IAppInitiatedVoipPhoneCallOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("86bebf63-ff5a-57fd-84c6-2d2cf18302f8")
                IAppInitiatedVoipPhoneCallOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Context(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Context(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContactName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ContactName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContactNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ContactNumber(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ServiceName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Media(
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Media(
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AssociatedDeviceIds(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppInitiatedVoipPhoneCallOptions = __uuidof(IAppInitiatedVoipPhoneCallOptions);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IAppInitiatedVoipPhoneCallOptionsFactory
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.AppInitiatedVoipPhoneCallOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IAppInitiatedVoipPhoneCallOptionsFactory[] = L"Windows.ApplicationModel.Calls.IAppInitiatedVoipPhoneCallOptionsFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("ca46c30c-f779-5f3b-8ebc-a635e7f652b5")
                IAppInitiatedVoipPhoneCallOptionsFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        __FIIterable_1_HSTRING* associatedDeviceIds,
                        ABI::Windows::ApplicationModel::Calls::IAppInitiatedVoipPhoneCallOptions** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAppInitiatedVoipPhoneCallOptionsFactory = __uuidof(IAppInitiatedVoipPhoneCallOptionsFactory);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.ICallAnswerEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.CallAnswerEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_ICallAnswerEventArgs[] = L"Windows.ApplicationModel.Calls.ICallAnswerEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("fd789617-2dd7-4c8c-b2bd-95d17a5bb733")
                ICallAnswerEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AcceptedMedia(
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICallAnswerEventArgs = __uuidof(ICallAnswerEventArgs);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.ICallAnswerEventArgs2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.CallAnswerEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_ICallAnswerEventArgs2[] = L"Windows.ApplicationModel.Calls.ICallAnswerEventArgs2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("408208f7-c3f7-579a-800d-541082cba051")
                ICallAnswerEventArgs2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SourceDeviceId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICallAnswerEventArgs2 = __uuidof(ICallAnswerEventArgs2);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.ICallRejectEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.CallRejectEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_ICallRejectEventArgs[] = L"Windows.ApplicationModel.Calls.ICallRejectEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("da47fad7-13d4-4d92-a1c2-b77811ee37ec")
                ICallRejectEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RejectReason(
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallRejectReason* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICallRejectEventArgs = __uuidof(ICallRejectEventArgs);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.ICallStateChangeEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.CallStateChangeEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_ICallStateChangeEventArgs[] = L"Windows.ApplicationModel.Calls.ICallStateChangeEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("eab2349e-66f5-47f9-9fb5-459c5198c720")
                ICallStateChangeEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallState* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICallStateChangeEventArgs = __uuidof(ICallStateChangeEventArgs);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IIncomingVoipPhoneCallOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.IncomingVoipPhoneCallOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IIncomingVoipPhoneCallOptions[] = L"Windows.ApplicationModel.Calls.IIncomingVoipPhoneCallOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("4379fcd6-ddd0-5e9b-81d8-5110495764ae")
                IIncomingVoipPhoneCallOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Context(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Context(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContactName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ContactName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContactNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ContactNumber(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContactImage(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ContactImage(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ServiceName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BrandingImage(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BrandingImage(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CallDetails(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CallDetails(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Ringtone(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Ringtone(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Media(
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Media(
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RingTimeout(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RingTimeout(
                        ABI::Windows::Foundation::TimeSpan value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContactRemoteId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ContactRemoteId(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AssociatedDeviceIds(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIncomingVoipPhoneCallOptions = __uuidof(IIncomingVoipPhoneCallOptions);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IIncomingVoipPhoneCallOptionsFactory
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.IncomingVoipPhoneCallOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IIncomingVoipPhoneCallOptionsFactory[] = L"Windows.ApplicationModel.Calls.IIncomingVoipPhoneCallOptionsFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("74062de4-08f0-5649-bd80-89ea87185c78")
                IIncomingVoipPhoneCallOptionsFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        __FIIterable_1_HSTRING* associatedDeviceIds,
                        ABI::Windows::ApplicationModel::Calls::IIncomingVoipPhoneCallOptions** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIncomingVoipPhoneCallOptionsFactory = __uuidof(IIncomingVoipPhoneCallOptionsFactory);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.ILockScreenCallEndCallDeferral
 *
 * Introduced to Windows.ApplicationModel.Calls.LockScreenCallContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.LockScreenCallEndCallDeferral
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_ILockScreenCallEndCallDeferral[] = L"Windows.ApplicationModel.Calls.ILockScreenCallEndCallDeferral";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("2dd7ed0d-98ed-4041-9632-50ff812b773f")
                ILockScreenCallEndCallDeferral : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ILockScreenCallEndCallDeferral = __uuidof(ILockScreenCallEndCallDeferral);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.ILockScreenCallEndRequestedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.LockScreenCallContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.LockScreenCallEndRequestedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_ILockScreenCallEndRequestedEventArgs[] = L"Windows.ApplicationModel.Calls.ILockScreenCallEndRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("8190a363-6f27-46e9-aeb6-c0ae83e47dc7")
                ILockScreenCallEndRequestedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::ApplicationModel::Calls::ILockScreenCallEndCallDeferral** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Deadline(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILockScreenCallEndRequestedEventArgs = __uuidof(ILockScreenCallEndRequestedEventArgs);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.ILockScreenCallUI
 *
 * Introduced to Windows.ApplicationModel.Calls.LockScreenCallContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.LockScreenCallUI
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_ILockScreenCallUI[] = L"Windows.ApplicationModel.Calls.ILockScreenCallUI";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("c596fd8d-73c9-4a14-b021-ec1c50a3b727")
                ILockScreenCallUI : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Dismiss(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_EndRequested(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_EndRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Closed(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Closed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CallTitle(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CallTitle(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILockScreenCallUI = __uuidof(ILockScreenCallUI);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IMuteChangeEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.MuteChangeEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IMuteChangeEventArgs[] = L"Windows.ApplicationModel.Calls.IMuteChangeEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("8585e159-0c41-432c-814d-c5f1fdf530be")
                IMuteChangeEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Muted(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMuteChangeEventArgs = __uuidof(IMuteChangeEventArgs);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IOutgoingVoipPhoneCallOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.OutgoingVoipPhoneCallOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IOutgoingVoipPhoneCallOptions[] = L"Windows.ApplicationModel.Calls.IOutgoingVoipPhoneCallOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("d6c59b57-57be-524f-9dc1-f2c12e5d1bcc")
                IOutgoingVoipPhoneCallOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Context(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Context(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContactName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ContactName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ServiceName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Media(
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Media(
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AssociatedDeviceIds(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IOutgoingVoipPhoneCallOptions = __uuidof(IOutgoingVoipPhoneCallOptions);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IOutgoingVoipPhoneCallOptionsFactory
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.OutgoingVoipPhoneCallOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IOutgoingVoipPhoneCallOptionsFactory[] = L"Windows.ApplicationModel.Calls.IOutgoingVoipPhoneCallOptionsFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("2ea2c6f4-0b7a-5789-9d33-fe3271fdefa8")
                IOutgoingVoipPhoneCallOptionsFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        __FIIterable_1_HSTRING* associatedDeviceIds,
                        ABI::Windows::ApplicationModel::Calls::IOutgoingVoipPhoneCallOptions** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IOutgoingVoipPhoneCallOptionsFactory = __uuidof(IOutgoingVoipPhoneCallOptionsFactory);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCall
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCall
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCall[] = L"Windows.ApplicationModel.Calls.IPhoneCall";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("c14ed0f8-c17d-59d2-9628-66e545b6cd21")
                IPhoneCall : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_StatusChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StatusChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AudioDeviceChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AudioDeviceChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_IsMutedChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_IsMutedChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CallId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsMuted(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AudioDevice(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallAudioDevice* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPhoneCallInfo(
                        ABI::Windows::ApplicationModel::Calls::IPhoneCallInfo** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPhoneCallInfoAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE End(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallOperationStatus* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EndAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendDtmfKey(
                        ABI::Windows::ApplicationModel::Calls::DtmfKey key,
                        ABI::Windows::ApplicationModel::Calls::DtmfToneAudioPlayback dtmfToneAudioPlayback,
                        ABI::Windows::ApplicationModel::Calls::PhoneCallOperationStatus* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendDtmfKeyAsync(
                        ABI::Windows::ApplicationModel::Calls::DtmfKey key,
                        ABI::Windows::ApplicationModel::Calls::DtmfToneAudioPlayback dtmfToneAudioPlayback,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AcceptIncoming(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallOperationStatus* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AcceptIncomingAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Hold(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallOperationStatus* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE HoldAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ResumeFromHold(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallOperationStatus* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ResumeFromHoldAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Mute(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallOperationStatus* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE MuteAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Unmute(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallOperationStatus* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UnmuteAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RejectIncoming(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallOperationStatus* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RejectIncomingAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ChangeAudioDevice(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallAudioDevice endpoint,
                        ABI::Windows::ApplicationModel::Calls::PhoneCallOperationStatus* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ChangeAudioDeviceAsync(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallAudioDevice endpoint,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCall = __uuidof(IPhoneCall);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallBlockingStatics
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallBlocking
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallBlockingStatics[] = L"Windows.ApplicationModel.Calls.IPhoneCallBlockingStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("19646f84-2b79-26f1-a46f-694be043f313")
                IPhoneCallBlockingStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_BlockUnknownNumbers(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BlockUnknownNumbers(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BlockPrivateNumbers(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BlockPrivateNumbers(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetCallBlockingListAsync(
                        __FIIterable_1_HSTRING* phoneNumberList,
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallBlockingStatics = __uuidof(IPhoneCallBlockingStatics);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryEntry
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryEntry
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryEntry[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryEntry";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("fab0e129-32a4-4b85-83d1-f90d8c23a857")
                IPhoneCallHistoryEntry : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Address(
                        ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntryAddress** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Address(
                        ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntryAddress* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Duration(
                        __FIReference_1_Windows__CFoundation__CTimeSpan** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Duration(
                        __FIReference_1_Windows__CFoundation__CTimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsCallerIdBlocked(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsCallerIdBlocked(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsEmergency(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsEmergency(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsIncoming(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsIncoming(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsMissed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsMissed(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsRinging(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsRinging(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSeen(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsSeen(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSuppressed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsSuppressed(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsVoicemail(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsVoicemail(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Media(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntryMedia* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Media(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntryMedia value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OtherAppReadAccess(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntryOtherAppReadAccess* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OtherAppReadAccess(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntryOtherAppReadAccess value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RemoteId(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SourceDisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SourceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SourceId(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SourceIdKind(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallHistorySourceIdKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SourceIdKind(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallHistorySourceIdKind value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StartTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_StartTime(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallHistoryEntry = __uuidof(IPhoneCallHistoryEntry);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryAddress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryEntryAddress[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryAddress";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("30f159da-3955-4042-84e6-66eebf82e67f")
                IPhoneCallHistoryEntryAddress : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContactId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ContactId(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RawAddress(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RawAddress(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RawAddressKind(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntryRawAddressKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RawAddressKind(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntryRawAddressKind value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallHistoryEntryAddress = __uuidof(IPhoneCallHistoryEntryAddress);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryAddressFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryEntryAddressFactory[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryAddressFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("fb0fadba-c7f0-4bb6-9f6b-ba5d73209aca")
                IPhoneCallHistoryEntryAddressFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        HSTRING rawAddress,
                        ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntryRawAddressKind rawAddressKind,
                        ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntryAddress** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallHistoryEntryAddressFactory = __uuidof(IPhoneCallHistoryEntryAddressFactory);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryQueryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryEntryQueryOptions[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryQueryOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("9c5fe15c-8bed-40ca-b06e-c4ca8eae5c87")
                IPhoneCallHistoryEntryQueryOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DesiredMedia(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntryQueryDesiredMedia* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DesiredMedia(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryEntryQueryDesiredMedia value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SourceIds(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallHistoryEntryQueryOptions = __uuidof(IPhoneCallHistoryEntryQueryOptions);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryEntryReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryEntryReader[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryReader";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("61ece4be-8d86-479f-8404-a9846920fee6")
                IPhoneCallHistoryEntryReader : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReadBatchAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallHistoryEntryReader = __uuidof(IPhoneCallHistoryEntryReader);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryManagerForUser[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerForUser";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("d925c523-f55f-4353-9db4-0205a5265a55")
                IPhoneCallHistoryManagerForUser : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestStoreAsync(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryStoreAccessType accessType,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_User(
                        ABI::Windows::System::IUser** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallHistoryManagerForUser = __uuidof(IPhoneCallHistoryManagerForUser);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryManagerStatics[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("f5a6da39-b31f-4f45-ac8e-1b08893c1b50")
                IPhoneCallHistoryManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestStoreAsync(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallHistoryStoreAccessType accessType,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallHistoryManagerStatics = __uuidof(IPhoneCallHistoryManagerStatics);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryManagerStatics2[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("efd474f0-a2db-4188-9e92-bc3cfa6813cf")
                IPhoneCallHistoryManagerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForUser(
                        ABI::Windows::System::IUser* user,
                        ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryManagerForUser** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallHistoryManagerStatics2 = __uuidof(IPhoneCallHistoryManagerStatics2);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryStore[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryStore";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("2f907db8-b40e-422b-8545-cb1910a61c52")
                IPhoneCallHistoryStore : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetEntryAsync(
                        HSTRING callHistoryEntryId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetEntryReader(
                        ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntryReader** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetEntryReaderWithOptions(
                        ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntryQueryOptions* queryOptions,
                        ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntryReader** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SaveEntryAsync(
                        ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntry* callHistoryEntry,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteEntryAsync(
                        ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntry* callHistoryEntry,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteEntriesAsync(
                        __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* callHistoryEntries,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE MarkEntryAsSeenAsync(
                        ABI::Windows::ApplicationModel::Calls::IPhoneCallHistoryEntry* callHistoryEntry,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE MarkEntriesAsSeenAsync(
                        __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* callHistoryEntries,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetUnseenCountAsync(
                        __FIAsyncOperation_1_UINT32** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE MarkAllAsSeenAsync(
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSourcesUnseenCountAsync(
                        __FIIterable_1_HSTRING* sourceIds,
                        __FIAsyncOperation_1_UINT32** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE MarkSourcesAsSeenAsync(
                        __FIIterable_1_HSTRING* sourceIds,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallHistoryStore = __uuidof(IPhoneCallHistoryStore);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallInfo
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallInfo
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallInfo[] = L"Windows.ApplicationModel.Calls.IPhoneCallInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("22b42577-3e4d-5dc6-89c2-469fe5ffc189")
                IPhoneCallInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LineId(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsHoldSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StartTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PhoneNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CallDirection(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallDirection* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallInfo = __uuidof(IPhoneCallInfo);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallManagerStatics
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallManagerStatics[] = L"Windows.ApplicationModel.Calls.IPhoneCallManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("60edac78-78a6-4872-a3ef-98325ec8b843")
                IPhoneCallManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ShowPhoneCallUI(
                        HSTRING phoneNumber,
                        HSTRING displayName
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallManagerStatics = __uuidof(IPhoneCallManagerStatics);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallManagerStatics2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallManagerStatics2[] = L"Windows.ApplicationModel.Calls.IPhoneCallManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("c7e3c8bc-2370-431c-98fd-43be5f03086d")
                IPhoneCallManagerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_CallStateChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_CallStateChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsCallActive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsCallIncoming(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowPhoneCallSettingsUI(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestStoreAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallManagerStatics2 = __uuidof(IPhoneCallManagerStatics2);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallStatics
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCall
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallStatics[] = L"Windows.ApplicationModel.Calls.IPhoneCallStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("2218eeab-f60b-53e7-ba13-5aeafbc22957")
                IPhoneCallStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetFromId(
                        HSTRING callId,
                        ABI::Windows::ApplicationModel::Calls::IPhoneCall** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallStatics = __uuidof(IPhoneCallStatics);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallStore
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallStore
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallStore[] = L"Windows.ApplicationModel.Calls.IPhoneCallStore";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("5f610748-18a6-4173-86d1-28be9dc62dba")
                IPhoneCallStore : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsEmergencyPhoneNumberAsync(
                        HSTRING number,
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDefaultLineAsync(
                        __FIAsyncOperation_1_GUID** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestLineWatcher(
                        ABI::Windows::ApplicationModel::Calls::IPhoneLineWatcher** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallStore = __uuidof(IPhoneCallStore);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallVideoCapabilities
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallVideoCapabilities[] = L"Windows.ApplicationModel.Calls.IPhoneCallVideoCapabilities";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("02382786-b16a-4fdb-be3b-c4240e13ad0d")
                IPhoneCallVideoCapabilities : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsVideoCallingCapable(
                        boolean* pValue
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallVideoCapabilities = __uuidof(IPhoneCallVideoCapabilities);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallVideoCapabilitiesManagerStatics
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallVideoCapabilitiesManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallVideoCapabilitiesManagerStatics[] = L"Windows.ApplicationModel.Calls.IPhoneCallVideoCapabilitiesManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("f3c64b56-f00b-4a1c-a0c6-ee1910749ce7")
                IPhoneCallVideoCapabilitiesManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetCapabilitiesAsync(
                        HSTRING phoneNumber,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallVideoCapabilitiesManagerStatics = __uuidof(IPhoneCallVideoCapabilitiesManagerStatics);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallsResult
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallsResult
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallsResult[] = L"Windows.ApplicationModel.Calls.IPhoneCallsResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("1bfad365-57cf-57dd-986d-b057c91eac33")
                IPhoneCallsResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_OperationStatus(
                        ABI::Windows::ApplicationModel::Calls::PhoneLineOperationStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllActivePhoneCalls(
                        __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneCallsResult = __uuidof(IPhoneCallsResult);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneDialOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneDialOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneDialOptions[] = L"Windows.ApplicationModel.Calls.IPhoneDialOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("b639c4b8-f06f-36cb-a863-823742b5f2d4")
                IPhoneDialOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Number(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Number(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisplayName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Contact(
                        ABI::Windows::ApplicationModel::Contacts::IContact** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Contact(
                        ABI::Windows::ApplicationModel::Contacts::IContact* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContactPhone(
                        ABI::Windows::ApplicationModel::Contacts::IContactPhone** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ContactPhone(
                        ABI::Windows::ApplicationModel::Contacts::IContactPhone* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Media(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallMedia* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Media(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallMedia value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AudioEndpoint(
                        ABI::Windows::ApplicationModel::Calls::PhoneAudioRoutingEndpoint* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AudioEndpoint(
                        ABI::Windows::ApplicationModel::Calls::PhoneAudioRoutingEndpoint value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneDialOptions = __uuidof(IPhoneDialOptions);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLine
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLine
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLine[] = L"Windows.ApplicationModel.Calls.IPhoneLine";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("27c66f30-6a69-34ca-a2ba-65302530c311")
                IPhoneLine : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_LineChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_LineChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayColor(
                        ABI::Windows::UI::Color* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkState(
                        ABI::Windows::ApplicationModel::Calls::PhoneNetworkState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Voicemail(
                        ABI::Windows::ApplicationModel::Calls::IPhoneVoicemail** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CellularDetails(
                        ABI::Windows::ApplicationModel::Calls::IPhoneLineCellularDetails** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Transport(
                        ABI::Windows::ApplicationModel::Calls::PhoneLineTransport* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CanDial(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportsTile(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VideoCallingCapabilities(
                        ABI::Windows::ApplicationModel::Calls::IPhoneCallVideoCapabilities** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LineConfiguration(
                        ABI::Windows::ApplicationModel::Calls::IPhoneLineConfiguration** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsImmediateDialNumberAsync(
                        HSTRING number,
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Dial(
                        HSTRING number,
                        HSTRING displayName
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DialWithOptions(
                        ABI::Windows::ApplicationModel::Calls::IPhoneDialOptions* options
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneLine = __uuidof(IPhoneLine);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLine2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLine
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLine2[] = L"Windows.ApplicationModel.Calls.IPhoneLine2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("0167f56a-5344-5d64-8af3-a31a950e916a")
                IPhoneLine2 : public IInspectable
                {
                public:
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                    DEPRECATED("EnableTextReply is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
                    virtual HRESULT STDMETHODCALLTYPE EnableTextReply(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransportDeviceId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneLine2 = __uuidof(IPhoneLine2);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLine3
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLine
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLine3[] = L"Windows.ApplicationModel.Calls.IPhoneLine3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("e2e33cf7-2406-57f3-826a-e5a5f40d6fb5")
                IPhoneLine3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE DialWithResult(
                        HSTRING number,
                        HSTRING displayName,
                        ABI::Windows::ApplicationModel::Calls::IPhoneLineDialResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DialWithResultAsync(
                        HSTRING number,
                        HSTRING displayName,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAllActivePhoneCalls(
                        ABI::Windows::ApplicationModel::Calls::IPhoneCallsResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAllActivePhoneCallsAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneLine3 = __uuidof(IPhoneLine3);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineCellularDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLineCellularDetails
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineCellularDetails[] = L"Windows.ApplicationModel.Calls.IPhoneLineCellularDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("192601d5-147c-4769-b673-98a5ec8426cb")
                IPhoneLineCellularDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SimState(
                        ABI::Windows::ApplicationModel::Calls::PhoneSimState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SimSlotIndex(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsModemOn(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RegistrationRejectCode(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetNetworkOperatorDisplayText(
                        ABI::Windows::ApplicationModel::Calls::PhoneLineNetworkOperatorDisplayTextLocation location,
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneLineCellularDetails = __uuidof(IPhoneLineCellularDetails);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineConfiguration
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLineConfiguration
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineConfiguration[] = L"Windows.ApplicationModel.Calls.IPhoneLineConfiguration";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("fe265862-f64f-4312-b2a8-4e257721aa95")
                IPhoneLineConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsVideoCallingEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedProperties(
                        __FIMapView_2_HSTRING_IInspectable** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneLineConfiguration = __uuidof(IPhoneLineConfiguration);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineDialResult
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLineDialResult
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineDialResult[] = L"Windows.ApplicationModel.Calls.IPhoneLineDialResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("e825a30a-5c7f-546f-b918-3ad2fe70fb34")
                IPhoneLineDialResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DialCallStatus(
                        ABI::Windows::ApplicationModel::Calls::PhoneCallOperationStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DialedCall(
                        ABI::Windows::ApplicationModel::Calls::IPhoneCall** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneLineDialResult = __uuidof(IPhoneLineDialResult);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineStatics
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLine
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineStatics[] = L"Windows.ApplicationModel.Calls.IPhoneLineStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("f38b5f23-ceb0-404f-bcf2-ba9f697d8adf")
                IPhoneLineStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        GUID lineId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneLineStatics = __uuidof(IPhoneLineStatics);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineTransportDevice
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLineTransportDevice
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineTransportDevice[] = L"Windows.ApplicationModel.Calls.IPhoneLineTransportDevice";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("efa8f889-cffa-59f4-97e4-74705b7dc490")
                IPhoneLineTransportDevice : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Transport(
                        ABI::Windows::ApplicationModel::Calls::PhoneLineTransport* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestAccessAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RegisterApp(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RegisterAppForUser(
                        ABI::Windows::System::IUser* user
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UnregisterApp(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UnregisterAppForUser(
                        ABI::Windows::System::IUser* user
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsRegistered(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Connect(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ConnectAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneLineTransportDevice = __uuidof(IPhoneLineTransportDevice);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineTransportDevice2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLineTransportDevice
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineTransportDevice2[] = L"Windows.ApplicationModel.Calls.IPhoneLineTransportDevice2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("64c885f2-ecf4-5761-8c04-3c248ce61690")
                IPhoneLineTransportDevice2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AudioRoutingStatus(
                        ABI::Windows::ApplicationModel::Calls::TransportDeviceAudioRoutingStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AudioRoutingStatusChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AudioRoutingStatusChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InBandRingingEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_InBandRingingEnabledChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_InBandRingingEnabledChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneLineTransportDevice2 = __uuidof(IPhoneLineTransportDevice2);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineTransportDeviceStatics
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLineTransportDevice
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineTransportDeviceStatics[] = L"Windows.ApplicationModel.Calls.IPhoneLineTransportDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("0f3121ac-d609-51a1-96f3-fb00d1819252")
                IPhoneLineTransportDeviceStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE FromId(
                        HSTRING id,
                        ABI::Windows::ApplicationModel::Calls::IPhoneLineTransportDevice** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorForPhoneLineTransport(
                        ABI::Windows::ApplicationModel::Calls::PhoneLineTransport transport,
                        HSTRING* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneLineTransportDeviceStatics = __uuidof(IPhoneLineTransportDeviceStatics);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineWatcher
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLineWatcher
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineWatcher[] = L"Windows.ApplicationModel.Calls.IPhoneLineWatcher";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("8a45cd0a-6323-44e0-a6f6-9f21f64dc90a")
                IPhoneLineWatcher : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_LineAdded(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_LineAdded(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_LineRemoved(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_LineRemoved(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_LineUpdated(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_LineUpdated(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_EnumerationCompleted(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_EnumerationCompleted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_Stopped(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_Stopped(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::ApplicationModel::Calls::PhoneLineWatcherStatus* status
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneLineWatcher = __uuidof(IPhoneLineWatcher);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineWatcherEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLineWatcherEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineWatcherEventArgs[] = L"Windows.ApplicationModel.Calls.IPhoneLineWatcherEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("d07c753e-9e12-4a37-82b7-ad535dad6a67")
                IPhoneLineWatcherEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LineId(
                        GUID* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneLineWatcherEventArgs = __uuidof(IPhoneLineWatcherEventArgs);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneVoicemail
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneVoicemail
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneVoicemail[] = L"Windows.ApplicationModel.Calls.IPhoneVoicemail";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("c9ce77f6-6e9f-3a8b-b727-6e0cf6998224")
                IPhoneVoicemail : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Number(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MessageCount(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Type(
                        ABI::Windows::ApplicationModel::Calls::PhoneVoicemailType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DialVoicemailAsync(
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPhoneVoicemail = __uuidof(IPhoneVoicemail);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipCallCoordinator
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipCallCoordinator
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipCallCoordinator[] = L"Windows.ApplicationModel.Calls.IVoipCallCoordinator";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("4f118bcf-e8ef-4434-9c5f-a8d893fafe79")
                IVoipCallCoordinator : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReserveCallResourcesAsync(
                        HSTRING taskEntryPoint,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_MuteStateChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs* muteChangeHandler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_MuteStateChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestNewIncomingCall(
                        HSTRING context,
                        HSTRING contactName,
                        HSTRING contactNumber,
                        ABI::Windows::Foundation::IUriRuntimeClass* contactImage,
                        HSTRING serviceName,
                        ABI::Windows::Foundation::IUriRuntimeClass* brandingImage,
                        HSTRING callDetails,
                        ABI::Windows::Foundation::IUriRuntimeClass* ringtone,
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia media,
                        ABI::Windows::Foundation::TimeSpan ringTimeout,
                        ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall** call
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestNewOutgoingCall(
                        HSTRING context,
                        HSTRING contactName,
                        HSTRING serviceName,
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia media,
                        ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall** call
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyMuted(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyUnmuted(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestOutgoingUpgradeToVideoCall(
                        GUID callUpgradeGuid,
                        HSTRING context,
                        HSTRING contactName,
                        HSTRING serviceName,
                        ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall** call
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestIncomingUpgradeToVideoCall(
                        HSTRING context,
                        HSTRING contactName,
                        HSTRING contactNumber,
                        ABI::Windows::Foundation::IUriRuntimeClass* contactImage,
                        HSTRING serviceName,
                        ABI::Windows::Foundation::IUriRuntimeClass* brandingImage,
                        HSTRING callDetails,
                        ABI::Windows::Foundation::IUriRuntimeClass* ringtone,
                        ABI::Windows::Foundation::TimeSpan ringTimeout,
                        ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall** call
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TerminateCellularCall(
                        GUID callUpgradeGuid
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CancelUpgrade(
                        GUID callUpgradeGuid
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVoipCallCoordinator = __uuidof(IVoipCallCoordinator);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipCallCoordinator2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipCallCoordinator
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Calls.IVoipCallCoordinator
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipCallCoordinator2[] = L"Windows.ApplicationModel.Calls.IVoipCallCoordinator2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("beb4a9f3-c704-4234-89ce-e88cc0d28fbe")
                IVoipCallCoordinator2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetupNewAcceptedCall(
                        HSTRING context,
                        HSTRING contactName,
                        HSTRING contactNumber,
                        HSTRING serviceName,
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia media,
                        ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall** call
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVoipCallCoordinator2 = __uuidof(IVoipCallCoordinator2);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipCallCoordinator3
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipCallCoordinator
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Calls.IVoipCallCoordinator
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipCallCoordinator3[] = L"Windows.ApplicationModel.Calls.IVoipCallCoordinator3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("338d0cbf-9b55-4021-87ca-e64b9bd666c7")
                IVoipCallCoordinator3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestNewAppInitiatedCall(
                        HSTRING context,
                        HSTRING contactName,
                        HSTRING contactNumber,
                        HSTRING serviceName,
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia media,
                        ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall** call
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestNewIncomingCallWithContactRemoteId(
                        HSTRING context,
                        HSTRING contactName,
                        HSTRING contactNumber,
                        ABI::Windows::Foundation::IUriRuntimeClass* contactImage,
                        HSTRING serviceName,
                        ABI::Windows::Foundation::IUriRuntimeClass* brandingImage,
                        HSTRING callDetails,
                        ABI::Windows::Foundation::IUriRuntimeClass* ringtone,
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia media,
                        ABI::Windows::Foundation::TimeSpan ringTimeout,
                        HSTRING contactRemoteId,
                        ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall** call
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVoipCallCoordinator3 = __uuidof(IVoipCallCoordinator3);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipCallCoordinator4
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipCallCoordinator
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Calls.IVoipCallCoordinator
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipCallCoordinator4[] = L"Windows.ApplicationModel.Calls.IVoipCallCoordinator4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("83737239-9311-468f-bb49-47e0dfb5d93e")
                IVoipCallCoordinator4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReserveOneProcessCallResourcesAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVoipCallCoordinator4 = __uuidof(IVoipCallCoordinator4);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipCallCoordinator5
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipCallCoordinator
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipCallCoordinator5[] = L"Windows.ApplicationModel.Calls.IVoipCallCoordinator5";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("d4f79017-d1c1-5820-955e-7a1676355d00")
                IVoipCallCoordinator5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestNewIncomingCallWithOptions(
                        ABI::Windows::ApplicationModel::Calls::IIncomingVoipPhoneCallOptions* callOptions,
                        ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestNewOutgoingCallWithOptions(
                        ABI::Windows::ApplicationModel::Calls::IOutgoingVoipPhoneCallOptions* callOptions,
                        ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetupNewAcceptedCallWithOptions(
                        ABI::Windows::ApplicationModel::Calls::IAcceptedVoipPhoneCallOptions* callOptions,
                        ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestNewAppInitiatedCallWithOptions(
                        ABI::Windows::ApplicationModel::Calls::IAppInitiatedVoipPhoneCallOptions* callOptions,
                        ABI::Windows::ApplicationModel::Calls::IVoipPhoneCall** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVoipCallCoordinator5 = __uuidof(IVoipCallCoordinator5);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipCallCoordinatorStatics
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipCallCoordinator
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipCallCoordinatorStatics[] = L"Windows.ApplicationModel.Calls.IVoipCallCoordinatorStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("7f5d1f2b-e04a-4d10-b31a-a55c922cc2fb")
                IVoipCallCoordinatorStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::ApplicationModel::Calls::IVoipCallCoordinator** coordinator
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVoipCallCoordinatorStatics = __uuidof(IVoipCallCoordinatorStatics);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipCallCoordinatorStatics2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipCallCoordinator
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipCallCoordinatorStatics2[] = L"Windows.ApplicationModel.Calls.IVoipCallCoordinatorStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("b8d0288b-01ea-5478-8404-a1fb06f2b83b")
                IVoipCallCoordinatorStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE IsCallControlDeviceKindSupportedForAssociation(
                        ABI::Windows::ApplicationModel::Calls::VoipCallControlDeviceKind kind,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelectorForCallControl(
                        HSTRING* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVoipCallCoordinatorStatics2 = __uuidof(IVoipCallCoordinatorStatics2);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipPhoneCall
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipPhoneCall
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipPhoneCall[] = L"Windows.ApplicationModel.Calls.IVoipPhoneCall";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("6cf1f19a-7794-4a5a-8c68-ae87947a6990")
                IVoipPhoneCall : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_EndRequested(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_EndRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_HoldRequested(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_HoldRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ResumeRequested(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ResumeRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AnswerRequested(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs* acceptHandler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AnswerRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RejectRequested(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs* rejectHandler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RejectRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyCallHeld(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyCallActive(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyCallEnded(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContactName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ContactName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StartTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_StartTime(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CallMedia(
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CallMedia(
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyCallReady(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IVoipPhoneCall = __uuidof(IVoipPhoneCall);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipPhoneCall2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipPhoneCall
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Calls.IVoipPhoneCall
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipPhoneCall2[] = L"Windows.ApplicationModel.Calls.IVoipPhoneCall2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("741b46e1-245f-41f3-9399-3141d25b52e3")
                IVoipPhoneCall2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE TryShowAppUI(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IVoipPhoneCall2 = __uuidof(IVoipPhoneCall2);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipPhoneCall3
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipPhoneCall
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Calls.IVoipPhoneCall2
 *     Windows.ApplicationModel.Calls.IVoipPhoneCall
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipPhoneCall3[] = L"Windows.ApplicationModel.Calls.IVoipPhoneCall3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("0d891522-e258-4aa9-907a-1aa413c25523")
                IVoipPhoneCall3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE NotifyCallAccepted(
                        ABI::Windows::ApplicationModel::Calls::VoipPhoneCallMedia media
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVoipPhoneCall3 = __uuidof(IVoipPhoneCall3);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipPhoneCall4
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipPhoneCall
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipPhoneCall4[] = L"Windows.ApplicationModel.Calls.IVoipPhoneCall4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                MIDL_INTERFACE("eba66290-ad6d-5899-bdda-81bfe9f999a1")
                IVoipPhoneCall4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsUsingAssociatedDevicesList(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyCallActiveOnDevices(
                        __FIIterable_1_HSTRING* associatedDeviceIds
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddAssociatedCallControlDevice(
                        HSTRING deviceId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveAssociatedCallControlDevice(
                        HSTRING deviceId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetAssociatedCallControlDevices(
                        __FIIterable_1_HSTRING* associatedDeviceIds
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAssociatedCallControlDevices(
                        __FIVectorView_1_HSTRING** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVoipPhoneCall4 = __uuidof(IVoipPhoneCall4);
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Calls.AcceptedVoipPhoneCallOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *   Type can be activated via the Windows.ApplicationModel.Calls.IAcceptedVoipPhoneCallOptionsFactory interface starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IAcceptedVoipPhoneCallOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_AcceptedVoipPhoneCallOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_AcceptedVoipPhoneCallOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_AcceptedVoipPhoneCallOptions[] = L"Windows.ApplicationModel.Calls.AcceptedVoipPhoneCallOptions";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Calls.AppInitiatedVoipPhoneCallOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Calls.IAppInitiatedVoipPhoneCallOptionsFactory interface starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IAppInitiatedVoipPhoneCallOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_AppInitiatedVoipPhoneCallOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_AppInitiatedVoipPhoneCallOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_AppInitiatedVoipPhoneCallOptions[] = L"Windows.ApplicationModel.Calls.AppInitiatedVoipPhoneCallOptions";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Calls.CallAnswerEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.ICallAnswerEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Calls.ICallAnswerEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_CallAnswerEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_CallAnswerEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_CallAnswerEventArgs[] = L"Windows.ApplicationModel.Calls.CallAnswerEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.CallRejectEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.ICallRejectEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_CallRejectEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_CallRejectEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_CallRejectEventArgs[] = L"Windows.ApplicationModel.Calls.CallRejectEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.CallStateChangeEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.ICallStateChangeEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_CallStateChangeEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_CallStateChangeEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_CallStateChangeEventArgs[] = L"Windows.ApplicationModel.Calls.CallStateChangeEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.IncomingVoipPhoneCallOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Calls.IIncomingVoipPhoneCallOptionsFactory interface starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IIncomingVoipPhoneCallOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_IncomingVoipPhoneCallOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_IncomingVoipPhoneCallOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_IncomingVoipPhoneCallOptions[] = L"Windows.ApplicationModel.Calls.IncomingVoipPhoneCallOptions";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Calls.LockScreenCallEndCallDeferral
 *
 * Introduced to Windows.ApplicationModel.Calls.LockScreenCallContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.ILockScreenCallEndCallDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_LockScreenCallEndCallDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_LockScreenCallEndCallDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_LockScreenCallEndCallDeferral[] = L"Windows.ApplicationModel.Calls.LockScreenCallEndCallDeferral";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.LockScreenCallEndRequestedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.LockScreenCallContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.ILockScreenCallEndRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_LockScreenCallEndRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_LockScreenCallEndRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_LockScreenCallEndRequestedEventArgs[] = L"Windows.ApplicationModel.Calls.LockScreenCallEndRequestedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.LockScreenCallUI
 *
 * Introduced to Windows.ApplicationModel.Calls.LockScreenCallContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.ILockScreenCallUI ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_LockScreenCallUI_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_LockScreenCallUI_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_LockScreenCallUI[] = L"Windows.ApplicationModel.Calls.LockScreenCallUI";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.MuteChangeEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IMuteChangeEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_MuteChangeEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_MuteChangeEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_MuteChangeEventArgs[] = L"Windows.ApplicationModel.Calls.MuteChangeEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.OutgoingVoipPhoneCallOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Calls.IOutgoingVoipPhoneCallOptionsFactory interface starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IOutgoingVoipPhoneCallOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_OutgoingVoipPhoneCallOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_OutgoingVoipPhoneCallOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_OutgoingVoipPhoneCallOptions[] = L"Windows.ApplicationModel.Calls.OutgoingVoipPhoneCallOptions";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCall
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneCallStatics interface starting with version 6.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCall ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCall_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCall_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCall[] = L"Windows.ApplicationModel.Calls.PhoneCall";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallBlocking
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneCallBlockingStatics interface starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallBlocking_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallBlocking_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallBlocking[] = L"Windows.ApplicationModel.Calls.PhoneCallBlocking";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallHistoryEntry
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallHistoryEntry ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryEntry_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryEntry_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallHistoryEntry[] = L"Windows.ApplicationModel.Calls.PhoneCallHistoryEntry";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryAddressFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryAddress ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryAddress_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryAddress_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryAddress[] = L"Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryQueryOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryQueryOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryQueryOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryQueryOptions[] = L"Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallHistoryEntryReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryReader ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryReader_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryReader[] = L"Windows.ApplicationModel.Calls.PhoneCallHistoryEntryReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallHistoryManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallHistoryManager[] = L"Windows.ApplicationModel.Calls.PhoneCallHistoryManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallHistoryManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerForUser ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryManagerForUser_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryManagerForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallHistoryManagerForUser[] = L"Windows.ApplicationModel.Calls.PhoneCallHistoryManagerForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallHistoryStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallHistoryStore ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryStore_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallHistoryStore[] = L"Windows.ApplicationModel.Calls.PhoneCallHistoryStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallInfo
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallInfo[] = L"Windows.ApplicationModel.Calls.PhoneCallInfo";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallManager
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneCallManagerStatics interface starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneCallManagerStatics2 interface starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallManager[] = L"Windows.ApplicationModel.Calls.PhoneCallManager";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallStore
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallStore ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallStore_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallStore[] = L"Windows.ApplicationModel.Calls.PhoneCallStore";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallVideoCapabilities ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallVideoCapabilities_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallVideoCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallVideoCapabilities[] = L"Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallVideoCapabilitiesManager
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneCallVideoCapabilitiesManagerStatics interface starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallVideoCapabilitiesManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallVideoCapabilitiesManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallVideoCapabilitiesManager[] = L"Windows.ApplicationModel.Calls.PhoneCallVideoCapabilitiesManager";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallsResult
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallsResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallsResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallsResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallsResult[] = L"Windows.ApplicationModel.Calls.PhoneCallsResult";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneDialOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneDialOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneDialOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneDialOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneDialOptions[] = L"Windows.ApplicationModel.Calls.PhoneDialOptions";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneLine
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneLineStatics interface starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneLine ** Default Interface **
 *    Windows.ApplicationModel.Calls.IPhoneLine2
 *    Windows.ApplicationModel.Calls.IPhoneLine3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLine_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLine_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneLine[] = L"Windows.ApplicationModel.Calls.PhoneLine";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneLineCellularDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneLineCellularDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineCellularDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineCellularDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneLineCellularDetails[] = L"Windows.ApplicationModel.Calls.PhoneLineCellularDetails";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneLineConfiguration
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneLineConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineConfiguration_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneLineConfiguration[] = L"Windows.ApplicationModel.Calls.PhoneLineConfiguration";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneLineDialResult
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneLineDialResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineDialResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineDialResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneLineDialResult[] = L"Windows.ApplicationModel.Calls.PhoneLineDialResult";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneLineTransportDevice
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneLineTransportDeviceStatics interface starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneLineTransportDevice ** Default Interface **
 *    Windows.ApplicationModel.Calls.IPhoneLineTransportDevice2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineTransportDevice_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineTransportDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneLineTransportDevice[] = L"Windows.ApplicationModel.Calls.PhoneLineTransportDevice";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneLineWatcher
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneLineWatcher ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineWatcher_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneLineWatcher[] = L"Windows.ApplicationModel.Calls.PhoneLineWatcher";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneLineWatcherEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneLineWatcherEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineWatcherEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineWatcherEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneLineWatcherEventArgs[] = L"Windows.ApplicationModel.Calls.PhoneLineWatcherEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneVoicemail
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneVoicemail ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneVoicemail_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneVoicemail_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneVoicemail[] = L"Windows.ApplicationModel.Calls.PhoneVoicemail";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.VoipCallCoordinator
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IVoipCallCoordinatorStatics interface starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IVoipCallCoordinatorStatics2 interface starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IVoipCallCoordinator ** Default Interface **
 *    Windows.ApplicationModel.Calls.IVoipCallCoordinator2
 *    Windows.ApplicationModel.Calls.IVoipCallCoordinator3
 *    Windows.ApplicationModel.Calls.IVoipCallCoordinator4
 *    Windows.ApplicationModel.Calls.IVoipCallCoordinator5
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_VoipCallCoordinator_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_VoipCallCoordinator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_VoipCallCoordinator[] = L"Windows.ApplicationModel.Calls.VoipCallCoordinator";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.VoipPhoneCall
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IVoipPhoneCall ** Default Interface **
 *    Windows.ApplicationModel.Calls.IVoipPhoneCall2
 *    Windows.ApplicationModel.Calls.IVoipPhoneCall3
 *    Windows.ApplicationModel.Calls.IVoipPhoneCall4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_VoipPhoneCall_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_VoipPhoneCall_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_VoipPhoneCall[] = L"Windows.ApplicationModel.Calls.VoipPhoneCall";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2 __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs __x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs __x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs __x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2 __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2 __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2 __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3 __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2 __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2 __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3 __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4 __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5 __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2 __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2 __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3 __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4 __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_GUID __FIAsyncOperationCompletedHandler_1_GUID;

#if !defined(____FIAsyncOperation_1_GUID_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_GUID_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_GUID __FIAsyncOperation_1_GUID;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_GUID;

typedef struct __FIAsyncOperation_1_GUIDVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_GUID* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_GUID* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_GUID* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_GUID* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_GUID* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_GUID* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_GUID* This,
        __FIAsyncOperationCompletedHandler_1_GUID* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_GUID* This,
        __FIAsyncOperationCompletedHandler_1_GUID** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_GUID* This,
        GUID* result);

    END_INTERFACE
} __FIAsyncOperation_1_GUIDVtbl;

interface __FIAsyncOperation_1_GUID
{
    CONST_VTBL struct __FIAsyncOperation_1_GUIDVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_GUID_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_GUID_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_GUID_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_GUID_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_GUID_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_GUID_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_GUID_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_GUID_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_GUID_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_GUID_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_GUID_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_GUID_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_GUID __FIAsyncOperationCompletedHandler_1_GUID;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_GUID;

typedef struct __FIAsyncOperationCompletedHandler_1_GUIDVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_GUID* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_GUID* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_GUID* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_GUID* This,
        __FIAsyncOperation_1_GUID* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_GUIDVtbl;

interface __FIAsyncOperationCompletedHandler_1_GUID
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_GUIDVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_GUID_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_GUID_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_GUID_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_GUID_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_GUID_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_UINT32 __FIAsyncOperationCompletedHandler_1_UINT32;

#if !defined(____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_UINT32 __FIAsyncOperation_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_UINT32;

typedef struct __FIAsyncOperation_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_UINT32* This,
        __FIAsyncOperationCompletedHandler_1_UINT32* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_UINT32* This,
        __FIAsyncOperationCompletedHandler_1_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_UINT32* This,
        UINT32* result);

    END_INTERFACE
} __FIAsyncOperation_1_UINT32Vtbl;

interface __FIAsyncOperation_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperation_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_UINT32_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_UINT32_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_UINT32_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_UINT32 __FIAsyncOperationCompletedHandler_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_UINT32;

typedef struct __FIAsyncOperationCompletedHandler_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_UINT32* This,
        __FIAsyncOperation_1_UINT32* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_UINT32Vtbl;

interface __FIAsyncOperationCompletedHandler_1_UINT32
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_UINT32_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStoreVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStoreVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo;

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfoVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfoVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallOperationStatus __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallOperationStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus;

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallOperationStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatusVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore;

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStoreVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStoreVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallStore_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities;

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilitiesVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilitiesVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult;

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine;

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLine_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult;

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallResourceReservationStatus __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallResourceReservationStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus;

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallResourceReservationStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatusVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceAccessStatus __x_ABI_CWindows_CDevices_CEnumeration_CDeviceAccessStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CDeviceAccessStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry;

typedef struct __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry;

typedef struct __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry_INTERFACE_DEFINED__
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

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall;

typedef struct __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCallVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall;

typedef struct __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        __FIIterator_1_Windows__CApplicationModel__CCalls__CPhoneCall** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCall_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

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

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

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

#if !defined(____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIEventHandler_1_IInspectable __FIEventHandler_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIEventHandler_1_IInspectable;

typedef struct __FIEventHandler_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIEventHandler_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIEventHandler_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIEventHandler_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIEventHandler_1_IInspectable* This,
        IInspectable* sender,
        IInspectable* args);

    END_INTERFACE
} __FIEventHandler_1_IInspectableVtbl;

interface __FIEventHandler_1_IInspectable
{
    CONST_VTBL struct __FIEventHandler_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIEventHandler_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIEventHandler_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIEventHandler_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIEventHandler_1_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FIEventHandler_1_IInspectable_INTERFACE_DEFINED__

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

#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI* sender,
        __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* sender,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* sender,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* sender,
        __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* sender,
        __x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* sender,
        __x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CIContact __x_ABI_CWindows_CApplicationModel_CContacts_CIContact;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactPhone_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactPhone_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CIContactPhone __x_ABI_CWindows_CApplicationModel_CContacts_CIContactPhone;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactPhone_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CUI_CColor __x_ABI_CWindows_CUI_CColor;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CDtmfKey __x_ABI_CWindows_CApplicationModel_CCalls_CDtmfKey;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CDtmfToneAudioPlayback __x_ABI_CWindows_CApplicationModel_CCalls_CDtmfToneAudioPlayback;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneAudioRoutingEndpoint __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneAudioRoutingEndpoint;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallAudioDevice __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallAudioDevice;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallDirection __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallDirection;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryMedia __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryMedia;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryOtherAppReadAccess __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryOtherAppReadAccess;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryQueryDesiredMedia __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryQueryDesiredMedia;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryRawAddressKind __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryRawAddressKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistorySourceIdKind __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistorySourceIdKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryStoreAccessType __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryStoreAccessType;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallMedia __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallMedia;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallStatus __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineNetworkOperatorDisplayTextLocation __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineNetworkOperatorDisplayTextLocation;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineOperationStatus __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineOperationStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineTransport __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineTransport;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineWatcherStatus __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineWatcherStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneNetworkState __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneNetworkState;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneSimState __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneSimState;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneVoicemailType __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneVoicemailType;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CTransportDeviceAudioRoutingStatus __x_ABI_CWindows_CApplicationModel_CCalls_CTransportDeviceAudioRoutingStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipCallControlDeviceKind __x_ABI_CWindows_CApplicationModel_CCalls_CVoipCallControlDeviceKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallRejectReason __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallRejectReason;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallState __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallState;

/*
 *
 * Struct Windows.ApplicationModel.Calls.CellularDtmfMode
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CCellularDtmfMode
{
    CellularDtmfMode_Continuous = 0,
    CellularDtmfMode_Burst = 1,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.DtmfKey
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CDtmfKey
{
    DtmfKey_D0 = 0,
    DtmfKey_D1 = 1,
    DtmfKey_D2 = 2,
    DtmfKey_D3 = 3,
    DtmfKey_D4 = 4,
    DtmfKey_D5 = 5,
    DtmfKey_D6 = 6,
    DtmfKey_D7 = 7,
    DtmfKey_D8 = 8,
    DtmfKey_D9 = 9,
    DtmfKey_Star = 10,
    DtmfKey_Pound = 11,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.Calls.DtmfToneAudioPlayback
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CDtmfToneAudioPlayback
{
    DtmfToneAudioPlayback_Play = 0,
    DtmfToneAudioPlayback_DoNotPlay = 1,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneAudioRoutingEndpoint
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneAudioRoutingEndpoint
{
    PhoneAudioRoutingEndpoint_Default = 0,
    PhoneAudioRoutingEndpoint_Bluetooth = 1,
    PhoneAudioRoutingEndpoint_Speakerphone = 2,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallAudioDevice
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallAudioDevice
{
    PhoneCallAudioDevice_Unknown = 0,
    PhoneCallAudioDevice_LocalDevice = 1,
    PhoneCallAudioDevice_RemoteDevice = 2,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallDirection
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallDirection
{
    PhoneCallDirection_Unknown = 0,
    PhoneCallDirection_Incoming = 1,
    PhoneCallDirection_Outgoing = 2,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallHistoryEntryMedia
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryMedia
{
    PhoneCallHistoryEntryMedia_Audio = 0,
    PhoneCallHistoryEntryMedia_Video = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallHistoryEntryOtherAppReadAccess
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryOtherAppReadAccess
{
    PhoneCallHistoryEntryOtherAppReadAccess_Full = 0,
    PhoneCallHistoryEntryOtherAppReadAccess_SystemOnly = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryDesiredMedia
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryQueryDesiredMedia
{
    PhoneCallHistoryEntryQueryDesiredMedia_None = 0,
    PhoneCallHistoryEntryQueryDesiredMedia_Audio = 0x1,
    PhoneCallHistoryEntryQueryDesiredMedia_Video = 0x2,
    PhoneCallHistoryEntryQueryDesiredMedia_All = 0xffffffff,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallHistoryEntryRawAddressKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryRawAddressKind
{
    PhoneCallHistoryEntryRawAddressKind_PhoneNumber = 0,
    PhoneCallHistoryEntryRawAddressKind_Custom = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallHistorySourceIdKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistorySourceIdKind
{
    PhoneCallHistorySourceIdKind_CellularPhoneLineId = 0,
    PhoneCallHistorySourceIdKind_PackageFamilyName = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallHistoryStoreAccessType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryStoreAccessType
{
    PhoneCallHistoryStoreAccessType_AppEntriesReadWrite = 0,
    PhoneCallHistoryStoreAccessType_AllEntriesLimitedReadWrite = 1,
    PhoneCallHistoryStoreAccessType_AllEntriesReadWrite = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallMedia
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallMedia
{
    PhoneCallMedia_Audio = 0,
    PhoneCallMedia_AudioAndVideo = 1,
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x40000
    PhoneCallMedia_AudioAndRealTimeText = 2,
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x40000
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallOperationStatus
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallOperationStatus
{
    PhoneCallOperationStatus_Succeeded = 0,
    PhoneCallOperationStatus_OtherFailure = 1,
    PhoneCallOperationStatus_TimedOut = 2,
    PhoneCallOperationStatus_ConnectionLost = 3,
    PhoneCallOperationStatus_InvalidCallState = 4,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneCallStatus
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallStatus
{
    PhoneCallStatus_Lost = 0,
    PhoneCallStatus_Incoming = 1,
    PhoneCallStatus_Dialing = 2,
    PhoneCallStatus_Talking = 3,
    PhoneCallStatus_Held = 4,
    PhoneCallStatus_Ended = 5,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneLineNetworkOperatorDisplayTextLocation
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineNetworkOperatorDisplayTextLocation
{
    PhoneLineNetworkOperatorDisplayTextLocation_Default = 0,
    PhoneLineNetworkOperatorDisplayTextLocation_Tile = 1,
    PhoneLineNetworkOperatorDisplayTextLocation_Dialer = 2,
    PhoneLineNetworkOperatorDisplayTextLocation_InCallUI = 3,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneLineOperationStatus
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineOperationStatus
{
    PhoneLineOperationStatus_Succeeded = 0,
    PhoneLineOperationStatus_OtherFailure = 1,
    PhoneLineOperationStatus_TimedOut = 2,
    PhoneLineOperationStatus_ConnectionLost = 3,
    PhoneLineOperationStatus_InvalidCallState = 4,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneLineTransport
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineTransport
{
    PhoneLineTransport_Cellular = 0,
    PhoneLineTransport_VoipApp = 1,
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000
    PhoneLineTransport_Bluetooth = 2,
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneLineWatcherStatus
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineWatcherStatus
{
    PhoneLineWatcherStatus_Created = 0,
    PhoneLineWatcherStatus_Started = 1,
    PhoneLineWatcherStatus_EnumerationCompleted = 2,
    PhoneLineWatcherStatus_Stopped = 3,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneNetworkState
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneNetworkState
{
    PhoneNetworkState_Unknown = 0,
    PhoneNetworkState_NoSignal = 1,
    PhoneNetworkState_Deregistered = 2,
    PhoneNetworkState_Denied = 3,
    PhoneNetworkState_Searching = 4,
    PhoneNetworkState_Home = 5,
    PhoneNetworkState_RoamingInternational = 6,
    PhoneNetworkState_RoamingDomestic = 7,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneSimState
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneSimState
{
    PhoneSimState_Unknown = 0,
    PhoneSimState_PinNotRequired = 1,
    PhoneSimState_PinUnlocked = 2,
    PhoneSimState_PinLocked = 3,
    PhoneSimState_PukLocked = 4,
    PhoneSimState_NotInserted = 5,
    PhoneSimState_Invalid = 6,
    PhoneSimState_Disabled = 7,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.PhoneVoicemailType
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneVoicemailType
{
    PhoneVoicemailType_None = 0,
    PhoneVoicemailType_Traditional = 1,
    PhoneVoicemailType_Visual = 2,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.TransportDeviceAudioRoutingStatus
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CTransportDeviceAudioRoutingStatus
{
    TransportDeviceAudioRoutingStatus_Unknown = 0,
    TransportDeviceAudioRoutingStatus_CanRouteToLocalDevice = 1,
    TransportDeviceAudioRoutingStatus_CannotRouteToLocalDevice = 2,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.Calls.VoipCallControlDeviceKind
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipCallControlDeviceKind
{
    VoipCallControlDeviceKind_Bluetooth = 0,
    VoipCallControlDeviceKind_Usb = 1,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.ApplicationModel.Calls.VoipPhoneCallMedia
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia
{
    VoipPhoneCallMedia_None = 0,
    VoipPhoneCallMedia_Audio = 0x1,
    VoipPhoneCallMedia_Video = 0x2,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.VoipPhoneCallRejectReason
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallRejectReason
{
    VoipPhoneCallRejectReason_UserIgnored = 0,
    VoipPhoneCallRejectReason_TimedOut = 1,
    VoipPhoneCallRejectReason_OtherIncomingCall = 2,
    VoipPhoneCallRejectReason_EmergencyCallExists = 3,
    VoipPhoneCallRejectReason_InvalidCallState = 4,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.VoipPhoneCallResourceReservationStatus
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallResourceReservationStatus
{
    VoipPhoneCallResourceReservationStatus_Success = 0,
    VoipPhoneCallResourceReservationStatus_ResourcesNotAvailable = 1,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Calls.VoipPhoneCallState
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallState
{
    VoipPhoneCallState_Ended = 0,
    VoipPhoneCallState_Held = 1,
    VoipPhoneCallState_Active = 2,
    VoipPhoneCallState_Incoming = 3,
    VoipPhoneCallState_Outgoing = 4,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IAcceptedVoipPhoneCallOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.AcceptedVoipPhoneCallOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IAcceptedVoipPhoneCallOptions[] = L"Windows.ApplicationModel.Calls.IAcceptedVoipPhoneCallOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Context)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Context)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ContactName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ContactName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ContactNumber)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ContactNumber)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ServiceName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Media)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia* value);
    HRESULT (STDMETHODCALLTYPE* put_Media)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia value);
    HRESULT (STDMETHODCALLTYPE* get_AssociatedDeviceIds)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* This,
        __FIVector_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_get_Context(This, value) \
    ((This)->lpVtbl->get_Context(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_put_Context(This, value) \
    ((This)->lpVtbl->put_Context(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_get_ContactName(This, value) \
    ((This)->lpVtbl->get_ContactName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_put_ContactName(This, value) \
    ((This)->lpVtbl->put_ContactName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_get_ContactNumber(This, value) \
    ((This)->lpVtbl->get_ContactNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_put_ContactNumber(This, value) \
    ((This)->lpVtbl->put_ContactNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_get_ServiceName(This, value) \
    ((This)->lpVtbl->get_ServiceName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_put_ServiceName(This, value) \
    ((This)->lpVtbl->put_ServiceName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_get_Media(This, value) \
    ((This)->lpVtbl->get_Media(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_put_Media(This, value) \
    ((This)->lpVtbl->put_Media(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_get_AssociatedDeviceIds(This, value) \
    ((This)->lpVtbl->get_AssociatedDeviceIds(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IAcceptedVoipPhoneCallOptionsFactory
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.AcceptedVoipPhoneCallOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IAcceptedVoipPhoneCallOptionsFactory[] = L"Windows.ApplicationModel.Calls.IAcceptedVoipPhoneCallOptionsFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory* This,
        __FIIterable_1_HSTRING* associatedDeviceIds,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_CreateInstance(This, associatedDeviceIds, value) \
    ((This)->lpVtbl->CreateInstance(This, associatedDeviceIds, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IAppInitiatedVoipPhoneCallOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.AppInitiatedVoipPhoneCallOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IAppInitiatedVoipPhoneCallOptions[] = L"Windows.ApplicationModel.Calls.IAppInitiatedVoipPhoneCallOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Context)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Context)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ContactName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ContactName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ContactNumber)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ContactNumber)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ServiceName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Media)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia* value);
    HRESULT (STDMETHODCALLTYPE* put_Media)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia value);
    HRESULT (STDMETHODCALLTYPE* get_AssociatedDeviceIds)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* This,
        __FIVector_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_get_Context(This, value) \
    ((This)->lpVtbl->get_Context(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_put_Context(This, value) \
    ((This)->lpVtbl->put_Context(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_get_ContactName(This, value) \
    ((This)->lpVtbl->get_ContactName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_put_ContactName(This, value) \
    ((This)->lpVtbl->put_ContactName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_get_ContactNumber(This, value) \
    ((This)->lpVtbl->get_ContactNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_put_ContactNumber(This, value) \
    ((This)->lpVtbl->put_ContactNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_get_ServiceName(This, value) \
    ((This)->lpVtbl->get_ServiceName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_put_ServiceName(This, value) \
    ((This)->lpVtbl->put_ServiceName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_get_Media(This, value) \
    ((This)->lpVtbl->get_Media(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_put_Media(This, value) \
    ((This)->lpVtbl->put_Media(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_get_AssociatedDeviceIds(This, value) \
    ((This)->lpVtbl->get_AssociatedDeviceIds(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IAppInitiatedVoipPhoneCallOptionsFactory
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.AppInitiatedVoipPhoneCallOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IAppInitiatedVoipPhoneCallOptionsFactory[] = L"Windows.ApplicationModel.Calls.IAppInitiatedVoipPhoneCallOptionsFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory* This,
        __FIIterable_1_HSTRING* associatedDeviceIds,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_CreateInstance(This, associatedDeviceIds, value) \
    ((This)->lpVtbl->CreateInstance(This, associatedDeviceIds, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.ICallAnswerEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.CallAnswerEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_ICallAnswerEventArgs[] = L"Windows.ApplicationModel.Calls.ICallAnswerEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AcceptedMedia)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_get_AcceptedMedia(This, value) \
    ((This)->lpVtbl->get_AcceptedMedia(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.ICallAnswerEventArgs2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.CallAnswerEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_ICallAnswerEventArgs2[] = L"Windows.ApplicationModel.Calls.ICallAnswerEventArgs2";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourceDeviceId)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_get_SourceDeviceId(This, value) \
    ((This)->lpVtbl->get_SourceDeviceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CICallAnswerEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.ICallRejectEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.CallRejectEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_ICallRejectEventArgs[] = L"Windows.ApplicationModel.Calls.ICallRejectEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RejectReason)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallRejectReason* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_get_RejectReason(This, value) \
    ((This)->lpVtbl->get_RejectReason(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CICallRejectEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.ICallStateChangeEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.CallStateChangeEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_ICallStateChangeEventArgs[] = L"Windows.ApplicationModel.Calls.ICallStateChangeEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallState* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CICallStateChangeEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IIncomingVoipPhoneCallOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.IncomingVoipPhoneCallOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IIncomingVoipPhoneCallOptions[] = L"Windows.ApplicationModel.Calls.IIncomingVoipPhoneCallOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Context)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Context)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ContactName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ContactName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ContactNumber)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ContactNumber)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ContactImage)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_ContactImage)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ServiceName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_BrandingImage)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_BrandingImage)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_CallDetails)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CallDetails)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Ringtone)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_Ringtone)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_Media)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia* value);
    HRESULT (STDMETHODCALLTYPE* put_Media)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia value);
    HRESULT (STDMETHODCALLTYPE* get_RingTimeout)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_RingTimeout)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_ContactRemoteId)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ContactRemoteId)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_AssociatedDeviceIds)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* This,
        __FIVector_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_get_Context(This, value) \
    ((This)->lpVtbl->get_Context(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_put_Context(This, value) \
    ((This)->lpVtbl->put_Context(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_get_ContactName(This, value) \
    ((This)->lpVtbl->get_ContactName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_put_ContactName(This, value) \
    ((This)->lpVtbl->put_ContactName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_get_ContactNumber(This, value) \
    ((This)->lpVtbl->get_ContactNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_put_ContactNumber(This, value) \
    ((This)->lpVtbl->put_ContactNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_get_ContactImage(This, value) \
    ((This)->lpVtbl->get_ContactImage(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_put_ContactImage(This, value) \
    ((This)->lpVtbl->put_ContactImage(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_get_ServiceName(This, value) \
    ((This)->lpVtbl->get_ServiceName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_put_ServiceName(This, value) \
    ((This)->lpVtbl->put_ServiceName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_get_BrandingImage(This, value) \
    ((This)->lpVtbl->get_BrandingImage(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_put_BrandingImage(This, value) \
    ((This)->lpVtbl->put_BrandingImage(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_get_CallDetails(This, value) \
    ((This)->lpVtbl->get_CallDetails(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_put_CallDetails(This, value) \
    ((This)->lpVtbl->put_CallDetails(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_get_Ringtone(This, value) \
    ((This)->lpVtbl->get_Ringtone(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_put_Ringtone(This, value) \
    ((This)->lpVtbl->put_Ringtone(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_get_Media(This, value) \
    ((This)->lpVtbl->get_Media(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_put_Media(This, value) \
    ((This)->lpVtbl->put_Media(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_get_RingTimeout(This, value) \
    ((This)->lpVtbl->get_RingTimeout(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_put_RingTimeout(This, value) \
    ((This)->lpVtbl->put_RingTimeout(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_get_ContactRemoteId(This, value) \
    ((This)->lpVtbl->get_ContactRemoteId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_put_ContactRemoteId(This, value) \
    ((This)->lpVtbl->put_ContactRemoteId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_get_AssociatedDeviceIds(This, value) \
    ((This)->lpVtbl->get_AssociatedDeviceIds(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IIncomingVoipPhoneCallOptionsFactory
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.IncomingVoipPhoneCallOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IIncomingVoipPhoneCallOptionsFactory[] = L"Windows.ApplicationModel.Calls.IIncomingVoipPhoneCallOptionsFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory* This,
        __FIIterable_1_HSTRING* associatedDeviceIds,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_CreateInstance(This, associatedDeviceIds, value) \
    ((This)->lpVtbl->CreateInstance(This, associatedDeviceIds, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.ILockScreenCallEndCallDeferral
 *
 * Introduced to Windows.ApplicationModel.Calls.LockScreenCallContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.LockScreenCallEndCallDeferral
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_ILockScreenCallEndCallDeferral[] = L"Windows.ApplicationModel.Calls.ILockScreenCallEndCallDeferral";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferralVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.ILockScreenCallEndRequestedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.LockScreenCallContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.LockScreenCallEndRequestedEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_ILockScreenCallEndRequestedEventArgs[] = L"Windows.ApplicationModel.Calls.ILockScreenCallEndRequestedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndCallDeferral** value);
    HRESULT (STDMETHODCALLTYPE* get_Deadline)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_get_Deadline(This, value) \
    ((This)->lpVtbl->get_Deadline(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallEndRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.ILockScreenCallUI
 *
 * Introduced to Windows.ApplicationModel.Calls.LockScreenCallContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.LockScreenCallUI
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_ILockScreenCallUI[] = L"Windows.ApplicationModel.Calls.ILockScreenCallUI";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUIVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Dismiss)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI* This);
    HRESULT (STDMETHODCALLTYPE* add_EndRequested)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_Windows__CApplicationModel__CCalls__CLockScreenCallEndRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EndRequested)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Closed)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CLockScreenCallUI_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Closed)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_CallTitle)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CallTitle)(__x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUIVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUIVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_Dismiss(This) \
    ((This)->lpVtbl->Dismiss(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_add_EndRequested(This, handler, token) \
    ((This)->lpVtbl->add_EndRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_remove_EndRequested(This, token) \
    ((This)->lpVtbl->remove_EndRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_add_Closed(This, handler, token) \
    ((This)->lpVtbl->add_Closed(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_remove_Closed(This, token) \
    ((This)->lpVtbl->remove_Closed(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_get_CallTitle(This, value) \
    ((This)->lpVtbl->get_CallTitle(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_put_CallTitle(This, value) \
    ((This)->lpVtbl->put_CallTitle(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CILockScreenCallUI_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IMuteChangeEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.MuteChangeEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IMuteChangeEventArgs[] = L"Windows.ApplicationModel.Calls.IMuteChangeEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Muted)(__x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_get_Muted(This, value) \
    ((This)->lpVtbl->get_Muted(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIMuteChangeEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IOutgoingVoipPhoneCallOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.OutgoingVoipPhoneCallOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IOutgoingVoipPhoneCallOptions[] = L"Windows.ApplicationModel.Calls.IOutgoingVoipPhoneCallOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Context)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Context)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ContactName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ContactName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ServiceName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Media)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia* value);
    HRESULT (STDMETHODCALLTYPE* put_Media)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia value);
    HRESULT (STDMETHODCALLTYPE* get_AssociatedDeviceIds)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions* This,
        __FIVector_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_get_Context(This, value) \
    ((This)->lpVtbl->get_Context(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_put_Context(This, value) \
    ((This)->lpVtbl->put_Context(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_get_ContactName(This, value) \
    ((This)->lpVtbl->get_ContactName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_put_ContactName(This, value) \
    ((This)->lpVtbl->put_ContactName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_get_ServiceName(This, value) \
    ((This)->lpVtbl->get_ServiceName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_put_ServiceName(This, value) \
    ((This)->lpVtbl->put_ServiceName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_get_Media(This, value) \
    ((This)->lpVtbl->get_Media(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_put_Media(This, value) \
    ((This)->lpVtbl->put_Media(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_get_AssociatedDeviceIds(This, value) \
    ((This)->lpVtbl->get_AssociatedDeviceIds(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IOutgoingVoipPhoneCallOptionsFactory
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.OutgoingVoipPhoneCallOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IOutgoingVoipPhoneCallOptionsFactory[] = L"Windows.ApplicationModel.Calls.IOutgoingVoipPhoneCallOptionsFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory* This,
        __FIIterable_1_HSTRING* associatedDeviceIds,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_CreateInstance(This, associatedDeviceIds, value) \
    ((This)->lpVtbl->CreateInstance(This, associatedDeviceIds, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCall
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCall
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCall[] = L"Windows.ApplicationModel.Calls.IPhoneCall";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_StatusChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StatusChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_AudioDeviceChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AudioDeviceChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_IsMutedChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneCall_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_IsMutedChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_CallId)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsMuted)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_AudioDevice)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallAudioDevice* value);
    HRESULT (STDMETHODCALLTYPE* GetPhoneCallInfo)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo** result);
    HRESULT (STDMETHODCALLTYPE* GetPhoneCallInfoAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallInfo** operation);
    HRESULT (STDMETHODCALLTYPE* End)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallOperationStatus* result);
    HRESULT (STDMETHODCALLTYPE* EndAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation);
    HRESULT (STDMETHODCALLTYPE* SendDtmfKey)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CDtmfKey key,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CDtmfToneAudioPlayback dtmfToneAudioPlayback,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallOperationStatus* result);
    HRESULT (STDMETHODCALLTYPE* SendDtmfKeyAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CDtmfKey key,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CDtmfToneAudioPlayback dtmfToneAudioPlayback,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation);
    HRESULT (STDMETHODCALLTYPE* AcceptIncoming)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallOperationStatus* result);
    HRESULT (STDMETHODCALLTYPE* AcceptIncomingAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation);
    HRESULT (STDMETHODCALLTYPE* Hold)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallOperationStatus* result);
    HRESULT (STDMETHODCALLTYPE* HoldAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation);
    HRESULT (STDMETHODCALLTYPE* ResumeFromHold)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallOperationStatus* result);
    HRESULT (STDMETHODCALLTYPE* ResumeFromHoldAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation);
    HRESULT (STDMETHODCALLTYPE* Mute)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallOperationStatus* result);
    HRESULT (STDMETHODCALLTYPE* MuteAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation);
    HRESULT (STDMETHODCALLTYPE* Unmute)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallOperationStatus* result);
    HRESULT (STDMETHODCALLTYPE* UnmuteAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation);
    HRESULT (STDMETHODCALLTYPE* RejectIncoming)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallOperationStatus* result);
    HRESULT (STDMETHODCALLTYPE* RejectIncomingAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation);
    HRESULT (STDMETHODCALLTYPE* ChangeAudioDevice)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallAudioDevice endpoint,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallOperationStatus* result);
    HRESULT (STDMETHODCALLTYPE* ChangeAudioDeviceAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallAudioDevice endpoint,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallOperationStatus** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_add_StatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_StatusChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_remove_StatusChanged(This, token) \
    ((This)->lpVtbl->remove_StatusChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_add_AudioDeviceChanged(This, handler, token) \
    ((This)->lpVtbl->add_AudioDeviceChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_remove_AudioDeviceChanged(This, token) \
    ((This)->lpVtbl->remove_AudioDeviceChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_add_IsMutedChanged(This, handler, token) \
    ((This)->lpVtbl->add_IsMutedChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_remove_IsMutedChanged(This, token) \
    ((This)->lpVtbl->remove_IsMutedChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_get_CallId(This, value) \
    ((This)->lpVtbl->get_CallId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_get_IsMuted(This, value) \
    ((This)->lpVtbl->get_IsMuted(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_get_AudioDevice(This, value) \
    ((This)->lpVtbl->get_AudioDevice(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_GetPhoneCallInfo(This, result) \
    ((This)->lpVtbl->GetPhoneCallInfo(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_GetPhoneCallInfoAsync(This, operation) \
    ((This)->lpVtbl->GetPhoneCallInfoAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_End(This, result) \
    ((This)->lpVtbl->End(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_EndAsync(This, operation) \
    ((This)->lpVtbl->EndAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_SendDtmfKey(This, key, dtmfToneAudioPlayback, result) \
    ((This)->lpVtbl->SendDtmfKey(This, key, dtmfToneAudioPlayback, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_SendDtmfKeyAsync(This, key, dtmfToneAudioPlayback, operation) \
    ((This)->lpVtbl->SendDtmfKeyAsync(This, key, dtmfToneAudioPlayback, operation))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_AcceptIncoming(This, result) \
    ((This)->lpVtbl->AcceptIncoming(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_AcceptIncomingAsync(This, operation) \
    ((This)->lpVtbl->AcceptIncomingAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_Hold(This, result) \
    ((This)->lpVtbl->Hold(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_HoldAsync(This, operation) \
    ((This)->lpVtbl->HoldAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_ResumeFromHold(This, result) \
    ((This)->lpVtbl->ResumeFromHold(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_ResumeFromHoldAsync(This, operation) \
    ((This)->lpVtbl->ResumeFromHoldAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_Mute(This, result) \
    ((This)->lpVtbl->Mute(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_MuteAsync(This, operation) \
    ((This)->lpVtbl->MuteAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_Unmute(This, result) \
    ((This)->lpVtbl->Unmute(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_UnmuteAsync(This, operation) \
    ((This)->lpVtbl->UnmuteAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_RejectIncoming(This, result) \
    ((This)->lpVtbl->RejectIncoming(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_RejectIncomingAsync(This, operation) \
    ((This)->lpVtbl->RejectIncomingAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_ChangeAudioDevice(This, endpoint, result) \
    ((This)->lpVtbl->ChangeAudioDevice(This, endpoint, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_ChangeAudioDeviceAsync(This, endpoint, operation) \
    ((This)->lpVtbl->ChangeAudioDeviceAsync(This, endpoint, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallBlockingStatics
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallBlocking
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallBlockingStatics[] = L"Windows.ApplicationModel.Calls.IPhoneCallBlockingStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_BlockUnknownNumbers)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_BlockUnknownNumbers)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_BlockPrivateNumbers)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_BlockPrivateNumbers)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* SetCallBlockingListAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics* This,
        __FIIterable_1_HSTRING* phoneNumberList,
        __FIAsyncOperation_1_boolean** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_get_BlockUnknownNumbers(This, value) \
    ((This)->lpVtbl->get_BlockUnknownNumbers(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_put_BlockUnknownNumbers(This, value) \
    ((This)->lpVtbl->put_BlockUnknownNumbers(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_get_BlockPrivateNumbers(This, value) \
    ((This)->lpVtbl->get_BlockPrivateNumbers(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_put_BlockPrivateNumbers(This, value) \
    ((This)->lpVtbl->put_BlockPrivateNumbers(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_SetCallBlockingListAsync(This, phoneNumberList, result) \
    ((This)->lpVtbl->SetCallBlockingListAsync(This, phoneNumberList, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallBlockingStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryEntry
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryEntry
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryEntry[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryEntry";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Address)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress** value);
    HRESULT (STDMETHODCALLTYPE* put_Address)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress* value);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* put_Duration)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_IsCallerIdBlocked)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsCallerIdBlocked)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsEmergency)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsEmergency)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsIncoming)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsIncoming)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsMissed)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsMissed)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsRinging)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsRinging)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsSeen)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsSeen)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsSuppressed)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsSuppressed)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsVoicemail)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsVoicemail)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_Media)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryMedia* value);
    HRESULT (STDMETHODCALLTYPE* put_Media)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryMedia value);
    HRESULT (STDMETHODCALLTYPE* get_OtherAppReadAccess)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryOtherAppReadAccess* value);
    HRESULT (STDMETHODCALLTYPE* put_OtherAppReadAccess)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryOtherAppReadAccess value);
    HRESULT (STDMETHODCALLTYPE* get_RemoteId)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_RemoteId)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_SourceDisplayName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SourceId)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_SourceId)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_SourceIdKind)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistorySourceIdKind* value);
    HRESULT (STDMETHODCALLTYPE* put_SourceIdKind)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistorySourceIdKind value);
    HRESULT (STDMETHODCALLTYPE* get_StartTime)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* put_StartTime)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_Address(This, value) \
    ((This)->lpVtbl->get_Address(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_put_Address(This, value) \
    ((This)->lpVtbl->put_Address(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_put_Duration(This, value) \
    ((This)->lpVtbl->put_Duration(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_IsCallerIdBlocked(This, value) \
    ((This)->lpVtbl->get_IsCallerIdBlocked(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_put_IsCallerIdBlocked(This, value) \
    ((This)->lpVtbl->put_IsCallerIdBlocked(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_IsEmergency(This, value) \
    ((This)->lpVtbl->get_IsEmergency(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_put_IsEmergency(This, value) \
    ((This)->lpVtbl->put_IsEmergency(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_IsIncoming(This, value) \
    ((This)->lpVtbl->get_IsIncoming(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_put_IsIncoming(This, value) \
    ((This)->lpVtbl->put_IsIncoming(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_IsMissed(This, value) \
    ((This)->lpVtbl->get_IsMissed(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_put_IsMissed(This, value) \
    ((This)->lpVtbl->put_IsMissed(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_IsRinging(This, value) \
    ((This)->lpVtbl->get_IsRinging(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_put_IsRinging(This, value) \
    ((This)->lpVtbl->put_IsRinging(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_IsSeen(This, value) \
    ((This)->lpVtbl->get_IsSeen(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_put_IsSeen(This, value) \
    ((This)->lpVtbl->put_IsSeen(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_IsSuppressed(This, value) \
    ((This)->lpVtbl->get_IsSuppressed(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_put_IsSuppressed(This, value) \
    ((This)->lpVtbl->put_IsSuppressed(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_IsVoicemail(This, value) \
    ((This)->lpVtbl->get_IsVoicemail(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_put_IsVoicemail(This, value) \
    ((This)->lpVtbl->put_IsVoicemail(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_Media(This, value) \
    ((This)->lpVtbl->get_Media(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_put_Media(This, value) \
    ((This)->lpVtbl->put_Media(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_OtherAppReadAccess(This, value) \
    ((This)->lpVtbl->get_OtherAppReadAccess(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_put_OtherAppReadAccess(This, value) \
    ((This)->lpVtbl->put_OtherAppReadAccess(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_RemoteId(This, value) \
    ((This)->lpVtbl->get_RemoteId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_put_RemoteId(This, value) \
    ((This)->lpVtbl->put_RemoteId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_SourceDisplayName(This, value) \
    ((This)->lpVtbl->get_SourceDisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_SourceId(This, value) \
    ((This)->lpVtbl->get_SourceId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_put_SourceId(This, value) \
    ((This)->lpVtbl->put_SourceId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_SourceIdKind(This, value) \
    ((This)->lpVtbl->get_SourceIdKind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_put_SourceIdKind(This, value) \
    ((This)->lpVtbl->put_SourceIdKind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_get_StartTime(This, value) \
    ((This)->lpVtbl->get_StartTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_put_StartTime(This, value) \
    ((This)->lpVtbl->put_StartTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryAddress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryEntryAddress[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryAddress";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContactId)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ContactId)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_RawAddress)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_RawAddress)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_RawAddressKind)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryRawAddressKind* value);
    HRESULT (STDMETHODCALLTYPE* put_RawAddressKind)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryRawAddressKind value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_get_ContactId(This, value) \
    ((This)->lpVtbl->get_ContactId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_put_ContactId(This, value) \
    ((This)->lpVtbl->put_ContactId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_get_RawAddress(This, value) \
    ((This)->lpVtbl->get_RawAddress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_put_RawAddress(This, value) \
    ((This)->lpVtbl->put_RawAddress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_get_RawAddressKind(This, value) \
    ((This)->lpVtbl->get_RawAddressKind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_put_RawAddressKind(This, value) \
    ((This)->lpVtbl->put_RawAddressKind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryAddressFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryEntryAddressFactory[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryAddressFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory* This,
        HSTRING rawAddress,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryRawAddressKind rawAddressKind,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddress** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_Create(This, rawAddress, rawAddressKind, result) \
    ((This)->lpVtbl->Create(This, rawAddress, rawAddressKind, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryAddressFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryQueryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryEntryQueryOptions[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryQueryOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DesiredMedia)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryQueryDesiredMedia* value);
    HRESULT (STDMETHODCALLTYPE* put_DesiredMedia)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryEntryQueryDesiredMedia value);
    HRESULT (STDMETHODCALLTYPE* get_SourceIds)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions* This,
        __FIVector_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_get_DesiredMedia(This, value) \
    ((This)->lpVtbl->get_DesiredMedia(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_put_DesiredMedia(This, value) \
    ((This)->lpVtbl->put_DesiredMedia(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_get_SourceIds(This, value) \
    ((This)->lpVtbl->get_SourceIds(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryEntryReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryEntryReader[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryReader";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReadBatchAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReaderVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_ReadBatchAsync(This, result) \
    ((This)->lpVtbl->ReadBatchAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryManagerForUser
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryManagerForUser[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerForUser";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUserVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestStoreAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryStoreAccessType accessType,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore** result);
    HRESULT (STDMETHODCALLTYPE* get_User)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser* This,
        __x_ABI_CWindows_CSystem_CIUser** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUserVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUserVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_RequestStoreAsync(This, accessType, result) \
    ((This)->lpVtbl->RequestStoreAsync(This, accessType, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_get_User(This, value) \
    ((This)->lpVtbl->get_User(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryManagerStatics[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestStoreAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallHistoryStoreAccessType accessType,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryStore** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_RequestStoreAsync(This, accessType, result) \
    ((This)->lpVtbl->RequestStoreAsync(This, accessType, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryManagerStatics2[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForUser)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerForUser** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_GetForUser(This, user, result) \
    ((This)->lpVtbl->GetForUser(This, user, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallHistoryStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallHistoryStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallHistoryStore[] = L"Windows.ApplicationModel.Calls.IPhoneCallHistoryStore";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetEntryAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This,
        HSTRING callHistoryEntryId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry** result);
    HRESULT (STDMETHODCALLTYPE* GetEntryReader)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader** result);
    HRESULT (STDMETHODCALLTYPE* GetEntryReaderWithOptions)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryQueryOptions* queryOptions,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntryReader** result);
    HRESULT (STDMETHODCALLTYPE* SaveEntryAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* callHistoryEntry,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* DeleteEntryAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* callHistoryEntry,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* DeleteEntriesAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This,
        __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* callHistoryEntries,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* MarkEntryAsSeenAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryEntry* callHistoryEntry,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* MarkEntriesAsSeenAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This,
        __FIIterable_1_Windows__CApplicationModel__CCalls__CPhoneCallHistoryEntry* callHistoryEntries,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* GetUnseenCountAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This,
        __FIAsyncOperation_1_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* MarkAllAsSeenAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* GetSourcesUnseenCountAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This,
        __FIIterable_1_HSTRING* sourceIds,
        __FIAsyncOperation_1_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* MarkSourcesAsSeenAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore* This,
        __FIIterable_1_HSTRING* sourceIds,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStoreVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_GetEntryAsync(This, callHistoryEntryId, result) \
    ((This)->lpVtbl->GetEntryAsync(This, callHistoryEntryId, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_GetEntryReader(This, result) \
    ((This)->lpVtbl->GetEntryReader(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_GetEntryReaderWithOptions(This, queryOptions, result) \
    ((This)->lpVtbl->GetEntryReaderWithOptions(This, queryOptions, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_SaveEntryAsync(This, callHistoryEntry, result) \
    ((This)->lpVtbl->SaveEntryAsync(This, callHistoryEntry, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_DeleteEntryAsync(This, callHistoryEntry, result) \
    ((This)->lpVtbl->DeleteEntryAsync(This, callHistoryEntry, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_DeleteEntriesAsync(This, callHistoryEntries, result) \
    ((This)->lpVtbl->DeleteEntriesAsync(This, callHistoryEntries, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_MarkEntryAsSeenAsync(This, callHistoryEntry, result) \
    ((This)->lpVtbl->MarkEntryAsSeenAsync(This, callHistoryEntry, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_MarkEntriesAsSeenAsync(This, callHistoryEntries, result) \
    ((This)->lpVtbl->MarkEntriesAsSeenAsync(This, callHistoryEntries, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_GetUnseenCountAsync(This, result) \
    ((This)->lpVtbl->GetUnseenCountAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_MarkAllAsSeenAsync(This, result) \
    ((This)->lpVtbl->MarkAllAsSeenAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_GetSourcesUnseenCountAsync(This, sourceIds, result) \
    ((This)->lpVtbl->GetSourcesUnseenCountAsync(This, sourceIds, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_MarkSourcesAsSeenAsync(This, sourceIds, result) \
    ((This)->lpVtbl->MarkSourcesAsSeenAsync(This, sourceIds, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallHistoryStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallInfo
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallInfo
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallInfo[] = L"Windows.ApplicationModel.Calls.IPhoneCallInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LineId)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_IsHoldSupported)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_StartTime)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_PhoneNumber)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CallDirection)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallDirection* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_get_LineId(This, value) \
    ((This)->lpVtbl->get_LineId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_get_IsHoldSupported(This, value) \
    ((This)->lpVtbl->get_IsHoldSupported(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_get_StartTime(This, value) \
    ((This)->lpVtbl->get_StartTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_get_PhoneNumber(This, value) \
    ((This)->lpVtbl->get_PhoneNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_get_CallDirection(This, value) \
    ((This)->lpVtbl->get_CallDirection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallManagerStatics
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallManagerStatics[] = L"Windows.ApplicationModel.Calls.IPhoneCallManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ShowPhoneCallUI)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics* This,
        HSTRING phoneNumber,
        HSTRING displayName);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_ShowPhoneCallUI(This, phoneNumber, displayName) \
    ((This)->lpVtbl->ShowPhoneCallUI(This, phoneNumber, displayName))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallManagerStatics2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallManagerStatics2[] = L"Windows.ApplicationModel.Calls.IPhoneCallManagerStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_CallStateChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CallStateChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_IsCallActive)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsCallIncoming)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* ShowPhoneCallSettingsUI)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* RequestStoreAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallStore** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_add_CallStateChanged(This, handler, token) \
    ((This)->lpVtbl->add_CallStateChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_remove_CallStateChanged(This, token) \
    ((This)->lpVtbl->remove_CallStateChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_get_IsCallActive(This, value) \
    ((This)->lpVtbl->get_IsCallActive(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_get_IsCallIncoming(This, value) \
    ((This)->lpVtbl->get_IsCallIncoming(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_ShowPhoneCallSettingsUI(This) \
    ((This)->lpVtbl->ShowPhoneCallSettingsUI(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_RequestStoreAsync(This, result) \
    ((This)->lpVtbl->RequestStoreAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallStatics
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCall
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallStatics[] = L"Windows.ApplicationModel.Calls.IPhoneCallStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetFromId)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics* This,
        HSTRING callId,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_GetFromId(This, callId, result) \
    ((This)->lpVtbl->GetFromId(This, callId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallStore
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallStore
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallStore[] = L"Windows.ApplicationModel.Calls.IPhoneCallStore";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsEmergencyPhoneNumberAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore* This,
        HSTRING number,
        __FIAsyncOperation_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* GetDefaultLineAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore* This,
        __FIAsyncOperation_1_GUID** result);
    HRESULT (STDMETHODCALLTYPE* RequestLineWatcher)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStoreVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_IsEmergencyPhoneNumberAsync(This, number, result) \
    ((This)->lpVtbl->IsEmergencyPhoneNumberAsync(This, number, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_GetDefaultLineAsync(This, result) \
    ((This)->lpVtbl->GetDefaultLineAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_RequestLineWatcher(This, result) \
    ((This)->lpVtbl->RequestLineWatcher(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallVideoCapabilities
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallVideoCapabilities[] = L"Windows.ApplicationModel.Calls.IPhoneCallVideoCapabilities";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsVideoCallingCapable)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities* This,
        boolean* pValue);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_get_IsVideoCallingCapable(This, pValue) \
    ((This)->lpVtbl->get_IsVideoCallingCapable(This, pValue))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallVideoCapabilitiesManagerStatics
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallVideoCapabilitiesManager
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallVideoCapabilitiesManagerStatics[] = L"Windows.ApplicationModel.Calls.IPhoneCallVideoCapabilitiesManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCapabilitiesAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics* This,
        HSTRING phoneNumber,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallVideoCapabilities** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_GetCapabilitiesAsync(This, phoneNumber, result) \
    ((This)->lpVtbl->GetCapabilitiesAsync(This, phoneNumber, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilitiesManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneCallsResult
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneCallsResult
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneCallsResult[] = L"Windows.ApplicationModel.Calls.IPhoneCallsResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OperationStatus)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineOperationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_AllActivePhoneCalls)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult* This,
        __FIVectorView_1_Windows__CApplicationModel__CCalls__CPhoneCall** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_get_OperationStatus(This, value) \
    ((This)->lpVtbl->get_OperationStatus(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_get_AllActivePhoneCalls(This, value) \
    ((This)->lpVtbl->get_AllActivePhoneCalls(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneDialOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneDialOptions
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneDialOptions[] = L"Windows.ApplicationModel.Calls.IPhoneDialOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Number)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Number)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DisplayName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContact** value);
    HRESULT (STDMETHODCALLTYPE* put_Contact)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContact* value);
    HRESULT (STDMETHODCALLTYPE* get_ContactPhone)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactPhone** value);
    HRESULT (STDMETHODCALLTYPE* put_ContactPhone)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactPhone* value);
    HRESULT (STDMETHODCALLTYPE* get_Media)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallMedia* value);
    HRESULT (STDMETHODCALLTYPE* put_Media)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallMedia value);
    HRESULT (STDMETHODCALLTYPE* get_AudioEndpoint)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneAudioRoutingEndpoint* value);
    HRESULT (STDMETHODCALLTYPE* put_AudioEndpoint)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneAudioRoutingEndpoint value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_get_Number(This, value) \
    ((This)->lpVtbl->get_Number(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_put_Number(This, value) \
    ((This)->lpVtbl->put_Number(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_put_DisplayName(This, value) \
    ((This)->lpVtbl->put_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_put_Contact(This, value) \
    ((This)->lpVtbl->put_Contact(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_get_ContactPhone(This, value) \
    ((This)->lpVtbl->get_ContactPhone(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_put_ContactPhone(This, value) \
    ((This)->lpVtbl->put_ContactPhone(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_get_Media(This, value) \
    ((This)->lpVtbl->get_Media(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_put_Media(This, value) \
    ((This)->lpVtbl->put_Media(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_get_AudioEndpoint(This, value) \
    ((This)->lpVtbl->get_AudioEndpoint(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_put_AudioEndpoint(This, value) \
    ((This)->lpVtbl->put_AudioEndpoint(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLine
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLine
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLine[] = L"Windows.ApplicationModel.Calls.IPhoneLine";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_LineChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLine_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_LineChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayColor)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        struct __x_ABI_CWindows_CUI_CColor* value);
    HRESULT (STDMETHODCALLTYPE* get_NetworkState)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneNetworkState* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Voicemail)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail** value);
    HRESULT (STDMETHODCALLTYPE* get_NetworkName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CellularDetails)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails** value);
    HRESULT (STDMETHODCALLTYPE* get_Transport)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineTransport* value);
    HRESULT (STDMETHODCALLTYPE* get_CanDial)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportsTile)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_VideoCallingCapabilities)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallVideoCapabilities** value);
    HRESULT (STDMETHODCALLTYPE* get_LineConfiguration)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration** value);
    HRESULT (STDMETHODCALLTYPE* IsImmediateDialNumberAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        HSTRING number,
        __FIAsyncOperation_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* Dial)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        HSTRING number,
        HSTRING displayName);
    HRESULT (STDMETHODCALLTYPE* DialWithOptions)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneDialOptions* options);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_add_LineChanged(This, handler, token) \
    ((This)->lpVtbl->add_LineChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_remove_LineChanged(This, token) \
    ((This)->lpVtbl->remove_LineChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_get_DisplayColor(This, value) \
    ((This)->lpVtbl->get_DisplayColor(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_get_NetworkState(This, value) \
    ((This)->lpVtbl->get_NetworkState(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_get_Voicemail(This, value) \
    ((This)->lpVtbl->get_Voicemail(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_get_NetworkName(This, value) \
    ((This)->lpVtbl->get_NetworkName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_get_CellularDetails(This, value) \
    ((This)->lpVtbl->get_CellularDetails(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_get_Transport(This, value) \
    ((This)->lpVtbl->get_Transport(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_get_CanDial(This, value) \
    ((This)->lpVtbl->get_CanDial(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_get_SupportsTile(This, value) \
    ((This)->lpVtbl->get_SupportsTile(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_get_VideoCallingCapabilities(This, value) \
    ((This)->lpVtbl->get_VideoCallingCapabilities(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_get_LineConfiguration(This, value) \
    ((This)->lpVtbl->get_LineConfiguration(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_IsImmediateDialNumberAsync(This, number, result) \
    ((This)->lpVtbl->IsImmediateDialNumberAsync(This, number, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_Dial(This, number, displayName) \
    ((This)->lpVtbl->Dial(This, number, displayName))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_DialWithOptions(This, options) \
    ((This)->lpVtbl->DialWithOptions(This, options))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLine2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLine
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLine2[] = L"Windows.ApplicationModel.Calls.IPhoneLine2";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("EnableTextReply is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    HRESULT (STDMETHODCALLTYPE* EnableTextReply)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_TransportDeviceId)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
    DEPRECATED("EnableTextReply is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x70000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_EnableTextReply(This, value) \
    ((This)->lpVtbl->EnableTextReply(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_get_TransportDeviceId(This, value) \
    ((This)->lpVtbl->get_TransportDeviceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLine3
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLine
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLine3[] = L"Windows.ApplicationModel.Calls.IPhoneLine3";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* DialWithResult)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3* This,
        HSTRING number,
        HSTRING displayName,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult** result);
    HRESULT (STDMETHODCALLTYPE* DialWithResultAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3* This,
        HSTRING number,
        HSTRING displayName,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLineDialResult** operation);
    HRESULT (STDMETHODCALLTYPE* GetAllActivePhoneCalls)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCallsResult** result);
    HRESULT (STDMETHODCALLTYPE* GetAllActivePhoneCallsAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneCallsResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_DialWithResult(This, number, displayName, result) \
    ((This)->lpVtbl->DialWithResult(This, number, displayName, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_DialWithResultAsync(This, number, displayName, operation) \
    ((This)->lpVtbl->DialWithResultAsync(This, number, displayName, operation))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_GetAllActivePhoneCalls(This, result) \
    ((This)->lpVtbl->GetAllActivePhoneCalls(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_GetAllActivePhoneCallsAsync(This, operation) \
    ((This)->lpVtbl->GetAllActivePhoneCallsAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLine3_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineCellularDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLineCellularDetails
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineCellularDetails[] = L"Windows.ApplicationModel.Calls.IPhoneLineCellularDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SimState)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneSimState* value);
    HRESULT (STDMETHODCALLTYPE* get_SimSlotIndex)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_IsModemOn)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_RegistrationRejectCode)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* GetNetworkOperatorDisplayText)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineNetworkOperatorDisplayTextLocation location,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_get_SimState(This, value) \
    ((This)->lpVtbl->get_SimState(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_get_SimSlotIndex(This, value) \
    ((This)->lpVtbl->get_SimSlotIndex(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_get_IsModemOn(This, value) \
    ((This)->lpVtbl->get_IsModemOn(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_get_RegistrationRejectCode(This, value) \
    ((This)->lpVtbl->get_RegistrationRejectCode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_GetNetworkOperatorDisplayText(This, location, value) \
    ((This)->lpVtbl->GetNetworkOperatorDisplayText(This, location, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineCellularDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineConfiguration
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLineConfiguration
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineConfiguration[] = L"Windows.ApplicationModel.Calls.IPhoneLineConfiguration";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsVideoCallingEnabled)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedProperties)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration* This,
        __FIMapView_2_HSTRING_IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfigurationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_get_IsVideoCallingEnabled(This, value) \
    ((This)->lpVtbl->get_IsVideoCallingEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_get_ExtendedProperties(This, value) \
    ((This)->lpVtbl->get_ExtendedProperties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineDialResult
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLineDialResult
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineDialResult[] = L"Windows.ApplicationModel.Calls.IPhoneLineDialResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DialCallStatus)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneCallOperationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_DialedCall)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneCall** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_get_DialCallStatus(This, value) \
    ((This)->lpVtbl->get_DialCallStatus(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_get_DialedCall(This, value) \
    ((This)->lpVtbl->get_DialedCall(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineDialResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineStatics
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLine
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineStatics[] = L"Windows.ApplicationModel.Calls.IPhoneLineStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics* This,
        GUID lineId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CPhoneLine** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_FromIdAsync(This, lineId, result) \
    ((This)->lpVtbl->FromIdAsync(This, lineId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineTransportDevice
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLineTransportDevice
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineTransportDevice[] = L"Windows.ApplicationModel.Calls.IPhoneLineTransportDevice";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Transport)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineTransport* value);
    HRESULT (STDMETHODCALLTYPE* RequestAccessAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* This,
        __FIAsyncOperation_1_Windows__CDevices__CEnumeration__CDeviceAccessStatus** operation);
    HRESULT (STDMETHODCALLTYPE* RegisterApp)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* This);
    HRESULT (STDMETHODCALLTYPE* RegisterAppForUser)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* This,
        __x_ABI_CWindows_CSystem_CIUser* user);
    HRESULT (STDMETHODCALLTYPE* UnregisterApp)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* This);
    HRESULT (STDMETHODCALLTYPE* UnregisterAppForUser)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* This,
        __x_ABI_CWindows_CSystem_CIUser* user);
    HRESULT (STDMETHODCALLTYPE* IsRegistered)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Connect)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* ConnectAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice* This,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_get_Transport(This, value) \
    ((This)->lpVtbl->get_Transport(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_RequestAccessAsync(This, operation) \
    ((This)->lpVtbl->RequestAccessAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_RegisterApp(This) \
    ((This)->lpVtbl->RegisterApp(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_RegisterAppForUser(This, user) \
    ((This)->lpVtbl->RegisterAppForUser(This, user))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_UnregisterApp(This) \
    ((This)->lpVtbl->UnregisterApp(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_UnregisterAppForUser(This, user) \
    ((This)->lpVtbl->UnregisterAppForUser(This, user))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_IsRegistered(This, result) \
    ((This)->lpVtbl->IsRegistered(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_Connect(This, result) \
    ((This)->lpVtbl->Connect(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_ConnectAsync(This, operation) \
    ((This)->lpVtbl->ConnectAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineTransportDevice2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLineTransportDevice
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineTransportDevice2[] = L"Windows.ApplicationModel.Calls.IPhoneLineTransportDevice2";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AudioRoutingStatus)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CTransportDeviceAudioRoutingStatus* value);
    HRESULT (STDMETHODCALLTYPE* add_AudioRoutingStatusChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AudioRoutingStatusChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_InBandRingingEnabled)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* add_InBandRingingEnabledChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineTransportDevice_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_InBandRingingEnabledChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_get_AudioRoutingStatus(This, value) \
    ((This)->lpVtbl->get_AudioRoutingStatus(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_add_AudioRoutingStatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_AudioRoutingStatusChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_remove_AudioRoutingStatusChanged(This, token) \
    ((This)->lpVtbl->remove_AudioRoutingStatusChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_get_InBandRingingEnabled(This, value) \
    ((This)->lpVtbl->get_InBandRingingEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_add_InBandRingingEnabledChanged(This, handler, token) \
    ((This)->lpVtbl->add_InBandRingingEnabledChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_remove_InBandRingingEnabledChanged(This, token) \
    ((This)->lpVtbl->remove_InBandRingingEnabledChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineTransportDeviceStatics
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLineTransportDevice
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineTransportDeviceStatics[] = L"Windows.ApplicationModel.Calls.IPhoneLineTransportDeviceStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FromId)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics* This,
        HSTRING id,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDevice** result);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorForPhoneLineTransport)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineTransport transport,
        HSTRING* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_FromId(This, id, result) \
    ((This)->lpVtbl->FromId(This, id, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_GetDeviceSelector(This, result) \
    ((This)->lpVtbl->GetDeviceSelector(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_GetDeviceSelectorForPhoneLineTransport(This, transport, result) \
    ((This)->lpVtbl->GetDeviceSelectorForPhoneLineTransport(This, transport, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineTransportDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineWatcher
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLineWatcher
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineWatcher[] = L"Windows.ApplicationModel.Calls.IPhoneLineWatcher";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This);
    HRESULT (STDMETHODCALLTYPE* add_LineAdded)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_LineAdded)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_LineRemoved)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_LineRemoved)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_LineUpdated)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_Windows__CApplicationModel__CCalls__CPhoneLineWatcherEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_LineUpdated)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_EnumerationCompleted)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EnumerationCompleted)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Stopped)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CPhoneLineWatcher_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Stopped)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneLineWatcherStatus* status);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_add_LineAdded(This, handler, token) \
    ((This)->lpVtbl->add_LineAdded(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_remove_LineAdded(This, token) \
    ((This)->lpVtbl->remove_LineAdded(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_add_LineRemoved(This, handler, token) \
    ((This)->lpVtbl->add_LineRemoved(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_remove_LineRemoved(This, token) \
    ((This)->lpVtbl->remove_LineRemoved(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_add_LineUpdated(This, handler, token) \
    ((This)->lpVtbl->add_LineUpdated(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_remove_LineUpdated(This, token) \
    ((This)->lpVtbl->remove_LineUpdated(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_add_EnumerationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_EnumerationCompleted(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_remove_EnumerationCompleted(This, token) \
    ((This)->lpVtbl->remove_EnumerationCompleted(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_add_Stopped(This, handler, token) \
    ((This)->lpVtbl->add_Stopped(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_remove_Stopped(This, token) \
    ((This)->lpVtbl->remove_Stopped(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_get_Status(This, status) \
    ((This)->lpVtbl->get_Status(This, status))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneLineWatcherEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneLineWatcherEventArgs
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneLineWatcherEventArgs[] = L"Windows.ApplicationModel.Calls.IPhoneLineWatcherEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LineId)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs* This,
        GUID* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_get_LineId(This, value) \
    ((This)->lpVtbl->get_LineId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneLineWatcherEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IPhoneVoicemail
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.PhoneVoicemail
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IPhoneVoicemail[] = L"Windows.ApplicationModel.Calls.IPhoneVoicemail";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemailVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Number)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MessageCount)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CPhoneVoicemailType* value);
    HRESULT (STDMETHODCALLTYPE* DialVoicemailAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemailVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemailVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_get_Number(This, value) \
    ((This)->lpVtbl->get_Number(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_get_MessageCount(This, value) \
    ((This)->lpVtbl->get_MessageCount(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_DialVoicemailAsync(This, result) \
    ((This)->lpVtbl->DialVoicemailAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIPhoneVoicemail_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipCallCoordinator
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipCallCoordinator
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipCallCoordinator[] = L"Windows.ApplicationModel.Calls.IVoipCallCoordinator";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReserveCallResourcesAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This,
        HSTRING taskEntryPoint,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus** operation);
    HRESULT (STDMETHODCALLTYPE* add_MuteStateChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipCallCoordinator_Windows__CApplicationModel__CCalls__CMuteChangeEventArgs* muteChangeHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_MuteStateChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* RequestNewIncomingCall)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This,
        HSTRING context,
        HSTRING contactName,
        HSTRING contactNumber,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* contactImage,
        HSTRING serviceName,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* brandingImage,
        HSTRING callDetails,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* ringtone,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia media,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan ringTimeout,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall** call);
    HRESULT (STDMETHODCALLTYPE* RequestNewOutgoingCall)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This,
        HSTRING context,
        HSTRING contactName,
        HSTRING serviceName,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia media,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall** call);
    HRESULT (STDMETHODCALLTYPE* NotifyMuted)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This);
    HRESULT (STDMETHODCALLTYPE* NotifyUnmuted)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This);
    HRESULT (STDMETHODCALLTYPE* RequestOutgoingUpgradeToVideoCall)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This,
        GUID callUpgradeGuid,
        HSTRING context,
        HSTRING contactName,
        HSTRING serviceName,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall** call);
    HRESULT (STDMETHODCALLTYPE* RequestIncomingUpgradeToVideoCall)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This,
        HSTRING context,
        HSTRING contactName,
        HSTRING contactNumber,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* contactImage,
        HSTRING serviceName,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* brandingImage,
        HSTRING callDetails,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* ringtone,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan ringTimeout,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall** call);
    HRESULT (STDMETHODCALLTYPE* TerminateCellularCall)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This,
        GUID callUpgradeGuid);
    HRESULT (STDMETHODCALLTYPE* CancelUpgrade)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator* This,
        GUID callUpgradeGuid);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_ReserveCallResourcesAsync(This, taskEntryPoint, operation) \
    ((This)->lpVtbl->ReserveCallResourcesAsync(This, taskEntryPoint, operation))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_add_MuteStateChanged(This, muteChangeHandler, token) \
    ((This)->lpVtbl->add_MuteStateChanged(This, muteChangeHandler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_remove_MuteStateChanged(This, token) \
    ((This)->lpVtbl->remove_MuteStateChanged(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_RequestNewIncomingCall(This, context, contactName, contactNumber, contactImage, serviceName, brandingImage, callDetails, ringtone, media, ringTimeout, call) \
    ((This)->lpVtbl->RequestNewIncomingCall(This, context, contactName, contactNumber, contactImage, serviceName, brandingImage, callDetails, ringtone, media, ringTimeout, call))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_RequestNewOutgoingCall(This, context, contactName, serviceName, media, call) \
    ((This)->lpVtbl->RequestNewOutgoingCall(This, context, contactName, serviceName, media, call))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_NotifyMuted(This) \
    ((This)->lpVtbl->NotifyMuted(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_NotifyUnmuted(This) \
    ((This)->lpVtbl->NotifyUnmuted(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_RequestOutgoingUpgradeToVideoCall(This, callUpgradeGuid, context, contactName, serviceName, call) \
    ((This)->lpVtbl->RequestOutgoingUpgradeToVideoCall(This, callUpgradeGuid, context, contactName, serviceName, call))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_RequestIncomingUpgradeToVideoCall(This, context, contactName, contactNumber, contactImage, serviceName, brandingImage, callDetails, ringtone, ringTimeout, call) \
    ((This)->lpVtbl->RequestIncomingUpgradeToVideoCall(This, context, contactName, contactNumber, contactImage, serviceName, brandingImage, callDetails, ringtone, ringTimeout, call))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_TerminateCellularCall(This, callUpgradeGuid) \
    ((This)->lpVtbl->TerminateCellularCall(This, callUpgradeGuid))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_CancelUpgrade(This, callUpgradeGuid) \
    ((This)->lpVtbl->CancelUpgrade(This, callUpgradeGuid))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipCallCoordinator2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipCallCoordinator
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Calls.IVoipCallCoordinator
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipCallCoordinator2[] = L"Windows.ApplicationModel.Calls.IVoipCallCoordinator2";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetupNewAcceptedCall)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2* This,
        HSTRING context,
        HSTRING contactName,
        HSTRING contactNumber,
        HSTRING serviceName,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia media,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall** call);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_SetupNewAcceptedCall(This, context, contactName, contactNumber, serviceName, media, call) \
    ((This)->lpVtbl->SetupNewAcceptedCall(This, context, contactName, contactNumber, serviceName, media, call))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipCallCoordinator3
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipCallCoordinator
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Calls.IVoipCallCoordinator
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipCallCoordinator3[] = L"Windows.ApplicationModel.Calls.IVoipCallCoordinator3";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestNewAppInitiatedCall)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3* This,
        HSTRING context,
        HSTRING contactName,
        HSTRING contactNumber,
        HSTRING serviceName,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia media,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall** call);
    HRESULT (STDMETHODCALLTYPE* RequestNewIncomingCallWithContactRemoteId)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3* This,
        HSTRING context,
        HSTRING contactName,
        HSTRING contactNumber,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* contactImage,
        HSTRING serviceName,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* brandingImage,
        HSTRING callDetails,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* ringtone,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia media,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan ringTimeout,
        HSTRING contactRemoteId,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall** call);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_RequestNewAppInitiatedCall(This, context, contactName, contactNumber, serviceName, media, call) \
    ((This)->lpVtbl->RequestNewAppInitiatedCall(This, context, contactName, contactNumber, serviceName, media, call))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_RequestNewIncomingCallWithContactRemoteId(This, context, contactName, contactNumber, contactImage, serviceName, brandingImage, callDetails, ringtone, media, ringTimeout, contactRemoteId, call) \
    ((This)->lpVtbl->RequestNewIncomingCallWithContactRemoteId(This, context, contactName, contactNumber, contactImage, serviceName, brandingImage, callDetails, ringtone, media, ringTimeout, contactRemoteId, call))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator3_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipCallCoordinator4
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipCallCoordinator
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Calls.IVoipCallCoordinator
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipCallCoordinator4[] = L"Windows.ApplicationModel.Calls.IVoipCallCoordinator4";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReserveOneProcessCallResourcesAsync)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CCalls__CVoipPhoneCallResourceReservationStatus** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_ReserveOneProcessCallResourcesAsync(This, operation) \
    ((This)->lpVtbl->ReserveOneProcessCallResourcesAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator4_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipCallCoordinator5
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipCallCoordinator
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipCallCoordinator5[] = L"Windows.ApplicationModel.Calls.IVoipCallCoordinator5";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestNewIncomingCallWithOptions)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIIncomingVoipPhoneCallOptions* callOptions,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall** result);
    HRESULT (STDMETHODCALLTYPE* RequestNewOutgoingCallWithOptions)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIOutgoingVoipPhoneCallOptions* callOptions,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall** result);
    HRESULT (STDMETHODCALLTYPE* SetupNewAcceptedCallWithOptions)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIAcceptedVoipPhoneCallOptions* callOptions,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall** result);
    HRESULT (STDMETHODCALLTYPE* RequestNewAppInitiatedCallWithOptions)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIAppInitiatedVoipPhoneCallOptions* callOptions,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_RequestNewIncomingCallWithOptions(This, callOptions, result) \
    ((This)->lpVtbl->RequestNewIncomingCallWithOptions(This, callOptions, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_RequestNewOutgoingCallWithOptions(This, callOptions, result) \
    ((This)->lpVtbl->RequestNewOutgoingCallWithOptions(This, callOptions, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_SetupNewAcceptedCallWithOptions(This, callOptions, result) \
    ((This)->lpVtbl->SetupNewAcceptedCallWithOptions(This, callOptions, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_RequestNewAppInitiatedCallWithOptions(This, callOptions, result) \
    ((This)->lpVtbl->RequestNewAppInitiatedCallWithOptions(This, callOptions, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator5_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipCallCoordinatorStatics
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipCallCoordinator
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipCallCoordinatorStatics[] = L"Windows.ApplicationModel.Calls.IVoipCallCoordinatorStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics* This,
        __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinator** coordinator);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_GetDefault(This, coordinator) \
    ((This)->lpVtbl->GetDefault(This, coordinator))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipCallCoordinatorStatics2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipCallCoordinator
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipCallCoordinatorStatics2[] = L"Windows.ApplicationModel.Calls.IVoipCallCoordinatorStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsCallControlDeviceKindSupportedForAssociation)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipCallControlDeviceKind kind,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelectorForCallControl)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2* This,
        HSTRING* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_IsCallControlDeviceKindSupportedForAssociation(This, kind, result) \
    ((This)->lpVtbl->IsCallControlDeviceKindSupportedForAssociation(This, kind, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_GetDeviceSelectorForCallControl(This, result) \
    ((This)->lpVtbl->GetDeviceSelectorForCallControl(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipCallCoordinatorStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipPhoneCall
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipPhoneCall
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipPhoneCall[] = L"Windows.ApplicationModel.Calls.IVoipPhoneCall";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCallVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_EndRequested)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EndRequested)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_HoldRequested)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_HoldRequested)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ResumeRequested)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallStateChangeEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ResumeRequested)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_AnswerRequested)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallAnswerEventArgs* acceptHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AnswerRequested)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_RejectRequested)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CCalls__CVoipPhoneCall_Windows__CApplicationModel__CCalls__CCallRejectEventArgs* rejectHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RejectRequested)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* NotifyCallHeld)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This);
    HRESULT (STDMETHODCALLTYPE* NotifyCallActive)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This);
    HRESULT (STDMETHODCALLTYPE* NotifyCallEnded)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This);
    HRESULT (STDMETHODCALLTYPE* get_ContactName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ContactName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_StartTime)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* put_StartTime)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* get_CallMedia)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia* value);
    HRESULT (STDMETHODCALLTYPE* put_CallMedia)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia value);
    HRESULT (STDMETHODCALLTYPE* NotifyCallReady)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCallVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCallVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_add_EndRequested(This, handler, token) \
    ((This)->lpVtbl->add_EndRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_remove_EndRequested(This, token) \
    ((This)->lpVtbl->remove_EndRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_add_HoldRequested(This, handler, token) \
    ((This)->lpVtbl->add_HoldRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_remove_HoldRequested(This, token) \
    ((This)->lpVtbl->remove_HoldRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_add_ResumeRequested(This, handler, token) \
    ((This)->lpVtbl->add_ResumeRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_remove_ResumeRequested(This, token) \
    ((This)->lpVtbl->remove_ResumeRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_add_AnswerRequested(This, acceptHandler, token) \
    ((This)->lpVtbl->add_AnswerRequested(This, acceptHandler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_remove_AnswerRequested(This, token) \
    ((This)->lpVtbl->remove_AnswerRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_add_RejectRequested(This, rejectHandler, token) \
    ((This)->lpVtbl->add_RejectRequested(This, rejectHandler, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_remove_RejectRequested(This, token) \
    ((This)->lpVtbl->remove_RejectRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_NotifyCallHeld(This) \
    ((This)->lpVtbl->NotifyCallHeld(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_NotifyCallActive(This) \
    ((This)->lpVtbl->NotifyCallActive(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_NotifyCallEnded(This) \
    ((This)->lpVtbl->NotifyCallEnded(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_get_ContactName(This, value) \
    ((This)->lpVtbl->get_ContactName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_put_ContactName(This, value) \
    ((This)->lpVtbl->put_ContactName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_get_StartTime(This, value) \
    ((This)->lpVtbl->get_StartTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_put_StartTime(This, value) \
    ((This)->lpVtbl->put_StartTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_get_CallMedia(This, value) \
    ((This)->lpVtbl->get_CallMedia(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_put_CallMedia(This, value) \
    ((This)->lpVtbl->put_CallMedia(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_NotifyCallReady(This) \
    ((This)->lpVtbl->NotifyCallReady(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipPhoneCall2
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipPhoneCall
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Calls.IVoipPhoneCall
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipPhoneCall2[] = L"Windows.ApplicationModel.Calls.IVoipPhoneCall2";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryShowAppUI)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_TryShowAppUI(This) \
    ((This)->lpVtbl->TryShowAppUI(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipPhoneCall3
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipPhoneCall
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Calls.IVoipPhoneCall2
 *     Windows.ApplicationModel.Calls.IVoipPhoneCall
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipPhoneCall3[] = L"Windows.ApplicationModel.Calls.IVoipPhoneCall3";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* NotifyCallAccepted)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CVoipPhoneCallMedia media);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_NotifyCallAccepted(This, media) \
    ((This)->lpVtbl->NotifyCallAccepted(This, media))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall3_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.IVoipPhoneCall4
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.VoipPhoneCall
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_IVoipPhoneCall4[] = L"Windows.ApplicationModel.Calls.IVoipPhoneCall4";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsUsingAssociatedDevicesList)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* NotifyCallActiveOnDevices)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4* This,
        __FIIterable_1_HSTRING* associatedDeviceIds);
    HRESULT (STDMETHODCALLTYPE* AddAssociatedCallControlDevice)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4* This,
        HSTRING deviceId);
    HRESULT (STDMETHODCALLTYPE* RemoveAssociatedCallControlDevice)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4* This,
        HSTRING deviceId);
    HRESULT (STDMETHODCALLTYPE* SetAssociatedCallControlDevices)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4* This,
        __FIIterable_1_HSTRING* associatedDeviceIds);
    HRESULT (STDMETHODCALLTYPE* GetAssociatedCallControlDevices)(__x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4* This,
        __FIVectorView_1_HSTRING** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_get_IsUsingAssociatedDevicesList(This, value) \
    ((This)->lpVtbl->get_IsUsingAssociatedDevicesList(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_NotifyCallActiveOnDevices(This, associatedDeviceIds) \
    ((This)->lpVtbl->NotifyCallActiveOnDevices(This, associatedDeviceIds))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_AddAssociatedCallControlDevice(This, deviceId) \
    ((This)->lpVtbl->AddAssociatedCallControlDevice(This, deviceId))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_RemoveAssociatedCallControlDevice(This, deviceId) \
    ((This)->lpVtbl->RemoveAssociatedCallControlDevice(This, deviceId))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_SetAssociatedCallControlDevices(This, associatedDeviceIds) \
    ((This)->lpVtbl->SetAssociatedCallControlDevices(This, associatedDeviceIds))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_GetAssociatedCallControlDevices(This, result) \
    ((This)->lpVtbl->GetAssociatedCallControlDevices(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CIVoipPhoneCall4_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Calls.AcceptedVoipPhoneCallOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *   Type can be activated via the Windows.ApplicationModel.Calls.IAcceptedVoipPhoneCallOptionsFactory interface starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IAcceptedVoipPhoneCallOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_AcceptedVoipPhoneCallOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_AcceptedVoipPhoneCallOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_AcceptedVoipPhoneCallOptions[] = L"Windows.ApplicationModel.Calls.AcceptedVoipPhoneCallOptions";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Calls.AppInitiatedVoipPhoneCallOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Calls.IAppInitiatedVoipPhoneCallOptionsFactory interface starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IAppInitiatedVoipPhoneCallOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_AppInitiatedVoipPhoneCallOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_AppInitiatedVoipPhoneCallOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_AppInitiatedVoipPhoneCallOptions[] = L"Windows.ApplicationModel.Calls.AppInitiatedVoipPhoneCallOptions";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Calls.CallAnswerEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.ICallAnswerEventArgs ** Default Interface **
 *    Windows.ApplicationModel.Calls.ICallAnswerEventArgs2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_CallAnswerEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_CallAnswerEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_CallAnswerEventArgs[] = L"Windows.ApplicationModel.Calls.CallAnswerEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.CallRejectEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.ICallRejectEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_CallRejectEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_CallRejectEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_CallRejectEventArgs[] = L"Windows.ApplicationModel.Calls.CallRejectEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.CallStateChangeEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.ICallStateChangeEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_CallStateChangeEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_CallStateChangeEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_CallStateChangeEventArgs[] = L"Windows.ApplicationModel.Calls.CallStateChangeEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.IncomingVoipPhoneCallOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Calls.IIncomingVoipPhoneCallOptionsFactory interface starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IIncomingVoipPhoneCallOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_IncomingVoipPhoneCallOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_IncomingVoipPhoneCallOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_IncomingVoipPhoneCallOptions[] = L"Windows.ApplicationModel.Calls.IncomingVoipPhoneCallOptions";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Calls.LockScreenCallEndCallDeferral
 *
 * Introduced to Windows.ApplicationModel.Calls.LockScreenCallContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.ILockScreenCallEndCallDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_LockScreenCallEndCallDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_LockScreenCallEndCallDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_LockScreenCallEndCallDeferral[] = L"Windows.ApplicationModel.Calls.LockScreenCallEndCallDeferral";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.LockScreenCallEndRequestedEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.LockScreenCallContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.ILockScreenCallEndRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_LockScreenCallEndRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_LockScreenCallEndRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_LockScreenCallEndRequestedEventArgs[] = L"Windows.ApplicationModel.Calls.LockScreenCallEndRequestedEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.LockScreenCallUI
 *
 * Introduced to Windows.ApplicationModel.Calls.LockScreenCallContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.ILockScreenCallUI ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_LockScreenCallUI_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_LockScreenCallUI_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_LockScreenCallUI[] = L"Windows.ApplicationModel.Calls.LockScreenCallUI";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_LOCKSCREENCALLCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.MuteChangeEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IMuteChangeEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_MuteChangeEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_MuteChangeEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_MuteChangeEventArgs[] = L"Windows.ApplicationModel.Calls.MuteChangeEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.OutgoingVoipPhoneCallOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 5.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Calls.IOutgoingVoipPhoneCallOptionsFactory interface starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *   Type can be activated via RoActivateInstance starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IOutgoingVoipPhoneCallOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_OutgoingVoipPhoneCallOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_OutgoingVoipPhoneCallOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_OutgoingVoipPhoneCallOptions[] = L"Windows.ApplicationModel.Calls.OutgoingVoipPhoneCallOptions";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCall
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneCallStatics interface starting with version 6.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCall ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCall_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCall_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCall[] = L"Windows.ApplicationModel.Calls.PhoneCall";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallBlocking
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneCallBlockingStatics interface starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallBlocking_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallBlocking_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallBlocking[] = L"Windows.ApplicationModel.Calls.PhoneCallBlocking";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallHistoryEntry
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallHistoryEntry ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryEntry_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryEntry_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallHistoryEntry[] = L"Windows.ApplicationModel.Calls.PhoneCallHistoryEntry";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryAddressFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryAddress ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryAddress_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryAddress_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryAddress[] = L"Windows.ApplicationModel.Calls.PhoneCallHistoryEntryAddress";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryQueryOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryQueryOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryQueryOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryQueryOptions[] = L"Windows.ApplicationModel.Calls.PhoneCallHistoryEntryQueryOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallHistoryEntryReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallHistoryEntryReader ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryReader_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallHistoryEntryReader[] = L"Windows.ApplicationModel.Calls.PhoneCallHistoryEntryReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallHistoryManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerStatics2 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallHistoryManager[] = L"Windows.ApplicationModel.Calls.PhoneCallHistoryManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallHistoryManagerForUser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallHistoryManagerForUser ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryManagerForUser_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryManagerForUser_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallHistoryManagerForUser[] = L"Windows.ApplicationModel.Calls.PhoneCallHistoryManagerForUser";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallHistoryStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallHistoryStore ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryStore_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallHistoryStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallHistoryStore[] = L"Windows.ApplicationModel.Calls.PhoneCallHistoryStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallInfo
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallInfo[] = L"Windows.ApplicationModel.Calls.PhoneCallInfo";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallManager
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneCallManagerStatics interface starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneCallManagerStatics2 interface starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallManager[] = L"Windows.ApplicationModel.Calls.PhoneCallManager";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallStore
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallStore ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallStore_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallStore[] = L"Windows.ApplicationModel.Calls.PhoneCallStore";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallVideoCapabilities ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallVideoCapabilities_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallVideoCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallVideoCapabilities[] = L"Windows.ApplicationModel.Calls.PhoneCallVideoCapabilities";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallVideoCapabilitiesManager
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneCallVideoCapabilitiesManagerStatics interface starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallVideoCapabilitiesManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallVideoCapabilitiesManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallVideoCapabilitiesManager[] = L"Windows.ApplicationModel.Calls.PhoneCallVideoCapabilitiesManager";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneCallsResult
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneCallsResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallsResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneCallsResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneCallsResult[] = L"Windows.ApplicationModel.Calls.PhoneCallsResult";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneDialOptions
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneDialOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneDialOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneDialOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneDialOptions[] = L"Windows.ApplicationModel.Calls.PhoneDialOptions";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneLine
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneLineStatics interface starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneLine ** Default Interface **
 *    Windows.ApplicationModel.Calls.IPhoneLine2
 *    Windows.ApplicationModel.Calls.IPhoneLine3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLine_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLine_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneLine[] = L"Windows.ApplicationModel.Calls.PhoneLine";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneLineCellularDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneLineCellularDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineCellularDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineCellularDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneLineCellularDetails[] = L"Windows.ApplicationModel.Calls.PhoneLineCellularDetails";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneLineConfiguration
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneLineConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineConfiguration_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneLineConfiguration[] = L"Windows.ApplicationModel.Calls.PhoneLineConfiguration";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneLineDialResult
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneLineDialResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineDialResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineDialResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneLineDialResult[] = L"Windows.ApplicationModel.Calls.PhoneLineDialResult";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneLineTransportDevice
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 5.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IPhoneLineTransportDeviceStatics interface starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsPhoneContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneLineTransportDevice ** Default Interface **
 *    Windows.ApplicationModel.Calls.IPhoneLineTransportDevice2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineTransportDevice_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineTransportDevice_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneLineTransportDevice[] = L"Windows.ApplicationModel.Calls.PhoneLineTransportDevice";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneLineWatcher
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneLineWatcher ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineWatcher_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineWatcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneLineWatcher[] = L"Windows.ApplicationModel.Calls.PhoneLineWatcher";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneLineWatcherEventArgs
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneLineWatcherEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineWatcherEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneLineWatcherEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneLineWatcherEventArgs[] = L"Windows.ApplicationModel.Calls.PhoneLineWatcherEventArgs";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.PhoneVoicemail
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsPhoneContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IPhoneVoicemail ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneVoicemail_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_PhoneVoicemail_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_PhoneVoicemail[] = L"Windows.ApplicationModel.Calls.PhoneVoicemail";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.VoipCallCoordinator
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IVoipCallCoordinatorStatics interface starting with version 1.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Calls.IVoipCallCoordinatorStatics2 interface starting with version 5.0 of the Windows.ApplicationModel.Calls.CallsVoipContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IVoipCallCoordinator ** Default Interface **
 *    Windows.ApplicationModel.Calls.IVoipCallCoordinator2
 *    Windows.ApplicationModel.Calls.IVoipCallCoordinator3
 *    Windows.ApplicationModel.Calls.IVoipCallCoordinator4
 *    Windows.ApplicationModel.Calls.IVoipCallCoordinator5
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_VoipCallCoordinator_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_VoipCallCoordinator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_VoipCallCoordinator[] = L"Windows.ApplicationModel.Calls.VoipCallCoordinator";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Calls.VoipPhoneCall
 *
 * Introduced to Windows.ApplicationModel.Calls.CallsVoipContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.IVoipPhoneCall ** Default Interface **
 *    Windows.ApplicationModel.Calls.IVoipPhoneCall2
 *    Windows.ApplicationModel.Calls.IVoipPhoneCall3
 *    Windows.ApplicationModel.Calls.IVoipPhoneCall4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_VoipPhoneCall_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_VoipPhoneCall_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_VoipPhoneCall[] = L"Windows.ApplicationModel.Calls.VoipPhoneCall";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_CALLSVOIPCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Ecalls_p_h__

#endif // __windows2Eapplicationmodel2Ecalls_h__
