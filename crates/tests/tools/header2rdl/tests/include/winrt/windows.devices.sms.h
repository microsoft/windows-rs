
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
#ifndef __windows2Edevices2Esms_h__
#define __windows2Edevices2Esms_h__
#ifndef __windows2Edevices2Esms_p_h__
#define __windows2Edevices2Esms_p_h__


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
#if !defined(WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION)
#define WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION)

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
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsDeviceStatusChangedEventHandler;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler ABI::Windows::Devices::Sms::ISmsDeviceStatusChangedEventHandler

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsMessageReceivedEventHandler;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler ABI::Windows::Devices::Sms::ISmsMessageReceivedEventHandler

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsAppMessage;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage ABI::Windows::Devices::Sms::ISmsAppMessage

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsBinaryMessage;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage ABI::Windows::Devices::Sms::ISmsBinaryMessage

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsBroadcastMessage;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage ABI::Windows::Devices::Sms::ISmsBroadcastMessage

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsDevice;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice ABI::Windows::Devices::Sms::ISmsDevice

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsDevice2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDevice2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsDevice2;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2 ABI::Windows::Devices::Sms::ISmsDevice2

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsDevice2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsDevice2Statics;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics ABI::Windows::Devices::Sms::ISmsDevice2Statics

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsDeviceMessageStore;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore ABI::Windows::Devices::Sms::ISmsDeviceMessageStore

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsDeviceStatics;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics ABI::Windows::Devices::Sms::ISmsDeviceStatics

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsDeviceStatics2;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2 ABI::Windows::Devices::Sms::ISmsDeviceStatics2

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsFilterRule;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule ABI::Windows::Devices::Sms::ISmsFilterRule

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsFilterRuleFactory;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory ABI::Windows::Devices::Sms::ISmsFilterRuleFactory

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsFilterRules;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRules ABI::Windows::Devices::Sms::ISmsFilterRules

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsFilterRulesFactory;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory ABI::Windows::Devices::Sms::ISmsFilterRulesFactory

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsMessage;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsMessage ABI::Windows::Devices::Sms::ISmsMessage

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsMessageBase;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase ABI::Windows::Devices::Sms::ISmsMessageBase

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsMessageReceivedEventArgs;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs ABI::Windows::Devices::Sms::ISmsMessageReceivedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsMessageReceivedTriggerDetails;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails ABI::Windows::Devices::Sms::ISmsMessageReceivedTriggerDetails

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsMessageRegistration;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration ABI::Windows::Devices::Sms::ISmsMessageRegistration

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsMessageRegistrationStatics;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics ABI::Windows::Devices::Sms::ISmsMessageRegistrationStatics

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsReceivedEventDetails;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails ABI::Windows::Devices::Sms::ISmsReceivedEventDetails

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsReceivedEventDetails2;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2 ABI::Windows::Devices::Sms::ISmsReceivedEventDetails2

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsSendMessageResult;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult ABI::Windows::Devices::Sms::ISmsSendMessageResult

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsStatusMessage;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage ABI::Windows::Devices::Sms::ISmsStatusMessage

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsTextMessage;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage ABI::Windows::Devices::Sms::ISmsTextMessage

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsTextMessage2;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2 ABI::Windows::Devices::Sms::ISmsTextMessage2

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsTextMessageStatics;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics ABI::Windows::Devices::Sms::ISmsTextMessageStatics

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsVoicemailMessage;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage ABI::Windows::Devices::Sms::ISmsVoicemailMessage

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                interface ISmsWapMessage;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage ABI::Windows::Devices::Sms::ISmsWapMessage

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("abf9b459-48c3-5eac-9749-4c6db4d507e6"))
IAsyncOperation<ABI::Windows::Devices::Sms::ISmsMessage*> : IAsyncOperation_impl<ABI::Windows::Devices::Sms::ISmsMessage*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Sms.ISmsMessage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Sms::ISmsMessage*> __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_t;
#define __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_USE */

#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4e6c4c86-ebe6-55d9-adc0-fecc38c82aa2"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Sms::ISmsMessage*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Devices::Sms::ISmsMessage*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Sms.ISmsMessage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Sms::ISmsMessage*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage_USE */

#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsDevice;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ab710de1-fcbb-5bd6-9f2f-285fa9fb44e8"))
IAsyncOperation<ABI::Windows::Devices::Sms::SmsDevice*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sms::SmsDevice*, ABI::Windows::Devices::Sms::ISmsDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Sms.SmsDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Sms::SmsDevice*> __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_t;
#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_USE */

#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("44aa5484-43d0-5893-a4ee-7db00113ae60"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Sms::SmsDevice*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sms::SmsDevice*, ABI::Windows::Devices::Sms::ISmsDevice*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Sms.SmsDevice>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Sms::SmsDevice*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice_USE */

#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsSendMessageResult;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fc0a0b0f-4dcc-5257-bc61-3435e302ce1f"))
IAsyncOperation<ABI::Windows::Devices::Sms::SmsSendMessageResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sms::SmsSendMessageResult*, ABI::Windows::Devices::Sms::ISmsSendMessageResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.Sms.SmsSendMessageResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::Sms::SmsSendMessageResult*> __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_t;
#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c7d5c6fe-9206-5eb1-abc1-c1bc21804eeb"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::Sms::SmsSendMessageResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sms::SmsSendMessageResult*, ABI::Windows::Devices::Sms::ISmsSendMessageResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.Sms.SmsSendMessageResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::Sms::SmsSendMessageResult*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CSms__CISmsMessage_USE
#define DEF___FIIterator_1_Windows__CDevices__CSms__CISmsMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("eed04f5c-b2b2-5c83-8b13-c78af6ca3a18"))
IIterator<ABI::Windows::Devices::Sms::ISmsMessage*> : IIterator_impl<ABI::Windows::Devices::Sms::ISmsMessage*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Sms.ISmsMessage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Sms::ISmsMessage*> __FIIterator_1_Windows__CDevices__CSms__CISmsMessage_t;
#define __FIIterator_1_Windows__CDevices__CSms__CISmsMessage ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CSms__CISmsMessage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CSms__CISmsMessage_USE */

#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CSms__CISmsMessage_USE
#define DEF___FIIterable_1_Windows__CDevices__CSms__CISmsMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ecabfd70-9601-5e38-83cf-b1046022a244"))
IIterable<ABI::Windows::Devices::Sms::ISmsMessage*> : IIterable_impl<ABI::Windows::Devices::Sms::ISmsMessage*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Sms.ISmsMessage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Sms::ISmsMessage*> __FIIterable_1_Windows__CDevices__CSms__CISmsMessage_t;
#define __FIIterable_1_Windows__CDevices__CSms__CISmsMessage ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CSms__CISmsMessage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CSms__CISmsMessage_USE */

#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_USE
#define DEF___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d3acc5b1-6f85-507e-b40a-6950749b426f"))
IVectorView<ABI::Windows::Devices::Sms::ISmsMessage*> : IVectorView_impl<ABI::Windows::Devices::Sms::ISmsMessage*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Sms.ISmsMessage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Sms::ISmsMessage*> __FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_t;
#define __FIVectorView_1_Windows__CDevices__CSms__CISmsMessage ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_USE */

#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c0454cfc-2f2f-5e0c-8de9-58b9e82a03ba"))
IAsyncOperationWithProgressCompletedHandler<__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage*, int> : IAsyncOperationWithProgressCompletedHandler_impl<__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage*, int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Sms.ISmsMessage>, Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage*, int> __FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_USE */

#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_USE
#define DEF___FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("12f85589-415d-5b5d-b0d0-fda3b0295adc"))
IAsyncOperationWithProgress<__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage*, int> : IAsyncOperationWithProgress_impl<__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage*, int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Sms.ISmsMessage>, Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage*, int> __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_t;
#define __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_USE */

#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_USE
#define DEF___FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3f9d1255-ebf8-569f-91c3-49740d5944ce"))
IAsyncOperationProgressHandler<__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage*, int> : IAsyncOperationProgressHandler_impl<__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage*, int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Sms.ISmsMessage>, Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage*, int> __FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_t;
#define __FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_USE */

#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIIterator_1_int_USE
#define DEF___FIIterator_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bfea7f78-50c2-5f1d-a6ea-9e978d2699ff"))
IIterator<int> : IIterator_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<int> __FIIterator_1_int_t;
#define __FIIterator_1_int ABI::Windows::Foundation::Collections::__FIIterator_1_int_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_int_USE */



#ifndef DEF___FIIterable_1_int_USE
#define DEF___FIIterable_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("81a643fb-f51c-5565-83c4-f96425777b66"))
IIterable<int> : IIterable_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<int> __FIIterable_1_int_t;
#define __FIIterable_1_int ABI::Windows::Foundation::Collections::__FIIterable_1_int_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_int_USE */



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


#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_USE
#define DEF___FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("13e60d89-ea0a-5b01-9c2f-0e5b435058e0"))
IIterator<ABI::Windows::Devices::Sms::ISmsBinaryMessage*> : IIterator_impl<ABI::Windows::Devices::Sms::ISmsBinaryMessage*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Sms.ISmsBinaryMessage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Sms::ISmsBinaryMessage*> __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_t;
#define __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_USE */

#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage_USE
#define DEF___FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5678a6a5-4d5a-51c2-a133-4b83bf25d987"))
IIterable<ABI::Windows::Devices::Sms::ISmsBinaryMessage*> : IIterable_impl<ABI::Windows::Devices::Sms::ISmsBinaryMessage*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Sms.ISmsBinaryMessage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Sms::ISmsBinaryMessage*> __FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage_t;
#define __FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage_USE */

#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                typedef enum SmsBroadcastType : int SmsBroadcastType;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_USE
#define DEF___FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6448ddea-c1cd-5143-a422-5fe4f008cc92"))
IIterator<enum ABI::Windows::Devices::Sms::SmsBroadcastType> : IIterator_impl<enum ABI::Windows::Devices::Sms::SmsBroadcastType>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Sms.SmsBroadcastType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Devices::Sms::SmsBroadcastType> __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_t;
#define __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType_USE
#define DEF___FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("12276b75-173e-514b-98f0-8a7927a9206c"))
IIterable<enum ABI::Windows::Devices::Sms::SmsBroadcastType> : IIterable_impl<enum ABI::Windows::Devices::Sms::SmsBroadcastType>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Sms.SmsBroadcastType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Devices::Sms::SmsBroadcastType> __FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType_t;
#define __FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsFilterRule;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_USE
#define DEF___FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fec7cae6-97f5-5a19-b32a-d8f0ba276f34"))
IIterator<ABI::Windows::Devices::Sms::SmsFilterRule*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sms::SmsFilterRule*, ABI::Windows::Devices::Sms::ISmsFilterRule*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Sms.SmsFilterRule>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Sms::SmsFilterRule*> __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_t;
#define __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule_USE
#define DEF___FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("03ed8267-9c90-5260-8bc0-6c3e3306c964"))
IIterable<ABI::Windows::Devices::Sms::SmsFilterRule*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sms::SmsFilterRule*, ABI::Windows::Devices::Sms::ISmsFilterRule*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Sms.SmsFilterRule>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Sms::SmsFilterRule*> __FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule_t;
#define __FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsMessageRegistration;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_USE
#define DEF___FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ee1a0675-f3c9-5c12-93e0-f2b01bdce611"))
IIterator<ABI::Windows::Devices::Sms::SmsMessageRegistration*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sms::SmsMessageRegistration*, ABI::Windows::Devices::Sms::ISmsMessageRegistration*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Devices.Sms.SmsMessageRegistration>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Devices::Sms::SmsMessageRegistration*> __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_t;
#define __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration_USE
#define DEF___FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f836fa0a-770d-5e8f-8664-01c43f959eea"))
IIterable<ABI::Windows::Devices::Sms::SmsMessageRegistration*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sms::SmsMessageRegistration*, ABI::Windows::Devices::Sms::ISmsMessageRegistration*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Devices.Sms.SmsMessageRegistration>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Devices::Sms::SmsMessageRegistration*> __FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration_t;
#define __FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration_USE */

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



#ifndef DEF___FIVectorView_1_int_USE
#define DEF___FIVectorView_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8d720cdf-3934-5d3f-9a55-40e8063b086a"))
IVectorView<int> : IVectorView_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<int> __FIVectorView_1_int_t;
#define __FIVectorView_1_int ABI::Windows::Foundation::Collections::__FIVectorView_1_int_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_int_USE */



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


#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_USE
#define DEF___FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6ea176ea-99ea-5c79-876a-f4c437b83df6"))
IVectorView<ABI::Windows::Devices::Sms::ISmsBinaryMessage*> : IVectorView_impl<ABI::Windows::Devices::Sms::ISmsBinaryMessage*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Sms.ISmsBinaryMessage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Sms::ISmsBinaryMessage*> __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_t;
#define __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_USE */

#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_USE
#define DEF___FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4063e791-da2d-5e4c-9113-5b6ba0a7c595"))
IVectorView<enum ABI::Windows::Devices::Sms::SmsBroadcastType> : IVectorView_impl<enum ABI::Windows::Devices::Sms::SmsBroadcastType>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Sms.SmsBroadcastType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Devices::Sms::SmsBroadcastType> __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_t;
#define __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_USE
#define DEF___FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f3ed4299-de36-5d82-993f-35fc677d8b72"))
IVectorView<ABI::Windows::Devices::Sms::SmsFilterRule*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sms::SmsFilterRule*, ABI::Windows::Devices::Sms::ISmsFilterRule*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Sms.SmsFilterRule>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Sms::SmsFilterRule*> __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_t;
#define __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_USE
#define DEF___FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("23aaa815-24ef-57c4-b1bc-94d62c0a59a3"))
IVectorView<ABI::Windows::Devices::Sms::SmsMessageRegistration*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sms::SmsMessageRegistration*, ABI::Windows::Devices::Sms::ISmsMessageRegistration*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Sms.SmsMessageRegistration>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Devices::Sms::SmsMessageRegistration*> __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_t;
#define __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIVector_1_int_USE
#define DEF___FIVector_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b939af5b-b45d-5489-9149-61442c1905fe"))
IVector<int> : IVector_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<int> __FIVector_1_int_t;
#define __FIVector_1_int ABI::Windows::Foundation::Collections::__FIVector_1_int_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_int_USE */



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

#ifndef DEF___FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_USE
#define DEF___FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d76bef0b-1358-5895-bd42-f17f6f33eed1"))
IVector<enum ABI::Windows::Devices::Sms::SmsBroadcastType> : IVector_impl<enum ABI::Windows::Devices::Sms::SmsBroadcastType>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Devices.Sms.SmsBroadcastType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<enum ABI::Windows::Devices::Sms::SmsBroadcastType> __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_t;
#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_USE
#define DEF___FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8e72fa52-4867-5696-b4d9-4ca23f19e177"))
IVector<ABI::Windows::Devices::Sms::SmsFilterRule*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sms::SmsFilterRule*, ABI::Windows::Devices::Sms::ISmsFilterRule*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Devices.Sms.SmsFilterRule>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Devices::Sms::SmsFilterRule*> __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_t;
#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIReference_1_int_USE
#define DEF___FIReference_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("548cefbd-bc8a-5fa0-8df2-957440fc8bf4"))
IReference<int> : IReference_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<int> __FIReference_1_int_t;
#define __FIReference_1_int ABI::Windows::Foundation::__FIReference_1_int_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_int_USE */


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsDevice2;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3f3808e6-3dee-57a6-a88d-bacfb066c7fb"))
ITypedEventHandler<ABI::Windows::Devices::Sms::SmsDevice2*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sms::SmsDevice2*, ABI::Windows::Devices::Sms::ISmsDevice2*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Sms.SmsDevice2, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Sms::SmsDevice2*, IInspectable*> __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsMessageReceivedTriggerDetails;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("33f985c7-dcfa-531f-9cce-ee5e6c26b1e6"))
ITypedEventHandler<ABI::Windows::Devices::Sms::SmsMessageRegistration*, ABI::Windows::Devices::Sms::SmsMessageReceivedTriggerDetails*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sms::SmsMessageRegistration*, ABI::Windows::Devices::Sms::ISmsMessageRegistration*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::Sms::SmsMessageReceivedTriggerDetails*, ABI::Windows::Devices::Sms::ISmsMessageReceivedTriggerDetails*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.Sms.SmsMessageRegistration, Windows.Devices.Sms.SmsMessageReceivedTriggerDetails>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::Sms::SmsMessageRegistration*, ABI::Windows::Devices::Sms::SmsMessageReceivedTriggerDetails*> __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails_t;
#define __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IBuffer;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIBuffer ABI::Windows::Storage::Streams::IBuffer

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                typedef enum CellularClass : int CellularClass;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                typedef enum SmsDataFormat : int SmsDataFormat;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                typedef enum SmsDeviceStatus : int SmsDeviceStatus;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                typedef enum SmsEncoding : int SmsEncoding;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                typedef enum SmsFilterActionType : int SmsFilterActionType;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                typedef enum SmsGeographicalScope : int SmsGeographicalScope;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                typedef enum SmsMessageClass : int SmsMessageClass;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                typedef enum SmsMessageFilter : int SmsMessageFilter;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                typedef enum SmsMessageType : int SmsMessageType;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                typedef enum SmsModemErrorCode : int SmsModemErrorCode;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                typedef struct SmsEncodedLength SmsEncodedLength;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SendSmsMessageOperation;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsAppMessage;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsBinaryMessage;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsBroadcastMessage;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsDeviceMessageStore;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsFilterRules;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsMessageReceivedEventArgs;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsStatusMessage;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsTextMessage;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsTextMessage2;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsVoicemailMessage;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                class SmsWapMessage;
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.Sms.CellularClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                enum CellularClass : int
                {
                    CellularClass_None = 0,
                    CellularClass_Gsm = 1,
                    CellularClass_Cdma = 2,
                };
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsBroadcastType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                enum SmsBroadcastType : int
                {
                    SmsBroadcastType_Other = 0,
                    SmsBroadcastType_CmasPresidential = 1,
                    SmsBroadcastType_CmasExtreme = 2,
                    SmsBroadcastType_CmasSevere = 3,
                    SmsBroadcastType_CmasAmber = 4,
                    SmsBroadcastType_CmasTest = 5,
                    SmsBroadcastType_EUAlert1 = 6,
                    SmsBroadcastType_EUAlert2 = 7,
                    SmsBroadcastType_EUAlert3 = 8,
                    SmsBroadcastType_EUAlertAmber = 9,
                    SmsBroadcastType_EUAlertInfo = 10,
                    SmsBroadcastType_EtwsEarthquake = 11,
                    SmsBroadcastType_EtwsTsunami = 12,
                    SmsBroadcastType_EtwsTsunamiAndEarthquake = 13,
                    SmsBroadcastType_LatAlertLocal = 14,
                };
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsDataFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                enum SmsDataFormat : int
                {
                    SmsDataFormat_Unknown = 0,
                    SmsDataFormat_CdmaSubmit = 1,
                    SmsDataFormat_GsmSubmit = 2,
                    SmsDataFormat_CdmaDeliver = 3,
                    SmsDataFormat_GsmDeliver = 4,
                };
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsDeviceStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                enum SmsDeviceStatus : int
                {
                    SmsDeviceStatus_Off = 0,
                    SmsDeviceStatus_Ready = 1,
                    SmsDeviceStatus_SimNotInserted = 2,
                    SmsDeviceStatus_BadSim = 3,
                    SmsDeviceStatus_DeviceFailure = 4,
                    SmsDeviceStatus_SubscriptionNotActivated = 5,
                    SmsDeviceStatus_DeviceLocked = 6,
                    SmsDeviceStatus_DeviceBlocked = 7,
                };
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsEncoding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                enum SmsEncoding : int
                {
                    SmsEncoding_Unknown = 0,
                    SmsEncoding_Optimal = 1,
                    SmsEncoding_SevenBitAscii = 2,
                    SmsEncoding_Unicode = 3,
                    SmsEncoding_GsmSevenBit = 4,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SmsEncoding_EightBit = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SmsEncoding_Latin = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SmsEncoding_Korean = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SmsEncoding_IA5 = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SmsEncoding_ShiftJis = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SmsEncoding_LatinHebrew = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsFilterActionType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                enum SmsFilterActionType : int
                {
                    SmsFilterActionType_AcceptImmediately = 0,
                    SmsFilterActionType_Drop = 1,
                    SmsFilterActionType_Peek = 2,
                    SmsFilterActionType_Accept = 3,
                };
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsGeographicalScope
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                enum SmsGeographicalScope : int
                {
                    SmsGeographicalScope_None = 0,
                    SmsGeographicalScope_CellWithImmediateDisplay = 1,
                    SmsGeographicalScope_LocationArea = 2,
                    SmsGeographicalScope_Plmn = 3,
                    SmsGeographicalScope_Cell = 4,
                };
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsMessageClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                enum SmsMessageClass : int
                {
                    SmsMessageClass_None = 0,
                    SmsMessageClass_Class0 = 1,
                    SmsMessageClass_Class1 = 2,
                    SmsMessageClass_Class2 = 3,
                    SmsMessageClass_Class3 = 4,
                };
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsMessageFilter
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                enum
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("SmsMessageFilter may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                SmsMessageFilter : int
                {
                    SmsMessageFilter_All = 0,
                    SmsMessageFilter_Unread = 1,
                    SmsMessageFilter_Read = 2,
                    SmsMessageFilter_Sent = 3,
                    SmsMessageFilter_Draft = 4,
                };
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsMessageType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                enum SmsMessageType : int
                {
                    SmsMessageType_Binary = 0,
                    SmsMessageType_Text = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SmsMessageType_Wap = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SmsMessageType_App = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SmsMessageType_Broadcast = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SmsMessageType_Voicemail = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    SmsMessageType_Status = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsModemErrorCode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                enum SmsModemErrorCode : int
                {
                    SmsModemErrorCode_Other = 0,
                    SmsModemErrorCode_MessagingNetworkError = 1,
                    SmsModemErrorCode_SmsOperationNotSupportedByDevice = 2,
                    SmsModemErrorCode_SmsServiceNotSupportedByNetwork = 3,
                    SmsModemErrorCode_DeviceFailure = 4,
                    SmsModemErrorCode_MessageNotEncodedProperly = 5,
                    SmsModemErrorCode_MessageTooLarge = 6,
                    SmsModemErrorCode_DeviceNotReady = 7,
                    SmsModemErrorCode_NetworkNotReady = 8,
                    SmsModemErrorCode_InvalidSmscAddress = 9,
                    SmsModemErrorCode_NetworkFailure = 10,
                    SmsModemErrorCode_FixedDialingNumberRestricted = 11,
                };
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsEncodedLength
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                struct SmsEncodedLength
                {
                    UINT32 SegmentCount;
                    UINT32 CharacterCountLastSegment;
                    UINT32 CharactersPerSegment;
                    UINT32 ByteCountLastSegment;
                    UINT32 BytesPerSegment;
                };
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Devices.Sms.SmsDeviceStatusChangedEventHandler
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("982b1162-3dd7-4618-af89-0c272d5d06d8")
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("SmsDeviceStatusChangedEventHandler may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                ISmsDeviceStatusChangedEventHandler : public IUnknown
                {
                public:
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsDeviceStatusChangedEventHandler may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::Devices::Sms::ISmsDevice* sender
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsDeviceStatusChangedEventHandler = __uuidof(ISmsDeviceStatusChangedEventHandler);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Devices.Sms.SmsMessageReceivedEventHandler
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("0b7ad409-ec2d-47ce-a253-732beeebcacd")
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("SmsMessageReceivedEventHandler may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                ISmsMessageReceivedEventHandler : public IUnknown
                {
                public:
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsMessageReceivedEventHandler may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        ABI::Windows::Devices::Sms::ISmsDevice* sender,
                        ABI::Windows::Devices::Sms::ISmsMessageReceivedEventArgs* e
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsMessageReceivedEventHandler = __uuidof(ISmsMessageReceivedEventHandler);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsAppMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsAppMessage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Sms.ISmsMessageBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsAppMessage[] = L"Windows.Devices.Sms.ISmsAppMessage";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("e8bb8494-d3a0-4a0a-86d7-291033a8cf54")
                ISmsAppMessage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_To(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_To(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_From(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Body(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Body(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CallbackNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CallbackNumber(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsDeliveryNotificationEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsDeliveryNotificationEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RetryAttemptCount(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RetryAttemptCount(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Encoding(
                        ABI::Windows::Devices::Sms::SmsEncoding* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Encoding(
                        ABI::Windows::Devices::Sms::SmsEncoding value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PortNumber(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PortNumber(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TeleserviceId(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TeleserviceId(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProtocolId(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ProtocolId(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BinaryBody(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BinaryBody(
                        ABI::Windows::Storage::Streams::IBuffer* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsAppMessage = __uuidof(ISmsAppMessage);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsAppMessage;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsBinaryMessage
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Sms.ISmsMessage
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsBinaryMessage[] = L"Windows.Devices.Sms.ISmsBinaryMessage";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("5bf4e813-3b53-4c6e-b61a-d86a63755650")
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("SmsBinaryMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsAppMessage.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                ISmsBinaryMessage : public IInspectable
                {
                public:
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsBinaryMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsAppMessage.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Format(
                        ABI::Windows::Devices::Sms::SmsDataFormat* value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsBinaryMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsAppMessage.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_Format(
                        ABI::Windows::Devices::Sms::SmsDataFormat value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsBinaryMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsAppMessage.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE GetData(
                        UINT32* valueLength,
                        BYTE** value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsBinaryMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsAppMessage.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE SetData(
                        UINT32 valueLength,
                        BYTE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsBinaryMessage = __uuidof(ISmsBinaryMessage);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsBroadcastMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsBroadcastMessage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Sms.ISmsMessageBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsBroadcastMessage[] = L"Windows.Devices.Sms.ISmsBroadcastMessage";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("75aebbf1-e4b7-4874-a09c-2956e592f957")
                ISmsBroadcastMessage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_To(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Body(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Channel(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GeographicalScope(
                        ABI::Windows::Devices::Sms::SmsGeographicalScope* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MessageCode(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UpdateNumber(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BroadcastType(
                        ABI::Windows::Devices::Sms::SmsBroadcastType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsEmergencyAlert(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsUserPopupRequested(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsBroadcastMessage = __uuidof(ISmsBroadcastMessage);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsDevice
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsDevice[] = L"Windows.Devices.Sms.ISmsDevice";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("091791ed-872b-4eec-9c72-ab11627b34ec")
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                ISmsDevice : public IInspectable
                {
                public:
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE SendMessageAsync(
                        ABI::Windows::Devices::Sms::ISmsMessage* message,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE CalculateLength(
                        ABI::Windows::Devices::Sms::ISmsTextMessage* message,
                        ABI::Windows::Devices::Sms::SmsEncodedLength* encodedLength
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_AccountPhoneNumber(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_CellularClass(
                        ABI::Windows::Devices::Sms::CellularClass* value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_MessageStore(
                        ABI::Windows::Devices::Sms::ISmsDeviceMessageStore** value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceStatus(
                        ABI::Windows::Devices::Sms::SmsDeviceStatus* value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_SmsMessageReceived(
                        ABI::Windows::Devices::Sms::ISmsMessageReceivedEventHandler* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_SmsMessageReceived(
                        EventRegistrationToken eventCookie
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE add_SmsDeviceStatusChanged(
                        ABI::Windows::Devices::Sms::ISmsDeviceStatusChangedEventHandler* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE remove_SmsDeviceStatusChanged(
                        EventRegistrationToken eventCookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsDevice = __uuidof(ISmsDevice);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsDevice2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsDevice2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDevice2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDevice2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsDevice2[] = L"Windows.Devices.Sms.ISmsDevice2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("bd8a5c13-e522-46cb-b8d5-9ead30fb6c47")
                ISmsDevice2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SmscAddress(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SmscAddress(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ParentDeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AccountPhoneNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CellularClass(
                        ABI::Windows::Devices::Sms::CellularClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceStatus(
                        ABI::Windows::Devices::Sms::SmsDeviceStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CalculateLength(
                        ABI::Windows::Devices::Sms::ISmsMessageBase* message,
                        ABI::Windows::Devices::Sms::SmsEncodedLength* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendMessageAndGetResultAsync(
                        ABI::Windows::Devices::Sms::ISmsMessageBase* message,
                        __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult** asyncInfo
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_DeviceStatusChanged(
                        __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_DeviceStatusChanged(
                        EventRegistrationToken eventCookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsDevice2 = __uuidof(ISmsDevice2);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsDevice2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDevice2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsDevice2Statics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsDevice2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsDevice2Statics[] = L"Windows.Devices.Sms.ISmsDevice2Statics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("65c78325-1031-491e-8fb6-ef9991afe363")
                ISmsDevice2Statics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromId(
                        HSTRING deviceId,
                        ABI::Windows::Devices::Sms::ISmsDevice2** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Devices::Sms::ISmsDevice2** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FromParentId(
                        HSTRING parentDeviceId,
                        ABI::Windows::Devices::Sms::ISmsDevice2** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsDevice2Statics = __uuidof(ISmsDevice2Statics);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsDeviceMessageStore
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsDeviceMessageStore
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsDeviceMessageStore[] = L"Windows.Devices.Sms.ISmsDeviceMessageStore";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("9889f253-f188-4427-8d54-ce0c2423c5c1")
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                ISmsDeviceMessageStore : public IInspectable
                {
                public:
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE DeleteMessageAsync(
                        UINT32 messageId,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE DeleteMessagesAsync(
                        ABI::Windows::Devices::Sms::SmsMessageFilter messageFilter,
                        ABI::Windows::Foundation::IAsyncAction** asyncInfo
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE GetMessageAsync(
                        UINT32 messageId,
                        __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage** asyncInfo
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE GetMessagesAsync(
                        ABI::Windows::Devices::Sms::SmsMessageFilter messageFilter,
                        __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int** asyncInfo
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_MaxMessages(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsDeviceMessageStore = __uuidof(ISmsDeviceMessageStore);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsDeviceStatics
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsDevice
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsDeviceStatics[] = L"Windows.Devices.Sms.ISmsDeviceStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("f88d07ea-d815-4dd1-a234-4520ce4604a4")
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("ISmsDeviceStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                ISmsDeviceStatics : public IInspectable
                {
                public:
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISmsDeviceStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* phstrDeviceClassSelector
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISmsDeviceStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                        HSTRING deviceId,
                        __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice** asyncInfo
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISmsDeviceStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE GetDefaultAsync(
                        __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsDeviceStatics = __uuidof(ISmsDeviceStatics);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsDeviceStatics2
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsDevice
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsDeviceStatics2[] = L"Windows.Devices.Sms.ISmsDeviceStatics2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("2ca11c87-0873-4caf-8a7d-bd471e8586d1")
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("ISmsDeviceStatics2 may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                ISmsDeviceStatics2 : public IInspectable
                {
                public:
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISmsDeviceStatics2 may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE FromNetworkAccountIdAsync(
                        HSTRING networkAccountId,
                        __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice** asyncInfo
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsDeviceStatics2 = __uuidof(ISmsDeviceStatics2);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsFilterRule
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsFilterRule
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsFilterRule[] = L"Windows.Devices.Sms.ISmsFilterRule";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("40e32fae-b049-4fbc-afe9-e2a610eff55c")
                ISmsFilterRule : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MessageType(
                        ABI::Windows::Devices::Sms::SmsMessageType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ImsiPrefixes(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceIds(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SenderNumbers(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TextMessagePrefixes(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PortNumbers(
                        __FIVector_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CellularClass(
                        ABI::Windows::Devices::Sms::CellularClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CellularClass(
                        ABI::Windows::Devices::Sms::CellularClass value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProtocolIds(
                        __FIVector_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TeleserviceIds(
                        __FIVector_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WapApplicationIds(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WapContentTypes(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BroadcastTypes(
                        __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BroadcastChannels(
                        __FIVector_1_int** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsFilterRule = __uuidof(ISmsFilterRule);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsFilterRule;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsFilterRuleFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsFilterRule
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsFilterRuleFactory[] = L"Windows.Devices.Sms.ISmsFilterRuleFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("00c36508-6296-4f29-9aad-8920ceba3ce8")
                ISmsFilterRuleFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFilterRule(
                        ABI::Windows::Devices::Sms::SmsMessageType messageType,
                        ABI::Windows::Devices::Sms::ISmsFilterRule** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsFilterRuleFactory = __uuidof(ISmsFilterRuleFactory);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsFilterRules
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsFilterRules
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsFilterRules[] = L"Windows.Devices.Sms.ISmsFilterRules";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("4e47eafb-79cd-4881-9894-55a4135b23fa")
                ISmsFilterRules : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ActionType(
                        ABI::Windows::Devices::Sms::SmsFilterActionType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Rules(
                        __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsFilterRules = __uuidof(ISmsFilterRules);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsFilterRules;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsFilterRulesFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsFilterRules
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsFilterRulesFactory[] = L"Windows.Devices.Sms.ISmsFilterRulesFactory";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("a09924ed-6e2e-4530-9fde-465d02eed00e")
                ISmsFilterRulesFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFilterRules(
                        ABI::Windows::Devices::Sms::SmsFilterActionType actionType,
                        ABI::Windows::Devices::Sms::ISmsFilterRules** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsFilterRulesFactory = __uuidof(ISmsFilterRulesFactory);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsMessage
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsMessage[] = L"Windows.Devices.Sms.ISmsMessage";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("ed3c5e28-6984-4b07-811d-8d5906ed3cea")
                ISmsMessage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MessageClass(
                        ABI::Windows::Devices::Sms::SmsMessageClass* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsMessage = __uuidof(ISmsMessage);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsMessage;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsMessageBase
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsMessageBase[] = L"Windows.Devices.Sms.ISmsMessageBase";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("2cf0fe30-fe50-4fc6-aa88-4ccfe27a29ea")
                ISmsMessageBase : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MessageType(
                        ABI::Windows::Devices::Sms::SmsMessageType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CellularClass(
                        ABI::Windows::Devices::Sms::CellularClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MessageClass(
                        ABI::Windows::Devices::Sms::SmsMessageClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SimIccId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsMessageBase = __uuidof(ISmsMessageBase);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsMessageBase;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsMessageReceivedEventArgs
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsMessageReceivedEventArgs
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsMessageReceivedEventArgs[] = L"Windows.Devices.Sms.ISmsMessageReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("08e80a98-b8e5-41c1-a3d8-d3abfae22675")
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("SmsMessageReceivedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                ISmsMessageReceivedEventArgs : public IInspectable
                {
                public:
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsMessageReceivedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_TextMessage(
                        ABI::Windows::Devices::Sms::ISmsTextMessage** value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsMessageReceivedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_BinaryMessage(
                        ABI::Windows::Devices::Sms::ISmsBinaryMessage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsMessageReceivedEventArgs = __uuidof(ISmsMessageReceivedEventArgs);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsMessageReceivedTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsMessageReceivedTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsMessageReceivedTriggerDetails[] = L"Windows.Devices.Sms.ISmsMessageReceivedTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("2bcfcbd4-2657-4128-ad5f-e3877132bdb1")
                ISmsMessageReceivedTriggerDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MessageType(
                        ABI::Windows::Devices::Sms::SmsMessageType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TextMessage(
                        ABI::Windows::Devices::Sms::ISmsTextMessage2** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WapMessage(
                        ABI::Windows::Devices::Sms::ISmsWapMessage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppMessage(
                        ABI::Windows::Devices::Sms::ISmsAppMessage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BroadcastMessage(
                        ABI::Windows::Devices::Sms::ISmsBroadcastMessage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VoicemailMessage(
                        ABI::Windows::Devices::Sms::ISmsVoicemailMessage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StatusMessage(
                        ABI::Windows::Devices::Sms::ISmsStatusMessage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Drop(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Accept(void) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsMessageReceivedTriggerDetails = __uuidof(ISmsMessageReceivedTriggerDetails);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsMessageRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsMessageRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsMessageRegistration[] = L"Windows.Devices.Sms.ISmsMessageRegistration";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("1720503e-f34f-446b-83b3-0ff19923b409")
                ISmsMessageRegistration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Unregister(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_MessageReceived(
                        __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_MessageReceived(
                        EventRegistrationToken eventCookie
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsMessageRegistration = __uuidof(ISmsMessageRegistration);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsMessageRegistrationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsMessageRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsMessageRegistrationStatics[] = L"Windows.Devices.Sms.ISmsMessageRegistrationStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("63a05464-2898-4778-a03c-6f994907d63a")
                ISmsMessageRegistrationStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_AllRegistrations(
                        __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Register(
                        HSTRING id,
                        ABI::Windows::Devices::Sms::ISmsFilterRules* filterRules,
                        ABI::Windows::Devices::Sms::ISmsMessageRegistration** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsMessageRegistrationStatics = __uuidof(ISmsMessageRegistrationStatics);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsReceivedEventDetails
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsReceivedEventDetails
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsReceivedEventDetails[] = L"Windows.Devices.Sms.ISmsReceivedEventDetails";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("5bb50f15-e46d-4c82-847d-5a0304c1d53d")
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                ISmsReceivedEventDetails : public IInspectable
                {
                public:
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_MessageIndex(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsReceivedEventDetails = __uuidof(ISmsReceivedEventDetails);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsReceivedEventDetails2
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsReceivedEventDetails
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsReceivedEventDetails2[] = L"Windows.Devices.Sms.ISmsReceivedEventDetails2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("40e05c86-a7b4-4771-9ae7-0b5ffb12c03a")
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                ISmsReceivedEventDetails2 : public IInspectable
                {
                public:
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_MessageClass(
                        ABI::Windows::Devices::Sms::SmsMessageClass* value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_BinaryMessage(
                        ABI::Windows::Devices::Sms::ISmsBinaryMessage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsReceivedEventDetails2 = __uuidof(ISmsReceivedEventDetails2);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsSendMessageResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsSendMessageResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsSendMessageResult[] = L"Windows.Devices.Sms.ISmsSendMessageResult";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("db139af2-78c9-4feb-9622-452328088d62")
                ISmsSendMessageResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsSuccessful(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MessageReferenceNumbers(
                        __FIVectorView_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CellularClass(
                        ABI::Windows::Devices::Sms::CellularClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ModemErrorCode(
                        ABI::Windows::Devices::Sms::SmsModemErrorCode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsErrorTransient(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkCauseCode(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransportFailureCause(
                        INT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsSendMessageResult = __uuidof(ISmsSendMessageResult);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsStatusMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsStatusMessage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Sms.ISmsMessageBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsStatusMessage[] = L"Windows.Devices.Sms.ISmsStatusMessage";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("e6d28342-b70b-4677-9379-c9783fdff8f4")
                ISmsStatusMessage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_To(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_From(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Body(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MessageReferenceNumber(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceCenterTimestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DischargeTime(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsStatusMessage = __uuidof(ISmsStatusMessage);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsTextMessage
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Sms.ISmsMessage
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsTextMessage[] = L"Windows.Devices.Sms.ISmsTextMessage";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("d61c904c-a495-487f-9a6f-971548c5bc9f")
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                ISmsTextMessage : public IInspectable
                {
                public:
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_PartReferenceId(
                        UINT32* value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_PartNumber(
                        UINT32* value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_PartCount(
                        UINT32* value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_To(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_To(
                        HSTRING value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_From(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_From(
                        HSTRING value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Body(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_Body(
                        HSTRING value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Encoding(
                        ABI::Windows::Devices::Sms::SmsEncoding* value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_Encoding(
                        ABI::Windows::Devices::Sms::SmsEncoding value
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE ToBinaryMessages(
                        ABI::Windows::Devices::Sms::SmsDataFormat format,
                        __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage** messages
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsTextMessage = __uuidof(ISmsTextMessage);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsTextMessage;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsTextMessage2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsTextMessage2
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Sms.ISmsMessageBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsTextMessage2[] = L"Windows.Devices.Sms.ISmsTextMessage2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("22a0d893-4555-4755-b5a1-e7fd84955f8d")
                ISmsTextMessage2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_To(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_To(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_From(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Body(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Body(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Encoding(
                        ABI::Windows::Devices::Sms::SmsEncoding* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Encoding(
                        ABI::Windows::Devices::Sms::SmsEncoding value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CallbackNumber(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CallbackNumber(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsDeliveryNotificationEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsDeliveryNotificationEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RetryAttemptCount(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RetryAttemptCount(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TeleserviceId(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProtocolId(
                        INT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsTextMessage2 = __uuidof(ISmsTextMessage2);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsTextMessageStatics
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsTextMessage
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsTextMessageStatics[] = L"Windows.Devices.Sms.ISmsTextMessageStatics";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("7f68c5ed-3ccc-47a3-8c55-380d3b010892")
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("ISmsTextMessageStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                ISmsTextMessageStatics : public IInspectable
                {
                public:
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISmsTextMessageStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE FromBinaryMessage(
                        ABI::Windows::Devices::Sms::ISmsBinaryMessage* binaryMessage,
                        ABI::Windows::Devices::Sms::ISmsTextMessage** textMessage
                        ) = 0;
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("ISmsTextMessageStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE FromBinaryData(
                        ABI::Windows::Devices::Sms::SmsDataFormat format,
                        UINT32 valueLength,
                        BYTE* value,
                        ABI::Windows::Devices::Sms::ISmsTextMessage** textMessage
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsTextMessageStatics = __uuidof(ISmsTextMessageStatics);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsVoicemailMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsVoicemailMessage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Sms.ISmsMessageBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsVoicemailMessage[] = L"Windows.Devices.Sms.ISmsVoicemailMessage";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("271aa0a6-95b1-44ff-bcb8-b8fdd7e08bc3")
                ISmsVoicemailMessage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_To(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Body(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MessageCount(
                        __FIReference_1_int** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsVoicemailMessage = __uuidof(ISmsVoicemailMessage);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsWapMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsWapMessage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Sms.ISmsMessageBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsWapMessage[] = L"Windows.Devices.Sms.ISmsWapMessage";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace Sms {
                MIDL_INTERFACE("cd937743-7a55-4d3b-9021-f22e022d09c5")
                ISmsWapMessage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_To(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_From(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ApplicationId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ContentType(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BinaryBody(
                        ABI::Windows::Storage::Streams::IBuffer** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Headers(
                        __FIMap_2_HSTRING_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISmsWapMessage = __uuidof(ISmsWapMessage);
            } /* Sms */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsWapMessage;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.DeleteSmsMessageOperation
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncAction ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_DeleteSmsMessageOperation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_DeleteSmsMessageOperation_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("DeleteSmsMessageOperation may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_DeleteSmsMessageOperation[] = L"Windows.Devices.Sms.DeleteSmsMessageOperation";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.DeleteSmsMessagesOperation
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncAction ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_DeleteSmsMessagesOperation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_DeleteSmsMessagesOperation_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("DeleteSmsMessagesOperation may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_DeleteSmsMessagesOperation[] = L"Windows.Devices.Sms.DeleteSmsMessagesOperation";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.GetSmsDeviceOperation
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncOperation`1<Windows.Devices.Sms.SmsDevice> ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_GetSmsDeviceOperation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_GetSmsDeviceOperation_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("GetSmsDeviceOperation may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_GetSmsDeviceOperation[] = L"Windows.Devices.Sms.GetSmsDeviceOperation";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.GetSmsMessageOperation
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncOperation`1<Windows.Devices.Sms.ISmsMessage> ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_GetSmsMessageOperation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_GetSmsMessageOperation_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("GetSmsMessageOperation may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_GetSmsMessageOperation[] = L"Windows.Devices.Sms.GetSmsMessageOperation";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.GetSmsMessagesOperation
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Sms.ISmsMessage>, Int32> ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_GetSmsMessagesOperation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_GetSmsMessagesOperation_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("GetSmsMessagesOperation may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_GetSmsMessagesOperation[] = L"Windows.Devices.Sms.GetSmsMessagesOperation";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SendSmsMessageOperation
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncAction ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SendSmsMessageOperation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SendSmsMessageOperation_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SendSmsMessageOperation may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SendSmsMessageOperation[] = L"Windows.Devices.Sms.SendSmsMessageOperation";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsAppMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsAppMessage ** Default Interface **
 *    Windows.Devices.Sms.ISmsMessageBase
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsAppMessage_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsAppMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsAppMessage[] = L"Windows.Devices.Sms.SmsAppMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsBinaryMessage
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Devices.Sms.LegacySmsApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsBinaryMessage ** Default Interface **
 *    Windows.Devices.Sms.ISmsMessage
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsBinaryMessage_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsBinaryMessage_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsBinaryMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsAppMessage.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsBinaryMessage[] = L"Windows.Devices.Sms.SmsBinaryMessage";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsBroadcastMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsBroadcastMessage ** Default Interface **
 *    Windows.Devices.Sms.ISmsMessageBase
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsBroadcastMessage_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsBroadcastMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsBroadcastMessage[] = L"Windows.Devices.Sms.SmsBroadcastMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsDevice
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Sms.ISmsDeviceStatics interface starting with version 1.0 of the Windows.Devices.Sms.LegacySmsApiContract API contract
 *   Static Methods exist on the Windows.Devices.Sms.ISmsDeviceStatics2 interface starting with version 1.0 of the Windows.Devices.Sms.LegacySmsApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsDevice ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsDevice_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsDevice[] = L"Windows.Devices.Sms.SmsDevice";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsDevice2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Sms.ISmsDevice2Statics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsDevice2 ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsDevice2_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsDevice2_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsDevice2[] = L"Windows.Devices.Sms.SmsDevice2";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsDeviceMessageStore
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsDeviceMessageStore ** Default Interface **
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsDeviceMessageStore_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsDeviceMessageStore_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsDeviceMessageStore[] = L"Windows.Devices.Sms.SmsDeviceMessageStore";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsFilterRule
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Sms.ISmsFilterRuleFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsFilterRule ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsFilterRule_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsFilterRule_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsFilterRule[] = L"Windows.Devices.Sms.SmsFilterRule";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsFilterRules
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Sms.ISmsFilterRulesFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsFilterRules ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsFilterRules_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsFilterRules_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsFilterRules[] = L"Windows.Devices.Sms.SmsFilterRules";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsMessageReceivedEventArgs
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsMessageReceivedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsMessageReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsMessageReceivedEventArgs_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsMessageReceivedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsMessageReceivedEventArgs[] = L"Windows.Devices.Sms.SmsMessageReceivedEventArgs";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsMessageReceivedTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsMessageReceivedTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsMessageReceivedTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsMessageReceivedTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsMessageReceivedTriggerDetails[] = L"Windows.Devices.Sms.SmsMessageReceivedTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsMessageRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Sms.ISmsMessageRegistrationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsMessageRegistration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsMessageRegistration_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsMessageRegistration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsMessageRegistration[] = L"Windows.Devices.Sms.SmsMessageRegistration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsReceivedEventDetails
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsReceivedEventDetails ** Default Interface **
 *    Windows.Devices.Sms.ISmsReceivedEventDetails2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsReceivedEventDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsReceivedEventDetails_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsReceivedEventDetails[] = L"Windows.Devices.Sms.SmsReceivedEventDetails";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsSendMessageResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsSendMessageResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsSendMessageResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsSendMessageResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsSendMessageResult[] = L"Windows.Devices.Sms.SmsSendMessageResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsStatusMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsStatusMessage ** Default Interface **
 *    Windows.Devices.Sms.ISmsMessageBase
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsStatusMessage_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsStatusMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsStatusMessage[] = L"Windows.Devices.Sms.SmsStatusMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsTextMessage
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Devices.Sms.LegacySmsApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Sms.ISmsTextMessageStatics interface starting with version 1.0 of the Windows.Devices.Sms.LegacySmsApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsTextMessage ** Default Interface **
 *    Windows.Devices.Sms.ISmsMessage
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsTextMessage_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsTextMessage_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsTextMessage[] = L"Windows.Devices.Sms.SmsTextMessage";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsTextMessage2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsTextMessage2 ** Default Interface **
 *    Windows.Devices.Sms.ISmsMessageBase
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsTextMessage2_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsTextMessage2_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsTextMessage2[] = L"Windows.Devices.Sms.SmsTextMessage2";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsVoicemailMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsVoicemailMessage ** Default Interface **
 *    Windows.Devices.Sms.ISmsMessageBase
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsVoicemailMessage_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsVoicemailMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsVoicemailMessage[] = L"Windows.Devices.Sms.SmsVoicemailMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsWapMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsWapMessage ** Default Interface **
 *    Windows.Devices.Sms.ISmsMessageBase
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsWapMessage_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsWapMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsWapMessage[] = L"Windows.Devices.Sms.SmsWapMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsDevice __x_ABI_CWindows_CDevices_CSms_CISmsDevice;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsDevice2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDevice2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsDevice2 __x_ABI_CWindows_CDevices_CSms_CISmsDevice2;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsDevice2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2 __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsFilterRules __x_ABI_CWindows_CDevices_CSms_CISmsFilterRules;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsMessage __x_ABI_CWindows_CDevices_CSms_CISmsMessage;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2 __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2 __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics __x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage;

#endif // ____x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage;

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessage** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessageVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage* This,
        __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessageVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CISmsMessage_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice;

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsDevice** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDeviceVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice* This,
        __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDeviceVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsDevice_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResultVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult* This,
        __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CSms__CSmsSendMessageResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CSms__CISmsMessage_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CSms__CISmsMessage_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CSms__CISmsMessage __FIIterator_1_Windows__CDevices__CSms__CISmsMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CSms__CISmsMessage;

typedef struct __FIIterator_1_Windows__CDevices__CSms__CISmsMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CSms__CISmsMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CSms__CISmsMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CSms__CISmsMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CSms__CISmsMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CSms__CISmsMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CSms__CISmsMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CSms__CISmsMessage* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessage** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CSms__CISmsMessage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CSms__CISmsMessage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CSms__CISmsMessage* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessage** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CSms__CISmsMessageVtbl;

interface __FIIterator_1_Windows__CDevices__CSms__CISmsMessage
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CSms__CISmsMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CSms__CISmsMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsMessage_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsMessage_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsMessage_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsMessage_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CSms__CISmsMessage_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CSms__CISmsMessage_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CSms__CISmsMessage_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CSms__CISmsMessage __FIIterable_1_Windows__CDevices__CSms__CISmsMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CSms__CISmsMessage;

typedef struct __FIIterable_1_Windows__CDevices__CSms__CISmsMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CSms__CISmsMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CSms__CISmsMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CSms__CISmsMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CSms__CISmsMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CSms__CISmsMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CSms__CISmsMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CSms__CISmsMessage* This,
        __FIIterator_1_Windows__CDevices__CSms__CISmsMessage** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CSms__CISmsMessageVtbl;

interface __FIIterable_1_Windows__CDevices__CSms__CISmsMessage
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CSms__CISmsMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CSms__CISmsMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CSms__CISmsMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CSms__CISmsMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CSms__CISmsMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CSms__CISmsMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CSms__CISmsMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CSms__CISmsMessage_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CSms__CISmsMessage_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CSms__CISmsMessage __FIVectorView_1_Windows__CDevices__CSms__CISmsMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage;

typedef struct __FIVectorView_1_Windows__CDevices__CSms__CISmsMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessage** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessage* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CSms__CISmsMessage* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessage** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CSms__CISmsMessageVtbl;

interface __FIVectorView_1_Windows__CDevices__CSms__CISmsMessage
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CSms__CISmsMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int __FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int;

typedef interface __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int;

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int __FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This,
        __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_intVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int;

typedef struct __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This,
        __FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This,
        __FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This,
        __FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This,
        __FIAsyncOperationWithProgressCompletedHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This,
        __FIVectorView_1_Windows__CDevices__CSms__CISmsMessage** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_intVtbl;

interface __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int __FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int;

typedef struct __FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* This,
        __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int* asyncInfo,
        INT32 progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_intVtbl;

interface __FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIIterator_1_int_INTERFACE_DEFINED__)
#define ____FIIterator_1_int_INTERFACE_DEFINED__

typedef interface __FIIterator_1_int __FIIterator_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_int;

typedef struct __FIIterator_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_int* This,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_int* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_int* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_int* This,
        UINT32 itemsLength,
        INT32* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_intVtbl;

interface __FIIterator_1_int
{
    CONST_VTBL struct __FIIterator_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_int_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_int_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_int_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_int_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_int_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_int_INTERFACE_DEFINED__)
#define ____FIIterable_1_int_INTERFACE_DEFINED__

typedef interface __FIIterable_1_int __FIIterable_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_int;

typedef struct __FIIterable_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_int* This,
        __FIIterator_1_int** result);

    END_INTERFACE
} __FIIterable_1_intVtbl;

interface __FIIterable_1_int
{
    CONST_VTBL struct __FIIterable_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_int_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_int_INTERFACE_DEFINED__

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

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage;

typedef struct __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessageVtbl;

interface __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage __FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage;

typedef struct __FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        __FIIterator_1_Windows__CDevices__CSms__CISmsBinaryMessage** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessageVtbl;

interface __FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CSms__CISmsBinaryMessage_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CDevices_CSms_CSmsBroadcastType __x_ABI_CWindows_CDevices_CSms_CSmsBroadcastType;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType;

typedef struct __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsBroadcastType* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsBroadcastType* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastTypeVtbl;

interface __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType __FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType;

typedef struct __FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        __FIIterator_1_Windows__CDevices__CSms__CSmsBroadcastType** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastTypeVtbl;

interface __FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CSms__CSmsBroadcastType_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule;

typedef struct __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRuleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRuleVtbl;

interface __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRuleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule __FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule;

typedef struct __FIIterable_1_Windows__CDevices__CSms__CSmsFilterRuleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        __FIIterator_1_Windows__CDevices__CSms__CSmsFilterRule** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CSms__CSmsFilterRuleVtbl;

interface __FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CSms__CSmsFilterRuleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CSms__CSmsFilterRule_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration;

typedef struct __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistrationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistrationVtbl;

interface __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration
{
    CONST_VTBL struct __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistrationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration __FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration;

typedef struct __FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistrationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        __FIIterator_1_Windows__CDevices__CSms__CSmsMessageRegistration** result);

    END_INTERFACE
} __FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistrationVtbl;

interface __FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration
{
    CONST_VTBL struct __FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistrationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CDevices__CSms__CSmsMessageRegistration_INTERFACE_DEFINED__
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

#if !defined(____FIVectorView_1_int_INTERFACE_DEFINED__)
#define ____FIVectorView_1_int_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_int __FIVectorView_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_int;

typedef struct __FIVectorView_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_int* This,
        UINT32 index,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_int* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_int* This,
        INT32 value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_int* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        INT32* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_intVtbl;

interface __FIVectorView_1_int
{
    CONST_VTBL struct __FIVectorView_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_int_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_int_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_int_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_int_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_int_INTERFACE_DEFINED__

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

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage;

typedef struct __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessageVtbl;

interface __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage_INTERFACE_DEFINED__
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType;

typedef struct __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        UINT32 index,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsBroadcastType* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsBroadcastType value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsBroadcastType* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastTypeVtbl;

interface __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule;

typedef struct __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRuleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRuleVtbl;

interface __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRuleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration;

typedef struct __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistrationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistrationVtbl;

interface __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration
{
    CONST_VTBL struct __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistrationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIVector_1_int_INTERFACE_DEFINED__)
#define ____FIVector_1_int_INTERFACE_DEFINED__

typedef interface __FIVector_1_int __FIVector_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_int;

typedef struct __FIVector_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_int* This,
        UINT32 index,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_int* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_int* This,
        __FIVectorView_1_int** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_int* This,
        INT32 value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_int* This,
        UINT32 index,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_int* This,
        UINT32 index,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_int* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_int* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_int* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_int* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        INT32* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_int* This,
        UINT32 itemsLength,
        INT32* items);

    END_INTERFACE
} __FIVector_1_intVtbl;

interface __FIVector_1_int
{
    CONST_VTBL struct __FIVector_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_int_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_int_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_int_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_int_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_int_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_int_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_int_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_int_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_int_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_int_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_int_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_int_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_int_INTERFACE_DEFINED__

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
#if !defined(____FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType;

typedef struct __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        UINT32 index,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsBroadcastType* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        __FIVectorView_1_Windows__CDevices__CSms__CSmsBroadcastType** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsBroadcastType value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        UINT32 index,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsBroadcastType value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        UINT32 index,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsBroadcastType value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsBroadcastType value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsBroadcastType* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsBroadcastType* items);

    END_INTERFACE
} __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastTypeVtbl;

interface __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType
{
    CONST_VTBL struct __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CDevices__CSms__CSmsFilterRule;

typedef struct __FIVector_1_Windows__CDevices__CSms__CSmsFilterRuleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        __FIVectorView_1_Windows__CDevices__CSms__CSmsFilterRule** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        UINT32 index,
        __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CDevices__CSms__CSmsFilterRule* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule** items);

    END_INTERFACE
} __FIVector_1_Windows__CDevices__CSms__CSmsFilterRuleVtbl;

interface __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule
{
    CONST_VTBL struct __FIVector_1_Windows__CDevices__CSms__CSmsFilterRuleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CDevices__CSms__CSmsFilterRule_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIReference_1_int_INTERFACE_DEFINED__)
#define ____FIReference_1_int_INTERFACE_DEFINED__

typedef interface __FIReference_1_int __FIReference_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_int;

typedef struct __FIReference_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_int* This,
        INT32* result);

    END_INTERFACE
} __FIReference_1_intVtbl;

interface __FIReference_1_int
{
    CONST_VTBL struct __FIReference_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_int_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_int_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsDevice2* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration* sender,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetailsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CSms_CCellularClass __x_ABI_CWindows_CDevices_CSms_CCellularClass;

typedef enum __x_ABI_CWindows_CDevices_CSms_CSmsDataFormat __x_ABI_CWindows_CDevices_CSms_CSmsDataFormat;

typedef enum __x_ABI_CWindows_CDevices_CSms_CSmsDeviceStatus __x_ABI_CWindows_CDevices_CSms_CSmsDeviceStatus;

typedef enum __x_ABI_CWindows_CDevices_CSms_CSmsEncoding __x_ABI_CWindows_CDevices_CSms_CSmsEncoding;

typedef enum __x_ABI_CWindows_CDevices_CSms_CSmsFilterActionType __x_ABI_CWindows_CDevices_CSms_CSmsFilterActionType;

typedef enum __x_ABI_CWindows_CDevices_CSms_CSmsGeographicalScope __x_ABI_CWindows_CDevices_CSms_CSmsGeographicalScope;

typedef enum __x_ABI_CWindows_CDevices_CSms_CSmsMessageClass __x_ABI_CWindows_CDevices_CSms_CSmsMessageClass;

typedef enum __x_ABI_CWindows_CDevices_CSms_CSmsMessageFilter __x_ABI_CWindows_CDevices_CSms_CSmsMessageFilter;

typedef enum __x_ABI_CWindows_CDevices_CSms_CSmsMessageType __x_ABI_CWindows_CDevices_CSms_CSmsMessageType;

typedef enum __x_ABI_CWindows_CDevices_CSms_CSmsModemErrorCode __x_ABI_CWindows_CDevices_CSms_CSmsModemErrorCode;

typedef struct __x_ABI_CWindows_CDevices_CSms_CSmsEncodedLength __x_ABI_CWindows_CDevices_CSms_CSmsEncodedLength;

/*
 *
 * Struct Windows.Devices.Sms.CellularClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSms_CCellularClass
{
    CellularClass_None = 0,
    CellularClass_Gsm = 1,
    CellularClass_Cdma = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsBroadcastType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSms_CSmsBroadcastType
{
    SmsBroadcastType_Other = 0,
    SmsBroadcastType_CmasPresidential = 1,
    SmsBroadcastType_CmasExtreme = 2,
    SmsBroadcastType_CmasSevere = 3,
    SmsBroadcastType_CmasAmber = 4,
    SmsBroadcastType_CmasTest = 5,
    SmsBroadcastType_EUAlert1 = 6,
    SmsBroadcastType_EUAlert2 = 7,
    SmsBroadcastType_EUAlert3 = 8,
    SmsBroadcastType_EUAlertAmber = 9,
    SmsBroadcastType_EUAlertInfo = 10,
    SmsBroadcastType_EtwsEarthquake = 11,
    SmsBroadcastType_EtwsTsunami = 12,
    SmsBroadcastType_EtwsTsunamiAndEarthquake = 13,
    SmsBroadcastType_LatAlertLocal = 14,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsDataFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSms_CSmsDataFormat
{
    SmsDataFormat_Unknown = 0,
    SmsDataFormat_CdmaSubmit = 1,
    SmsDataFormat_GsmSubmit = 2,
    SmsDataFormat_CdmaDeliver = 3,
    SmsDataFormat_GsmDeliver = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsDeviceStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSms_CSmsDeviceStatus
{
    SmsDeviceStatus_Off = 0,
    SmsDeviceStatus_Ready = 1,
    SmsDeviceStatus_SimNotInserted = 2,
    SmsDeviceStatus_BadSim = 3,
    SmsDeviceStatus_DeviceFailure = 4,
    SmsDeviceStatus_SubscriptionNotActivated = 5,
    SmsDeviceStatus_DeviceLocked = 6,
    SmsDeviceStatus_DeviceBlocked = 7,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsEncoding
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSms_CSmsEncoding
{
    SmsEncoding_Unknown = 0,
    SmsEncoding_Optimal = 1,
    SmsEncoding_SevenBitAscii = 2,
    SmsEncoding_Unicode = 3,
    SmsEncoding_GsmSevenBit = 4,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SmsEncoding_EightBit = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SmsEncoding_Latin = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SmsEncoding_Korean = 7,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SmsEncoding_IA5 = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SmsEncoding_ShiftJis = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SmsEncoding_LatinHebrew = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsFilterActionType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSms_CSmsFilterActionType
{
    SmsFilterActionType_AcceptImmediately = 0,
    SmsFilterActionType_Drop = 1,
    SmsFilterActionType_Peek = 2,
    SmsFilterActionType_Accept = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsGeographicalScope
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSms_CSmsGeographicalScope
{
    SmsGeographicalScope_None = 0,
    SmsGeographicalScope_CellWithImmediateDisplay = 1,
    SmsGeographicalScope_LocationArea = 2,
    SmsGeographicalScope_Plmn = 3,
    SmsGeographicalScope_Cell = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsMessageClass
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSms_CSmsMessageClass
{
    SmsMessageClass_None = 0,
    SmsMessageClass_Class0 = 1,
    SmsMessageClass_Class1 = 2,
    SmsMessageClass_Class2 = 3,
    SmsMessageClass_Class3 = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsMessageFilter
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsMessageFilter may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CDevices_CSms_CSmsMessageFilter
{
    SmsMessageFilter_All = 0,
    SmsMessageFilter_Unread = 1,
    SmsMessageFilter_Read = 2,
    SmsMessageFilter_Sent = 3,
    SmsMessageFilter_Draft = 4,
};
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsMessageType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSms_CSmsMessageType
{
    SmsMessageType_Binary = 0,
    SmsMessageType_Text = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SmsMessageType_Wap = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SmsMessageType_App = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SmsMessageType_Broadcast = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SmsMessageType_Voicemail = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    SmsMessageType_Status = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsModemErrorCode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CDevices_CSms_CSmsModemErrorCode
{
    SmsModemErrorCode_Other = 0,
    SmsModemErrorCode_MessagingNetworkError = 1,
    SmsModemErrorCode_SmsOperationNotSupportedByDevice = 2,
    SmsModemErrorCode_SmsServiceNotSupportedByNetwork = 3,
    SmsModemErrorCode_DeviceFailure = 4,
    SmsModemErrorCode_MessageNotEncodedProperly = 5,
    SmsModemErrorCode_MessageTooLarge = 6,
    SmsModemErrorCode_DeviceNotReady = 7,
    SmsModemErrorCode_NetworkNotReady = 8,
    SmsModemErrorCode_InvalidSmscAddress = 9,
    SmsModemErrorCode_NetworkFailure = 10,
    SmsModemErrorCode_FixedDialingNumberRestricted = 11,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Devices.Sms.SmsEncodedLength
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CDevices_CSms_CSmsEncodedLength
{
    UINT32 SegmentCount;
    UINT32 CharacterCountLastSegment;
    UINT32 CharactersPerSegment;
    UINT32 ByteCountLastSegment;
    UINT32 BytesPerSegment;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Devices.Sms.SmsDeviceStatusChangedEventHandler
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler_INTERFACE_DEFINED__
typedef struct
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsDeviceStatusChangedEventHandler may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler* This);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDeviceStatusChangedEventHandler may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsDevice* sender);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandlerVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDeviceStatusChangedEventHandler may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler_Invoke(This, sender) \
    ((This)->lpVtbl->Invoke(This, sender))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Devices.Sms.SmsMessageReceivedEventHandler
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler_INTERFACE_DEFINED__
typedef struct
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsMessageReceivedEventHandler may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandlerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler* This);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsMessageReceivedEventHandler may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsDevice* sender,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs* e);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandlerVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandlerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler_Release(This) \
    ((This)->lpVtbl->Release(This))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsMessageReceivedEventHandler may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler_Invoke(This, sender, e) \
    ((This)->lpVtbl->Invoke(This, sender, e))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsAppMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsAppMessage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Sms.ISmsMessageBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsAppMessage[] = L"Windows.Devices.Sms.ISmsAppMessage";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsAppMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_To)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_To)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_From)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Body)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Body)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_CallbackNumber)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CallbackNumber)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IsDeliveryNotificationEnabled)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsDeliveryNotificationEnabled)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_RetryAttemptCount)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_RetryAttemptCount)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Encoding)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsEncoding* value);
    HRESULT (STDMETHODCALLTYPE* put_Encoding)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsEncoding value);
    HRESULT (STDMETHODCALLTYPE* get_PortNumber)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_PortNumber)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_TeleserviceId)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TeleserviceId)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ProtocolId)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ProtocolId)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_BinaryBody)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* put_BinaryBody)(__x_ABI_CWindows_CDevices_CSms_CISmsAppMessage* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsAppMessageVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsAppMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_get_To(This, value) \
    ((This)->lpVtbl->get_To(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_put_To(This, value) \
    ((This)->lpVtbl->put_To(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_get_From(This, value) \
    ((This)->lpVtbl->get_From(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_get_Body(This, value) \
    ((This)->lpVtbl->get_Body(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_put_Body(This, value) \
    ((This)->lpVtbl->put_Body(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_get_CallbackNumber(This, value) \
    ((This)->lpVtbl->get_CallbackNumber(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_put_CallbackNumber(This, value) \
    ((This)->lpVtbl->put_CallbackNumber(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_get_IsDeliveryNotificationEnabled(This, value) \
    ((This)->lpVtbl->get_IsDeliveryNotificationEnabled(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_put_IsDeliveryNotificationEnabled(This, value) \
    ((This)->lpVtbl->put_IsDeliveryNotificationEnabled(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_get_RetryAttemptCount(This, value) \
    ((This)->lpVtbl->get_RetryAttemptCount(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_put_RetryAttemptCount(This, value) \
    ((This)->lpVtbl->put_RetryAttemptCount(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_get_Encoding(This, value) \
    ((This)->lpVtbl->get_Encoding(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_put_Encoding(This, value) \
    ((This)->lpVtbl->put_Encoding(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_get_PortNumber(This, value) \
    ((This)->lpVtbl->get_PortNumber(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_put_PortNumber(This, value) \
    ((This)->lpVtbl->put_PortNumber(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_get_TeleserviceId(This, value) \
    ((This)->lpVtbl->get_TeleserviceId(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_put_TeleserviceId(This, value) \
    ((This)->lpVtbl->put_TeleserviceId(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_get_ProtocolId(This, value) \
    ((This)->lpVtbl->get_ProtocolId(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_put_ProtocolId(This, value) \
    ((This)->lpVtbl->put_ProtocolId(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_get_BinaryBody(This, value) \
    ((This)->lpVtbl->get_BinaryBody(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_put_BinaryBody(This, value) \
    ((This)->lpVtbl->put_BinaryBody(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsAppMessage;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsAppMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsBinaryMessage
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Sms.ISmsMessage
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsBinaryMessage[] = L"Windows.Devices.Sms.ISmsBinaryMessage";
typedef struct
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsBinaryMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsAppMessage.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage* This,
        TrustLevel* trustLevel);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsBinaryMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsAppMessage.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Format)(__x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsDataFormat* value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsBinaryMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsAppMessage.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_Format)(__x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsDataFormat value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsBinaryMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsAppMessage.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* GetData)(__x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage* This,
        UINT32* valueLength,
        BYTE** value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsBinaryMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsAppMessage.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* SetData)(__x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage* This,
        UINT32 valueLength,
        BYTE* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessageVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsBinaryMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsAppMessage.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_get_Format(This, value) \
    ((This)->lpVtbl->get_Format(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsBinaryMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsAppMessage.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_put_Format(This, value) \
    ((This)->lpVtbl->put_Format(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsBinaryMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsAppMessage.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_GetData(This, valueLength, value) \
    ((This)->lpVtbl->GetData(This, valueLength, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsBinaryMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsAppMessage.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_SetData(This, valueLength, value) \
    ((This)->lpVtbl->SetData(This, valueLength, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsBroadcastMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsBroadcastMessage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Sms.ISmsMessageBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsBroadcastMessage[] = L"Windows.Devices.Sms.ISmsBroadcastMessage";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_To)(__x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Body)(__x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Channel)(__x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_GeographicalScope)(__x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsGeographicalScope* value);
    HRESULT (STDMETHODCALLTYPE* get_MessageCode)(__x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_UpdateNumber)(__x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_BroadcastType)(__x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsBroadcastType* value);
    HRESULT (STDMETHODCALLTYPE* get_IsEmergencyAlert)(__x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsUserPopupRequested)(__x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessageVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_get_To(This, value) \
    ((This)->lpVtbl->get_To(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_get_Body(This, value) \
    ((This)->lpVtbl->get_Body(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_get_Channel(This, value) \
    ((This)->lpVtbl->get_Channel(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_get_GeographicalScope(This, value) \
    ((This)->lpVtbl->get_GeographicalScope(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_get_MessageCode(This, value) \
    ((This)->lpVtbl->get_MessageCode(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_get_UpdateNumber(This, value) \
    ((This)->lpVtbl->get_UpdateNumber(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_get_BroadcastType(This, value) \
    ((This)->lpVtbl->get_BroadcastType(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_get_IsEmergencyAlert(This, value) \
    ((This)->lpVtbl->get_IsEmergencyAlert(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_get_IsUserPopupRequested(This, value) \
    ((This)->lpVtbl->get_IsUserPopupRequested(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsDevice
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDevice_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDevice_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsDevice[] = L"Windows.Devices.Sms.ISmsDevice";
typedef struct
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CDevices_CSms_CISmsDeviceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice* This,
        TrustLevel* trustLevel);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* SendMessageAsync)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessage* message,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* CalculateLength)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* message,
        struct __x_ABI_CWindows_CDevices_CSms_CSmsEncodedLength* encodedLength);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_AccountPhoneNumber)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice* This,
        HSTRING* value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_CellularClass)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice* This,
        enum __x_ABI_CWindows_CDevices_CSms_CCellularClass* value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_MessageStore)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore** value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_DeviceStatus)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsDeviceStatus* value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_SmsMessageReceived)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventHandler* eventHandler,
        EventRegistrationToken* eventCookie);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_SmsMessageReceived)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice* This,
        EventRegistrationToken eventCookie);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_SmsDeviceStatusChanged)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatusChangedEventHandler* eventHandler,
        EventRegistrationToken* eventCookie);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_SmsDeviceStatusChanged)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsDeviceVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsDevice
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsDeviceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice_SendMessageAsync(This, message, asyncInfo) \
    ((This)->lpVtbl->SendMessageAsync(This, message, asyncInfo))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice_CalculateLength(This, message, encodedLength) \
    ((This)->lpVtbl->CalculateLength(This, message, encodedLength))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice_get_AccountPhoneNumber(This, value) \
    ((This)->lpVtbl->get_AccountPhoneNumber(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice_get_CellularClass(This, value) \
    ((This)->lpVtbl->get_CellularClass(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice_get_MessageStore(This, value) \
    ((This)->lpVtbl->get_MessageStore(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice_get_DeviceStatus(This, value) \
    ((This)->lpVtbl->get_DeviceStatus(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice_add_SmsMessageReceived(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_SmsMessageReceived(This, eventHandler, eventCookie))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice_remove_SmsMessageReceived(This, eventCookie) \
    ((This)->lpVtbl->remove_SmsMessageReceived(This, eventCookie))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice_add_SmsDeviceStatusChanged(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_SmsDeviceStatusChanged(This, eventHandler, eventCookie))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice_remove_SmsDeviceStatusChanged(This, eventCookie) \
    ((This)->lpVtbl->remove_SmsDeviceStatusChanged(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsDevice;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDevice_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsDevice2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsDevice2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDevice2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDevice2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsDevice2[] = L"Windows.Devices.Sms.ISmsDevice2";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SmscAddress)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_SmscAddress)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ParentDeviceId)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AccountPhoneNumber)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CellularClass)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This,
        enum __x_ABI_CWindows_CDevices_CSms_CCellularClass* value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceStatus)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsDeviceStatus* value);
    HRESULT (STDMETHODCALLTYPE* CalculateLength)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase* message,
        struct __x_ABI_CWindows_CDevices_CSms_CSmsEncodedLength* value);
    HRESULT (STDMETHODCALLTYPE* SendMessageAndGetResultAsync)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase* message,
        __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsSendMessageResult** asyncInfo);
    HRESULT (STDMETHODCALLTYPE* add_DeviceStatusChanged)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This,
        __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsDevice2_IInspectable* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_DeviceStatusChanged)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Vtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsDevice2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_get_SmscAddress(This, value) \
    ((This)->lpVtbl->get_SmscAddress(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_put_SmscAddress(This, value) \
    ((This)->lpVtbl->put_SmscAddress(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_get_ParentDeviceId(This, value) \
    ((This)->lpVtbl->get_ParentDeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_get_AccountPhoneNumber(This, value) \
    ((This)->lpVtbl->get_AccountPhoneNumber(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_get_CellularClass(This, value) \
    ((This)->lpVtbl->get_CellularClass(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_get_DeviceStatus(This, value) \
    ((This)->lpVtbl->get_DeviceStatus(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_CalculateLength(This, message, value) \
    ((This)->lpVtbl->CalculateLength(This, message, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_SendMessageAndGetResultAsync(This, message, asyncInfo) \
    ((This)->lpVtbl->SendMessageAndGetResultAsync(This, message, asyncInfo))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_add_DeviceStatusChanged(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_DeviceStatusChanged(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2_remove_DeviceStatusChanged(This, eventCookie) \
    ((This)->lpVtbl->remove_DeviceStatusChanged(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsDevice2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDevice2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsDevice2Statics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsDevice2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsDevice2Statics[] = L"Windows.Devices.Sms.ISmsDevice2Statics";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsDevice2StaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* FromId)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics* This,
        HSTRING deviceId,
        __x_ABI_CWindows_CDevices_CSms_CISmsDevice2** value);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsDevice2** value);
    HRESULT (STDMETHODCALLTYPE* FromParentId)(__x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics* This,
        HSTRING parentDeviceId,
        __x_ABI_CWindows_CDevices_CSms_CISmsDevice2** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsDevice2StaticsVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsDevice2StaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_GetDeviceSelector(This, value) \
    ((This)->lpVtbl->GetDeviceSelector(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_FromId(This, deviceId, value) \
    ((This)->lpVtbl->FromId(This, deviceId, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_GetDefault(This, value) \
    ((This)->lpVtbl->GetDefault(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_FromParentId(This, parentDeviceId, value) \
    ((This)->lpVtbl->FromParentId(This, parentDeviceId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDevice2Statics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsDeviceMessageStore
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsDeviceMessageStore
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsDeviceMessageStore[] = L"Windows.Devices.Sms.ISmsDeviceMessageStore";
typedef struct
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore* This,
        TrustLevel* trustLevel);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* DeleteMessageAsync)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore* This,
        UINT32 messageId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* DeleteMessagesAsync)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsMessageFilter messageFilter,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncInfo);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* GetMessageAsync)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore* This,
        UINT32 messageId,
        __FIAsyncOperation_1_Windows__CDevices__CSms__CISmsMessage** asyncInfo);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* GetMessagesAsync)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsMessageFilter messageFilter,
        __FIAsyncOperationWithProgress_2___FIVectorView_1_Windows__CDevices__CSms__CISmsMessage_int** asyncInfo);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_MaxMessages)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStoreVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_DeleteMessageAsync(This, messageId, asyncInfo) \
    ((This)->lpVtbl->DeleteMessageAsync(This, messageId, asyncInfo))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_DeleteMessagesAsync(This, messageFilter, asyncInfo) \
    ((This)->lpVtbl->DeleteMessagesAsync(This, messageFilter, asyncInfo))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_GetMessageAsync(This, messageId, asyncInfo) \
    ((This)->lpVtbl->GetMessageAsync(This, messageId, asyncInfo))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_GetMessagesAsync(This, messageFilter, asyncInfo) \
    ((This)->lpVtbl->GetMessagesAsync(This, messageFilter, asyncInfo))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_get_MaxMessages(This, value) \
    ((This)->lpVtbl->get_MaxMessages(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDeviceMessageStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsDeviceStatics
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsDevice
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsDeviceStatics[] = L"Windows.Devices.Sms.ISmsDeviceStatics";
typedef struct
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("ISmsDeviceStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ISmsDeviceStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics* This,
        HSTRING* phstrDeviceClassSelector);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ISmsDeviceStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics* This,
        HSTRING deviceId,
        __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice** asyncInfo);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ISmsDeviceStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* GetDefaultAsync)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics* This,
        __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ISmsDeviceStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_GetDeviceSelector(This, phstrDeviceClassSelector) \
    ((This)->lpVtbl->GetDeviceSelector(This, phstrDeviceClassSelector))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ISmsDeviceStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_FromIdAsync(This, deviceId, asyncInfo) \
    ((This)->lpVtbl->FromIdAsync(This, deviceId, asyncInfo))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ISmsDeviceStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_GetDefaultAsync(This, asyncInfo) \
    ((This)->lpVtbl->GetDefaultAsync(This, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsDeviceStatics2
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsDevice
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsDeviceStatics2[] = L"Windows.Devices.Sms.ISmsDeviceStatics2";
typedef struct
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("ISmsDeviceStatics2 may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2* This,
        TrustLevel* trustLevel);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ISmsDeviceStatics2 may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* FromNetworkAccountIdAsync)(__x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2* This,
        HSTRING networkAccountId,
        __FIAsyncOperation_1_Windows__CDevices__CSms__CSmsDevice** asyncInfo);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2Vtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ISmsDeviceStatics2 may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_FromNetworkAccountIdAsync(This, networkAccountId, asyncInfo) \
    ((This)->lpVtbl->FromNetworkAccountIdAsync(This, networkAccountId, asyncInfo))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsDeviceStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsFilterRule
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsFilterRule
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsFilterRule[] = L"Windows.Devices.Sms.ISmsFilterRule";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MessageType)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsMessageType* value);
    HRESULT (STDMETHODCALLTYPE* get_ImsiPrefixes)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceIds)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_SenderNumbers)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_TextMessagePrefixes)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_PortNumbers)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        __FIVector_1_int** value);
    HRESULT (STDMETHODCALLTYPE* get_CellularClass)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        enum __x_ABI_CWindows_CDevices_CSms_CCellularClass* value);
    HRESULT (STDMETHODCALLTYPE* put_CellularClass)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        enum __x_ABI_CWindows_CDevices_CSms_CCellularClass value);
    HRESULT (STDMETHODCALLTYPE* get_ProtocolIds)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        __FIVector_1_int** value);
    HRESULT (STDMETHODCALLTYPE* get_TeleserviceIds)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        __FIVector_1_int** value);
    HRESULT (STDMETHODCALLTYPE* get_WapApplicationIds)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_WapContentTypes)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_BroadcastTypes)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        __FIVector_1_Windows__CDevices__CSms__CSmsBroadcastType** value);
    HRESULT (STDMETHODCALLTYPE* get_BroadcastChannels)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRule* This,
        __FIVector_1_int** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_get_MessageType(This, value) \
    ((This)->lpVtbl->get_MessageType(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_get_ImsiPrefixes(This, value) \
    ((This)->lpVtbl->get_ImsiPrefixes(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_get_DeviceIds(This, value) \
    ((This)->lpVtbl->get_DeviceIds(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_get_SenderNumbers(This, value) \
    ((This)->lpVtbl->get_SenderNumbers(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_get_TextMessagePrefixes(This, value) \
    ((This)->lpVtbl->get_TextMessagePrefixes(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_get_PortNumbers(This, value) \
    ((This)->lpVtbl->get_PortNumbers(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_get_CellularClass(This, value) \
    ((This)->lpVtbl->get_CellularClass(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_put_CellularClass(This, value) \
    ((This)->lpVtbl->put_CellularClass(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_get_ProtocolIds(This, value) \
    ((This)->lpVtbl->get_ProtocolIds(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_get_TeleserviceIds(This, value) \
    ((This)->lpVtbl->get_TeleserviceIds(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_get_WapApplicationIds(This, value) \
    ((This)->lpVtbl->get_WapApplicationIds(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_get_WapContentTypes(This, value) \
    ((This)->lpVtbl->get_WapContentTypes(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_get_BroadcastTypes(This, value) \
    ((This)->lpVtbl->get_BroadcastTypes(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_get_BroadcastChannels(This, value) \
    ((This)->lpVtbl->get_BroadcastChannels(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsFilterRule;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsFilterRule_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsFilterRuleFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsFilterRule
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsFilterRuleFactory[] = L"Windows.Devices.Sms.ISmsFilterRuleFactory";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFilterRule)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsMessageType messageType,
        __x_ABI_CWindows_CDevices_CSms_CISmsFilterRule** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_CreateFilterRule(This, messageType, value) \
    ((This)->lpVtbl->CreateFilterRule(This, messageType, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsFilterRuleFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsFilterRules
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsFilterRules
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsFilterRules[] = L"Windows.Devices.Sms.ISmsFilterRules";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRules* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRules* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRules* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRules* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRules* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRules* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ActionType)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRules* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsFilterActionType* value);
    HRESULT (STDMETHODCALLTYPE* get_Rules)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRules* This,
        __FIVector_1_Windows__CDevices__CSms__CSmsFilterRule** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsFilterRules
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_get_ActionType(This, value) \
    ((This)->lpVtbl->get_ActionType(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_get_Rules(This, value) \
    ((This)->lpVtbl->get_Rules(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsFilterRules;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsFilterRules_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsFilterRulesFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsFilterRules
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsFilterRulesFactory[] = L"Windows.Devices.Sms.ISmsFilterRulesFactory";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFilterRules)(__x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsFilterActionType actionType,
        __x_ABI_CWindows_CDevices_CSms_CISmsFilterRules** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactoryVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_CreateFilterRules(This, actionType, value) \
    ((This)->lpVtbl->CreateFilterRules(This, actionType, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsFilterRulesFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsMessage
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsMessage[] = L"Windows.Devices.Sms.ISmsMessage";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CSms_CISmsMessage* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MessageClass)(__x_ABI_CWindows_CDevices_CSms_CISmsMessage* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsMessageClass* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsMessageVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsMessage
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessage_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessage_get_MessageClass(This, value) \
    ((This)->lpVtbl->get_MessageClass(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsMessage;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsMessageBase
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsMessageBase[] = L"Windows.Devices.Sms.ISmsMessageBase";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsMessageBaseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageBase* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageBase* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageBase* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageBase* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageBase* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageBase* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MessageType)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageBase* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsMessageType* value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageBase* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CellularClass)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageBase* This,
        enum __x_ABI_CWindows_CDevices_CSms_CCellularClass* value);
    HRESULT (STDMETHODCALLTYPE* get_MessageClass)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageBase* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsMessageClass* value);
    HRESULT (STDMETHODCALLTYPE* get_SimIccId)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageBase* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsMessageBaseVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsMessageBaseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_get_MessageType(This, value) \
    ((This)->lpVtbl->get_MessageType(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_get_CellularClass(This, value) \
    ((This)->lpVtbl->get_CellularClass(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_get_MessageClass(This, value) \
    ((This)->lpVtbl->get_MessageClass(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_get_SimIccId(This, value) \
    ((This)->lpVtbl->get_SimIccId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsMessageBase;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageBase_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsMessageReceivedEventArgs
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsMessageReceivedEventArgs
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsMessageReceivedEventArgs[] = L"Windows.Devices.Sms.ISmsMessageReceivedEventArgs";
typedef struct
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsMessageReceivedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsMessageReceivedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_TextMessage)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage** value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsMessageReceivedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_BinaryMessage)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsMessageReceivedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_get_TextMessage(This, value) \
    ((This)->lpVtbl->get_TextMessage(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsMessageReceivedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_get_BinaryMessage(This, value) \
    ((This)->lpVtbl->get_BinaryMessage(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsMessageReceivedTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsMessageReceivedTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsMessageReceivedTriggerDetails[] = L"Windows.Devices.Sms.ISmsMessageReceivedTriggerDetails";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MessageType)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsMessageType* value);
    HRESULT (STDMETHODCALLTYPE* get_TextMessage)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2** value);
    HRESULT (STDMETHODCALLTYPE* get_WapMessage)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage** value);
    HRESULT (STDMETHODCALLTYPE* get_AppMessage)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsAppMessage** value);
    HRESULT (STDMETHODCALLTYPE* get_BroadcastMessage)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsBroadcastMessage** value);
    HRESULT (STDMETHODCALLTYPE* get_VoicemailMessage)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage** value);
    HRESULT (STDMETHODCALLTYPE* get_StatusMessage)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage** value);
    HRESULT (STDMETHODCALLTYPE* Drop)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* Accept)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails* This);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetailsVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_get_MessageType(This, value) \
    ((This)->lpVtbl->get_MessageType(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_get_TextMessage(This, value) \
    ((This)->lpVtbl->get_TextMessage(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_get_WapMessage(This, value) \
    ((This)->lpVtbl->get_WapMessage(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_get_AppMessage(This, value) \
    ((This)->lpVtbl->get_AppMessage(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_get_BroadcastMessage(This, value) \
    ((This)->lpVtbl->get_BroadcastMessage(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_get_VoicemailMessage(This, value) \
    ((This)->lpVtbl->get_VoicemailMessage(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_get_StatusMessage(This, value) \
    ((This)->lpVtbl->get_StatusMessage(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_Drop(This) \
    ((This)->lpVtbl->Drop(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_Accept(This) \
    ((This)->lpVtbl->Accept(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageReceivedTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsMessageRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsMessageRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsMessageRegistration[] = L"Windows.Devices.Sms.ISmsMessageRegistration";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* Unregister)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration* This);
    HRESULT (STDMETHODCALLTYPE* add_MessageReceived)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration* This,
        __FITypedEventHandler_2_Windows__CDevices__CSms__CSmsMessageRegistration_Windows__CDevices__CSms__CSmsMessageReceivedTriggerDetails* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_MessageReceived)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_Unregister(This) \
    ((This)->lpVtbl->Unregister(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_add_MessageReceived(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_MessageReceived(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_remove_MessageReceived(This, eventCookie) \
    ((This)->lpVtbl->remove_MessageReceived(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsMessageRegistrationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsMessageRegistration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsMessageRegistrationStatics[] = L"Windows.Devices.Sms.ISmsMessageRegistrationStatics";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AllRegistrations)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics* This,
        __FIVectorView_1_Windows__CDevices__CSms__CSmsMessageRegistration** value);
    HRESULT (STDMETHODCALLTYPE* Register)(__x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics* This,
        HSTRING id,
        __x_ABI_CWindows_CDevices_CSms_CISmsFilterRules* filterRules,
        __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistration** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_get_AllRegistrations(This, value) \
    ((This)->lpVtbl->get_AllRegistrations(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_Register(This, id, filterRules, value) \
    ((This)->lpVtbl->Register(This, id, filterRules, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsMessageRegistrationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsReceivedEventDetails
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsReceivedEventDetails
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsReceivedEventDetails[] = L"Windows.Devices.Sms.ISmsReceivedEventDetails";
typedef struct
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails* This,
        TrustLevel* trustLevel);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails* This,
        HSTRING* value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_MessageIndex)(__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetailsVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_get_MessageIndex(This, value) \
    ((This)->lpVtbl->get_MessageIndex(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsReceivedEventDetails2
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsReceivedEventDetails
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsReceivedEventDetails2[] = L"Windows.Devices.Sms.ISmsReceivedEventDetails2";
typedef struct
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2* This,
        TrustLevel* trustLevel);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_MessageClass)(__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsMessageClass* value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_BinaryMessage)(__x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2Vtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_get_MessageClass(This, value) \
    ((This)->lpVtbl->get_MessageClass(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_get_BinaryMessage(This, value) \
    ((This)->lpVtbl->get_BinaryMessage(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsReceivedEventDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsSendMessageResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsSendMessageResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsSendMessageResult[] = L"Windows.Devices.Sms.ISmsSendMessageResult";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsSuccessful)(__x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_MessageReferenceNumbers)(__x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult* This,
        __FIVectorView_1_int** value);
    HRESULT (STDMETHODCALLTYPE* get_CellularClass)(__x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult* This,
        enum __x_ABI_CWindows_CDevices_CSms_CCellularClass* value);
    HRESULT (STDMETHODCALLTYPE* get_ModemErrorCode)(__x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsModemErrorCode* value);
    HRESULT (STDMETHODCALLTYPE* get_IsErrorTransient)(__x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_NetworkCauseCode)(__x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_TransportFailureCause)(__x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResultVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_get_IsSuccessful(This, value) \
    ((This)->lpVtbl->get_IsSuccessful(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_get_MessageReferenceNumbers(This, value) \
    ((This)->lpVtbl->get_MessageReferenceNumbers(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_get_CellularClass(This, value) \
    ((This)->lpVtbl->get_CellularClass(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_get_ModemErrorCode(This, value) \
    ((This)->lpVtbl->get_ModemErrorCode(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_get_IsErrorTransient(This, value) \
    ((This)->lpVtbl->get_IsErrorTransient(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_get_NetworkCauseCode(This, value) \
    ((This)->lpVtbl->get_NetworkCauseCode(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_get_TransportFailureCause(This, value) \
    ((This)->lpVtbl->get_TransportFailureCause(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsSendMessageResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsStatusMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsStatusMessage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Sms.ISmsMessageBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsStatusMessage[] = L"Windows.Devices.Sms.ISmsStatusMessage";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_To)(__x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_From)(__x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Body)(__x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MessageReferenceNumber)(__x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceCenterTimestamp)(__x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_DischargeTime)(__x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessageVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_get_To(This, value) \
    ((This)->lpVtbl->get_To(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_get_From(This, value) \
    ((This)->lpVtbl->get_From(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_get_Body(This, value) \
    ((This)->lpVtbl->get_Body(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_get_MessageReferenceNumber(This, value) \
    ((This)->lpVtbl->get_MessageReferenceNumber(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_get_ServiceCenterTimestamp(This, value) \
    ((This)->lpVtbl->get_ServiceCenterTimestamp(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_get_DischargeTime(This, value) \
    ((This)->lpVtbl->get_DischargeTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsStatusMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsTextMessage
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Sms.ISmsMessage
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsTextMessage[] = L"Windows.Devices.Sms.ISmsTextMessage";
typedef struct
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CDevices_CSms_CISmsTextMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        TrustLevel* trustLevel);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_PartReferenceId)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        UINT32* value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_PartNumber)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        UINT32* value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_PartCount)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        UINT32* value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_To)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        HSTRING* value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_To)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        HSTRING value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_From)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        HSTRING* value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_From)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        HSTRING value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Body)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        HSTRING* value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_Body)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        HSTRING value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Encoding)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsEncoding* value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_Encoding)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsEncoding value);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* ToBinaryMessages)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsDataFormat format,
        __FIVectorView_1_Windows__CDevices__CSms__CISmsBinaryMessage** messages);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsTextMessageVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsTextMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_get_PartReferenceId(This, value) \
    ((This)->lpVtbl->get_PartReferenceId(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_get_PartNumber(This, value) \
    ((This)->lpVtbl->get_PartNumber(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_get_PartCount(This, value) \
    ((This)->lpVtbl->get_PartCount(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_get_To(This, value) \
    ((This)->lpVtbl->get_To(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_put_To(This, value) \
    ((This)->lpVtbl->put_To(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_get_From(This, value) \
    ((This)->lpVtbl->get_From(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_put_From(This, value) \
    ((This)->lpVtbl->put_From(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_get_Body(This, value) \
    ((This)->lpVtbl->get_Body(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_put_Body(This, value) \
    ((This)->lpVtbl->put_Body(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_get_Encoding(This, value) \
    ((This)->lpVtbl->get_Encoding(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_put_Encoding(This, value) \
    ((This)->lpVtbl->put_Encoding(This, value))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_ToBinaryMessages(This, format, messages) \
    ((This)->lpVtbl->ToBinaryMessages(This, format, messages))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsTextMessage;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsTextMessage2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsTextMessage2
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Sms.ISmsMessageBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsTextMessage2[] = L"Windows.Devices.Sms.ISmsTextMessage2";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_To)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_To)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_From)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Body)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Body)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Encoding)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsEncoding* value);
    HRESULT (STDMETHODCALLTYPE* put_Encoding)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsEncoding value);
    HRESULT (STDMETHODCALLTYPE* get_CallbackNumber)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CallbackNumber)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IsDeliveryNotificationEnabled)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsDeliveryNotificationEnabled)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_RetryAttemptCount)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_RetryAttemptCount)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_TeleserviceId)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ProtocolId)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2Vtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_get_To(This, value) \
    ((This)->lpVtbl->get_To(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_put_To(This, value) \
    ((This)->lpVtbl->put_To(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_get_From(This, value) \
    ((This)->lpVtbl->get_From(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_get_Body(This, value) \
    ((This)->lpVtbl->get_Body(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_put_Body(This, value) \
    ((This)->lpVtbl->put_Body(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_get_Encoding(This, value) \
    ((This)->lpVtbl->get_Encoding(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_put_Encoding(This, value) \
    ((This)->lpVtbl->put_Encoding(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_get_CallbackNumber(This, value) \
    ((This)->lpVtbl->get_CallbackNumber(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_put_CallbackNumber(This, value) \
    ((This)->lpVtbl->put_CallbackNumber(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_get_IsDeliveryNotificationEnabled(This, value) \
    ((This)->lpVtbl->get_IsDeliveryNotificationEnabled(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_put_IsDeliveryNotificationEnabled(This, value) \
    ((This)->lpVtbl->put_IsDeliveryNotificationEnabled(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_get_RetryAttemptCount(This, value) \
    ((This)->lpVtbl->get_RetryAttemptCount(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_put_RetryAttemptCount(This, value) \
    ((This)->lpVtbl->put_RetryAttemptCount(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_get_TeleserviceId(This, value) \
    ((This)->lpVtbl->get_TeleserviceId(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_get_ProtocolId(This, value) \
    ((This)->lpVtbl->get_ProtocolId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsTextMessage2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsTextMessageStatics
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsTextMessage
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsTextMessageStatics[] = L"Windows.Devices.Sms.ISmsTextMessageStatics";
typedef struct
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("ISmsTextMessageStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ISmsTextMessageStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* FromBinaryMessage)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics* This,
        __x_ABI_CWindows_CDevices_CSms_CISmsBinaryMessage* binaryMessage,
        __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage** textMessage);
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ISmsTextMessageStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* FromBinaryData)(__x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics* This,
        enum __x_ABI_CWindows_CDevices_CSms_CSmsDataFormat format,
        UINT32 valueLength,
        BYTE* value,
        __x_ABI_CWindows_CDevices_CSms_CISmsTextMessage** textMessage);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStaticsVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ISmsTextMessageStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_FromBinaryMessage(This, binaryMessage, textMessage) \
    ((This)->lpVtbl->FromBinaryMessage(This, binaryMessage, textMessage))

#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("ISmsTextMessageStatics may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_FromBinaryData(This, format, valueLength, value, textMessage) \
    ((This)->lpVtbl->FromBinaryData(This, format, valueLength, value, textMessage))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsTextMessageStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsVoicemailMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsVoicemailMessage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Sms.ISmsMessageBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsVoicemailMessage[] = L"Windows.Devices.Sms.ISmsVoicemailMessage";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_To)(__x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Body)(__x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MessageCount)(__x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage* This,
        __FIReference_1_int** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessageVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_get_To(This, value) \
    ((This)->lpVtbl->get_To(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_get_Body(This, value) \
    ((This)->lpVtbl->get_Body(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_get_MessageCount(This, value) \
    ((This)->lpVtbl->get_MessageCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsVoicemailMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Devices.Sms.ISmsWapMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Devices.Sms.SmsWapMessage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Devices.Sms.ISmsMessageBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_Sms_ISmsWapMessage[] = L"Windows.Devices.Sms.ISmsWapMessage";
typedef struct __x_ABI_CWindows_CDevices_CSms_CISmsWapMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CSms_CISmsWapMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CSms_CISmsWapMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CSms_CISmsWapMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CSms_CISmsWapMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CSms_CISmsWapMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CSms_CISmsWapMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CDevices_CSms_CISmsWapMessage* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_To)(__x_ABI_CWindows_CDevices_CSms_CISmsWapMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_From)(__x_ABI_CWindows_CDevices_CSms_CISmsWapMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ApplicationId)(__x_ABI_CWindows_CDevices_CSms_CISmsWapMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ContentType)(__x_ABI_CWindows_CDevices_CSms_CISmsWapMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_BinaryBody)(__x_ABI_CWindows_CDevices_CSms_CISmsWapMessage* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* get_Headers)(__x_ABI_CWindows_CDevices_CSms_CISmsWapMessage* This,
        __FIMap_2_HSTRING_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CSms_CISmsWapMessageVtbl;

interface __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CSms_CISmsWapMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_get_To(This, value) \
    ((This)->lpVtbl->get_To(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_get_From(This, value) \
    ((This)->lpVtbl->get_From(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_get_ApplicationId(This, value) \
    ((This)->lpVtbl->get_ApplicationId(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_get_ContentType(This, value) \
    ((This)->lpVtbl->get_ContentType(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_get_BinaryBody(This, value) \
    ((This)->lpVtbl->get_BinaryBody(This, value))

#define __x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_get_Headers(This, value) \
    ((This)->lpVtbl->get_Headers(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CSms_CISmsWapMessage;
#endif /* !defined(____x_ABI_CWindows_CDevices_CSms_CISmsWapMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.DeleteSmsMessageOperation
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncAction ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_DeleteSmsMessageOperation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_DeleteSmsMessageOperation_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("DeleteSmsMessageOperation may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_DeleteSmsMessageOperation[] = L"Windows.Devices.Sms.DeleteSmsMessageOperation";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.DeleteSmsMessagesOperation
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncAction ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_DeleteSmsMessagesOperation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_DeleteSmsMessagesOperation_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("DeleteSmsMessagesOperation may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_DeleteSmsMessagesOperation[] = L"Windows.Devices.Sms.DeleteSmsMessagesOperation";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.GetSmsDeviceOperation
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncOperation`1<Windows.Devices.Sms.SmsDevice> ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_GetSmsDeviceOperation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_GetSmsDeviceOperation_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("GetSmsDeviceOperation may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_GetSmsDeviceOperation[] = L"Windows.Devices.Sms.GetSmsDeviceOperation";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.GetSmsMessageOperation
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncOperation`1<Windows.Devices.Sms.ISmsMessage> ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_GetSmsMessageOperation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_GetSmsMessageOperation_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("GetSmsMessageOperation may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_GetSmsMessageOperation[] = L"Windows.Devices.Sms.GetSmsMessageOperation";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.GetSmsMessagesOperation
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Foundation.Collections.IVectorView`1<Windows.Devices.Sms.ISmsMessage>, Int32> ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_GetSmsMessagesOperation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_GetSmsMessagesOperation_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("GetSmsMessagesOperation may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_GetSmsMessagesOperation[] = L"Windows.Devices.Sms.GetSmsMessagesOperation";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SendSmsMessageOperation
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IAsyncAction ** Default Interface **
 *    Windows.Foundation.IAsyncInfo
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SendSmsMessageOperation_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SendSmsMessageOperation_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SendSmsMessageOperation may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SendSmsMessageOperation[] = L"Windows.Devices.Sms.SendSmsMessageOperation";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsAppMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsAppMessage ** Default Interface **
 *    Windows.Devices.Sms.ISmsMessageBase
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsAppMessage_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsAppMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsAppMessage[] = L"Windows.Devices.Sms.SmsAppMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsBinaryMessage
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Devices.Sms.LegacySmsApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsBinaryMessage ** Default Interface **
 *    Windows.Devices.Sms.ISmsMessage
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsBinaryMessage_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsBinaryMessage_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsBinaryMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsAppMessage.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsBinaryMessage[] = L"Windows.Devices.Sms.SmsBinaryMessage";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsBroadcastMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsBroadcastMessage ** Default Interface **
 *    Windows.Devices.Sms.ISmsMessageBase
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsBroadcastMessage_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsBroadcastMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsBroadcastMessage[] = L"Windows.Devices.Sms.SmsBroadcastMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsDevice
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Sms.ISmsDeviceStatics interface starting with version 1.0 of the Windows.Devices.Sms.LegacySmsApiContract API contract
 *   Static Methods exist on the Windows.Devices.Sms.ISmsDeviceStatics2 interface starting with version 1.0 of the Windows.Devices.Sms.LegacySmsApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsDevice ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsDevice_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsDevice_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsDevice may be altered or unavailable for releases after Windows 10. Instead, use SmsDevice2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsDevice[] = L"Windows.Devices.Sms.SmsDevice";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsDevice2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Sms.ISmsDevice2Statics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsDevice2 ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsDevice2_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsDevice2_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsDevice2[] = L"Windows.Devices.Sms.SmsDevice2";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsDeviceMessageStore
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsDeviceMessageStore ** Default Interface **
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsDeviceMessageStore_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsDeviceMessageStore_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsDeviceMessageStore may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsDeviceMessageStore[] = L"Windows.Devices.Sms.SmsDeviceMessageStore";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsFilterRule
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Sms.ISmsFilterRuleFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsFilterRule ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsFilterRule_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsFilterRule_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsFilterRule[] = L"Windows.Devices.Sms.SmsFilterRule";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsFilterRules
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Devices.Sms.ISmsFilterRulesFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsFilterRules ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsFilterRules_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsFilterRules_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsFilterRules[] = L"Windows.Devices.Sms.SmsFilterRules";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsMessageReceivedEventArgs
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsMessageReceivedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsMessageReceivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsMessageReceivedEventArgs_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsMessageReceivedEventArgs may be altered or unavailable for releases after Windows 10.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsMessageReceivedEventArgs[] = L"Windows.Devices.Sms.SmsMessageReceivedEventArgs";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsMessageReceivedTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsMessageReceivedTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsMessageReceivedTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsMessageReceivedTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsMessageReceivedTriggerDetails[] = L"Windows.Devices.Sms.SmsMessageReceivedTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsMessageRegistration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Sms.ISmsMessageRegistrationStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsMessageRegistration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsMessageRegistration_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsMessageRegistration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsMessageRegistration[] = L"Windows.Devices.Sms.SmsMessageRegistration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsReceivedEventDetails
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsReceivedEventDetails ** Default Interface **
 *    Windows.Devices.Sms.ISmsReceivedEventDetails2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsReceivedEventDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsReceivedEventDetails_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsReceivedEventDetails may be altered or unavailable for releases after Windows 10. Instead, use SmsMessageReceivedTriggerDetails.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsReceivedEventDetails[] = L"Windows.Devices.Sms.SmsReceivedEventDetails";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsSendMessageResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsSendMessageResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsSendMessageResult_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsSendMessageResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsSendMessageResult[] = L"Windows.Devices.Sms.SmsSendMessageResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsStatusMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsStatusMessage ** Default Interface **
 *    Windows.Devices.Sms.ISmsMessageBase
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsStatusMessage_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsStatusMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsStatusMessage[] = L"Windows.Devices.Sms.SmsStatusMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsTextMessage
 *
 * Introduced to Windows.Devices.Sms.LegacySmsApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Devices.Sms.LegacySmsApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Devices.Sms.ISmsTextMessageStatics interface starting with version 1.0 of the Windows.Devices.Sms.LegacySmsApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsTextMessage ** Default Interface **
 *    Windows.Devices.Sms.ISmsMessage
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsTextMessage_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsTextMessage_DEFINED
#if WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
DEPRECATED("SmsTextMessage may be altered or unavailable for releases after Windows 10. Instead, use SmsTextMessage2.")
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsTextMessage[] = L"Windows.Devices.Sms.SmsTextMessage";
#endif
#endif // WINDOWS_DEVICES_SMS_LEGACYSMSAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsTextMessage2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsTextMessage2 ** Default Interface **
 *    Windows.Devices.Sms.ISmsMessageBase
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsTextMessage2_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsTextMessage2_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsTextMessage2[] = L"Windows.Devices.Sms.SmsTextMessage2";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsVoicemailMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsVoicemailMessage ** Default Interface **
 *    Windows.Devices.Sms.ISmsMessageBase
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsVoicemailMessage_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsVoicemailMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsVoicemailMessage[] = L"Windows.Devices.Sms.SmsVoicemailMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Devices.Sms.SmsWapMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.Sms.ISmsWapMessage ** Default Interface **
 *    Windows.Devices.Sms.ISmsMessageBase
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Devices_Sms_SmsWapMessage_DEFINED
#define RUNTIMECLASS_Windows_Devices_Sms_SmsWapMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_Sms_SmsWapMessage[] = L"Windows.Devices.Sms.SmsWapMessage";
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
#endif // __windows2Edevices2Esms_p_h__

#endif // __windows2Edevices2Esms_h__
