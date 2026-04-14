
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
#ifndef __windows2Eapplicationmodel2Echat_h__
#define __windows2Eapplicationmodel2Echat_h__
#ifndef __windows2Eapplicationmodel2Echat_p_h__
#define __windows2Eapplicationmodel2Echat_p_h__


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
#include "Windows.Media.MediaProperties.h"
#include "Windows.Security.Credentials.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatCapabilities;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities ABI::Windows::ApplicationModel::Chat::IChatCapabilities

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatCapabilitiesManagerStatics;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics ABI::Windows::ApplicationModel::Chat::IChatCapabilitiesManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatCapabilitiesManagerStatics2;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2 ABI::Windows::ApplicationModel::Chat::IChatCapabilitiesManagerStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatConversation;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation ABI::Windows::ApplicationModel::Chat::IChatConversation

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatConversation2;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2 ABI::Windows::ApplicationModel::Chat::IChatConversation2

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatConversationReader;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader ABI::Windows::ApplicationModel::Chat::IChatConversationReader

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatConversationThreadingInfo;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo ABI::Windows::ApplicationModel::Chat::IChatConversationThreadingInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatItem;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatItem ABI::Windows::ApplicationModel::Chat::IChatItem

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessage;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage ABI::Windows::ApplicationModel::Chat::IChatMessage

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessage2;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2 ABI::Windows::ApplicationModel::Chat::IChatMessage2

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessage3;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3 ABI::Windows::ApplicationModel::Chat::IChatMessage3

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessage4;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4 ABI::Windows::ApplicationModel::Chat::IChatMessage4

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageAttachment;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment ABI::Windows::ApplicationModel::Chat::IChatMessageAttachment

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageAttachment2;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2 ABI::Windows::ApplicationModel::Chat::IChatMessageAttachment2

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageAttachmentFactory;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory ABI::Windows::ApplicationModel::Chat::IChatMessageAttachmentFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageBlockingStatic;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic ABI::Windows::ApplicationModel::Chat::IChatMessageBlockingStatic

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageChange;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange ABI::Windows::ApplicationModel::Chat::IChatMessageChange

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageChangeReader;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader ABI::Windows::ApplicationModel::Chat::IChatMessageChangeReader

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageChangeTracker;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker ABI::Windows::ApplicationModel::Chat::IChatMessageChangeTracker

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageChangedDeferral;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral ABI::Windows::ApplicationModel::Chat::IChatMessageChangedDeferral

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageChangedEventArgs;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs ABI::Windows::ApplicationModel::Chat::IChatMessageChangedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageManager2Statics;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics ABI::Windows::ApplicationModel::Chat::IChatMessageManager2Statics

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageManagerStatic;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic ABI::Windows::ApplicationModel::Chat::IChatMessageManagerStatic

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageManagerStatics3;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3 ABI::Windows::ApplicationModel::Chat::IChatMessageManagerStatics3

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageNotificationTriggerDetails;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails ABI::Windows::ApplicationModel::Chat::IChatMessageNotificationTriggerDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageNotificationTriggerDetails2;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2 ABI::Windows::ApplicationModel::Chat::IChatMessageNotificationTriggerDetails2

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageReader;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader ABI::Windows::ApplicationModel::Chat::IChatMessageReader

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageReader2;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2 ABI::Windows::ApplicationModel::Chat::IChatMessageReader2

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageStore;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore ABI::Windows::ApplicationModel::Chat::IChatMessageStore

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageStore2;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2 ABI::Windows::ApplicationModel::Chat::IChatMessageStore2

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageStore3;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3 ABI::Windows::ApplicationModel::Chat::IChatMessageStore3

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageStoreChangedEventArgs;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs ABI::Windows::ApplicationModel::Chat::IChatMessageStoreChangedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageTransport;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport ABI::Windows::ApplicationModel::Chat::IChatMessageTransport

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageTransport2;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2 ABI::Windows::ApplicationModel::Chat::IChatMessageTransport2

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageTransportConfiguration;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration ABI::Windows::ApplicationModel::Chat::IChatMessageTransportConfiguration

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatMessageValidationResult;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult ABI::Windows::ApplicationModel::Chat::IChatMessageValidationResult

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatQueryOptions;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions ABI::Windows::ApplicationModel::Chat::IChatQueryOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatRecipientDeliveryInfo;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo ABI::Windows::ApplicationModel::Chat::IChatRecipientDeliveryInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatSearchReader;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader ABI::Windows::ApplicationModel::Chat::IChatSearchReader

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatSyncConfiguration;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration ABI::Windows::ApplicationModel::Chat::IChatSyncConfiguration

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IChatSyncManager;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager ABI::Windows::ApplicationModel::Chat::IChatSyncManager

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IRcsEndUserMessage;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage ABI::Windows::ApplicationModel::Chat::IRcsEndUserMessage

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IRcsEndUserMessageAction;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction ABI::Windows::ApplicationModel::Chat::IRcsEndUserMessageAction

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IRcsEndUserMessageAvailableEventArgs;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs ABI::Windows::ApplicationModel::Chat::IRcsEndUserMessageAvailableEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IRcsEndUserMessageAvailableTriggerDetails;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails ABI::Windows::ApplicationModel::Chat::IRcsEndUserMessageAvailableTriggerDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IRcsEndUserMessageManager;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager ABI::Windows::ApplicationModel::Chat::IRcsEndUserMessageManager

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IRcsManagerStatics;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics ABI::Windows::ApplicationModel::Chat::IRcsManagerStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IRcsManagerStatics2;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2 ABI::Windows::ApplicationModel::Chat::IRcsManagerStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IRcsServiceKindSupportedChangedEventArgs;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs ABI::Windows::ApplicationModel::Chat::IRcsServiceKindSupportedChangedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IRcsTransport;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport ABI::Windows::ApplicationModel::Chat::IRcsTransport

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IRcsTransportConfiguration;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration ABI::Windows::ApplicationModel::Chat::IRcsTransportConfiguration

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                interface IRemoteParticipantComposingChangedEventArgs;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs ABI::Windows::ApplicationModel::Chat::IRemoteParticipantComposingChangedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_FWD_DEFINED__

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



#ifndef DEF___FIAsyncOperation_1_int_USE
#define DEF___FIAsyncOperation_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("968b9665-06ed-5774-8f53-8edeabd5f7b5"))
IAsyncOperation<int> : IAsyncOperation_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<int> __FIAsyncOperation_1_int_t;
#define __FIAsyncOperation_1_int ABI::Windows::Foundation::__FIAsyncOperation_1_int_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_int_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_int_USE
#define DEF___FIAsyncOperationCompletedHandler_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d60cae9d-88cb-59f1-8576-3fba44796be8"))
IAsyncOperationCompletedHandler<int> : IAsyncOperationCompletedHandler_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<int> __FIAsyncOperationCompletedHandler_1_int_t;
#define __FIAsyncOperationCompletedHandler_1_int ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_int_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_int_USE */



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
            namespace Chat {
                class ChatCapabilities;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("30889035-c687-573b-86e4-024e38f2aa6d"))
IAsyncOperation<ABI::Windows::ApplicationModel::Chat::ChatCapabilities*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatCapabilities*, ABI::Windows::ApplicationModel::Chat::IChatCapabilities*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Chat.ChatCapabilities>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Chat::ChatCapabilities*> __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ce2d035c-7686-56bd-a2ca-194735fd8617"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Chat::ChatCapabilities*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatCapabilities*, ABI::Windows::ApplicationModel::Chat::IChatCapabilities*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Chat.ChatCapabilities>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Chat::ChatCapabilities*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatConversation;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c4d8e414-0e54-5adf-88c3-e56d4e88b5d4"))
IAsyncOperation<ABI::Windows::ApplicationModel::Chat::ChatConversation*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatConversation*, ABI::Windows::ApplicationModel::Chat::IChatConversation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Chat.ChatConversation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Chat::ChatConversation*> __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0b9c15f1-1871-50c5-86de-6e614d593c57"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Chat::ChatConversation*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatConversation*, ABI::Windows::ApplicationModel::Chat::IChatConversation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Chat.ChatConversation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Chat::ChatConversation*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatMessage;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ee129c22-da8f-5c55-90b6-a73bacc9d735"))
IAsyncOperation<ABI::Windows::ApplicationModel::Chat::ChatMessage*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessage*, ABI::Windows::ApplicationModel::Chat::IChatMessage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Chat.ChatMessage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Chat::ChatMessage*> __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2704edec-009d-5abb-a718-767718158d88"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Chat::ChatMessage*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessage*, ABI::Windows::ApplicationModel::Chat::IChatMessage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Chat.ChatMessage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Chat::ChatMessage*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatMessageStore;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("19642405-0e06-5119-9ac5-16cfd106b337"))
IAsyncOperation<ABI::Windows::ApplicationModel::Chat::ChatMessageStore*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageStore*, ABI::Windows::ApplicationModel::Chat::IChatMessageStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Chat.ChatMessageStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Chat::ChatMessageStore*> __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a9174f86-1fc7-50f4-8d7e-103d3fd6e5a3"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Chat::ChatMessageStore*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageStore*, ABI::Windows::ApplicationModel::Chat::IChatMessageStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Chat.ChatMessageStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Chat::ChatMessageStore*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatMessageTransport;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("275e7895-3a8e-5175-b76f-21d2e045bd2f"))
IAsyncOperation<ABI::Windows::ApplicationModel::Chat::ChatMessageTransport*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageTransport*, ABI::Windows::ApplicationModel::Chat::IChatMessageTransport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Chat.ChatMessageTransport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Chat::ChatMessageTransport*> __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("69dc85b1-ba0b-57d3-b7c3-618e0156f8cb"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Chat::ChatMessageTransport*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageTransport*, ABI::Windows::ApplicationModel::Chat::IChatMessageTransport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Chat.ChatMessageTransport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Chat::ChatMessageTransport*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatSyncManager;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a4667a6a-1c64-51f6-972e-a1050719f8ea"))
IAsyncOperation<ABI::Windows::ApplicationModel::Chat::ChatSyncManager*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatSyncManager*, ABI::Windows::ApplicationModel::Chat::IChatSyncManager*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Chat.ChatSyncManager>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Chat::ChatSyncManager*> __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5b3512cc-8528-5e87-b061-1b982a647fc4"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Chat::ChatSyncManager*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatSyncManager*, ABI::Windows::ApplicationModel::Chat::IChatSyncManager*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Chat.ChatSyncManager>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Chat::ChatSyncManager*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class RcsTransport;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f88774e8-9664-5df5-abbd-c64556d25062"))
IAsyncOperation<ABI::Windows::ApplicationModel::Chat::RcsTransport*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::RcsTransport*, ABI::Windows::ApplicationModel::Chat::IRcsTransport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Chat.RcsTransport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Chat::RcsTransport*> __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7851a9c5-1467-5c7d-af74-57ec6bd33417"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Chat::RcsTransport*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::RcsTransport*, ABI::Windows::ApplicationModel::Chat::IRcsTransport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Chat.RcsTransport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Chat::RcsTransport*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7007a155-47ca-59c8-bf1e-960b82159907"))
IIterator<ABI::Windows::ApplicationModel::Chat::ChatConversation*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatConversation*, ABI::Windows::ApplicationModel::Chat::IChatConversation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Chat.ChatConversation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Chat::ChatConversation*> __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_t;
#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("af43c676-a512-5388-9f69-0308953d719b"))
IIterable<ABI::Windows::ApplicationModel::Chat::ChatConversation*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatConversation*, ABI::Windows::ApplicationModel::Chat::IChatConversation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Chat.ChatConversation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Chat::ChatConversation*> __FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation_t;
#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8d4e5cf4-610b-5a29-b66a-2649700f5245"))
IVectorView<ABI::Windows::ApplicationModel::Chat::ChatConversation*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatConversation*, ABI::Windows::ApplicationModel::Chat::IChatConversation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.ChatConversation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Chat::ChatConversation*> __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_t;
#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d531f714-d3c7-589b-875c-bfaa27522478"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.ChatConversation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a54ad656-836f-5b3e-aebd-17c3e98de48e"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.ChatConversation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cd3c8682-2366-5646-9af5-bcb9c3be9ebe"))
IIterator<ABI::Windows::ApplicationModel::Chat::ChatMessage*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessage*, ABI::Windows::ApplicationModel::Chat::IChatMessage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Chat.ChatMessage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Chat::ChatMessage*> __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_t;
#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("647bf12c-f621-5fd8-af39-c72b16baf07e"))
IIterable<ABI::Windows::ApplicationModel::Chat::ChatMessage*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessage*, ABI::Windows::ApplicationModel::Chat::IChatMessage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Chat.ChatMessage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Chat::ChatMessage*> __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage_t;
#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ac612e87-63fd-5c05-999a-0dae0d8ec7a3"))
IVectorView<ABI::Windows::ApplicationModel::Chat::ChatMessage*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessage*, ABI::Windows::ApplicationModel::Chat::IChatMessage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.ChatMessage>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Chat::ChatMessage*> __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_t;
#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d9f6a9dd-7fcf-5e9e-86c7-151663db67cc"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.ChatMessage>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e136bd95-1eca-5b87-b233-9d47d6aa5224"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.ChatMessage>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatMessageChange;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("df6e6b4f-3e45-52cd-aade-3316896abad0"))
IIterator<ABI::Windows::ApplicationModel::Chat::ChatMessageChange*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageChange*, ABI::Windows::ApplicationModel::Chat::IChatMessageChange*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Chat.ChatMessageChange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Chat::ChatMessageChange*> __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_t;
#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0df896df-354c-5d35-b7ac-6b2e1d70c8eb"))
IIterable<ABI::Windows::ApplicationModel::Chat::ChatMessageChange*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageChange*, ABI::Windows::ApplicationModel::Chat::IChatMessageChange*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Chat.ChatMessageChange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Chat::ChatMessageChange*> __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange_t;
#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("447a59fc-e729-5846-91da-b650fdeca785"))
IVectorView<ABI::Windows::ApplicationModel::Chat::ChatMessageChange*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageChange*, ABI::Windows::ApplicationModel::Chat::IChatMessageChange*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.ChatMessageChange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Chat::ChatMessageChange*> __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_t;
#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("831968b7-aa61-58cc-8713-98f65cad4ed9"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.ChatMessageChange>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("38b09bc0-5029-54de-9165-dcd8b3b9b549"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.ChatMessageChange>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("98ebffc3-ef5f-58ac-9695-047a96c3f0b0"))
IIterator<ABI::Windows::ApplicationModel::Chat::ChatMessageTransport*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageTransport*, ABI::Windows::ApplicationModel::Chat::IChatMessageTransport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Chat.ChatMessageTransport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Chat::ChatMessageTransport*> __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_t;
#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("aae32f12-ed85-528c-8bad-6362d876ef8b"))
IIterable<ABI::Windows::ApplicationModel::Chat::ChatMessageTransport*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageTransport*, ABI::Windows::ApplicationModel::Chat::IChatMessageTransport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Chat.ChatMessageTransport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Chat::ChatMessageTransport*> __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport_t;
#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c7b1733f-6e87-5a65-8542-cf36a4521695"))
IVectorView<ABI::Windows::ApplicationModel::Chat::ChatMessageTransport*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageTransport*, ABI::Windows::ApplicationModel::Chat::IChatMessageTransport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.ChatMessageTransport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Chat::ChatMessageTransport*> __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_t;
#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d558786d-95ce-5f23-9e90-539902ac92a6"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.ChatMessageTransport>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a40a3b3a-6a96-5297-8afb-cbb497d48e15"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.ChatMessageTransport>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5d2c1b3f-9bc7-5126-93cd-f52326494de1"))
IIterator<ABI::Windows::ApplicationModel::Chat::IChatItem*> : IIterator_impl<ABI::Windows::ApplicationModel::Chat::IChatItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Chat.IChatItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Chat::IChatItem*> __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_t;
#define __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ea8e3309-099a-592e-9e28-441b151fe061"))
IIterable<ABI::Windows::ApplicationModel::Chat::IChatItem*> : IIterable_impl<ABI::Windows::ApplicationModel::Chat::IChatItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Chat.IChatItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Chat::IChatItem*> __FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem_t;
#define __FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4e46b3db-f003-5bfc-9eeb-f702f0801949"))
IVectorView<ABI::Windows::ApplicationModel::Chat::IChatItem*> : IVectorView_impl<ABI::Windows::ApplicationModel::Chat::IChatItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.IChatItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Chat::IChatItem*> __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_t;
#define __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("65b3eea1-7b6f-51a0-a2eb-7fb1dc473022"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.IChatItem>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("53daad1e-44e7-5a96-8688-2db7c00d8143"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.IChatItem>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a4eda5fb-fb99-56a7-8948-95c668a3ed3c"))
IIterator<ABI::Windows::ApplicationModel::Chat::RcsTransport*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::RcsTransport*, ABI::Windows::ApplicationModel::Chat::IRcsTransport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Chat.RcsTransport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Chat::RcsTransport*> __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_t;
#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("184c2264-65fd-5ad4-b22f-cb34e8eaec63"))
IIterable<ABI::Windows::ApplicationModel::Chat::RcsTransport*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::RcsTransport*, ABI::Windows::ApplicationModel::Chat::IRcsTransport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Chat.RcsTransport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Chat::RcsTransport*> __FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport_t;
#define __FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0627ce33-6cf1-5bc9-9508-938d67a494ed"))
IVectorView<ABI::Windows::ApplicationModel::Chat::RcsTransport*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::RcsTransport*, ABI::Windows::ApplicationModel::Chat::IRcsTransport*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.RcsTransport>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Chat::RcsTransport*> __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_t;
#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("17e1f428-6e60-5153-b0a6-5faf186561c1"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.RcsTransport>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f6555a8d-a624-5eb4-a2f3-f5b1c5c1a0d2"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.RcsTransport>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_USE */

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
            namespace Chat {
                class ChatMessageAttachment;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bc37f8a5-cd61-5054-a897-6d402b56b58b"))
IIterator<ABI::Windows::ApplicationModel::Chat::ChatMessageAttachment*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageAttachment*, ABI::Windows::ApplicationModel::Chat::IChatMessageAttachment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Chat.ChatMessageAttachment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Chat::ChatMessageAttachment*> __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_t;
#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8a1e36ac-13b6-577c-9b43-3d24a453f866"))
IIterable<ABI::Windows::ApplicationModel::Chat::ChatMessageAttachment*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageAttachment*, ABI::Windows::ApplicationModel::Chat::IChatMessageAttachment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Chat.ChatMessageAttachment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Chat::ChatMessageAttachment*> __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_t;
#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatRecipientDeliveryInfo;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3a7b0f38-ecd7-55f8-a5c5-e03d43a3978e"))
IIterator<ABI::Windows::ApplicationModel::Chat::ChatRecipientDeliveryInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatRecipientDeliveryInfo*, ABI::Windows::ApplicationModel::Chat::IChatRecipientDeliveryInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Chat::ChatRecipientDeliveryInfo*> __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_t;
#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e3d7b44c-4829-561b-a15e-a745e5adf6d1"))
IIterable<ABI::Windows::ApplicationModel::Chat::ChatRecipientDeliveryInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatRecipientDeliveryInfo*, ABI::Windows::ApplicationModel::Chat::IChatRecipientDeliveryInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Chat::ChatRecipientDeliveryInfo*> __FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_t;
#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class RcsEndUserMessageAction;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("75a37f34-d87d-555c-8e64-a1ea48a3dd20"))
IIterator<ABI::Windows::ApplicationModel::Chat::RcsEndUserMessageAction*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::RcsEndUserMessageAction*, ABI::Windows::ApplicationModel::Chat::IRcsEndUserMessageAction*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Chat.RcsEndUserMessageAction>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Chat::RcsEndUserMessageAction*> __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_t;
#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b6d21a55-471e-5a25-acdd-7784f7b4d7ba"))
IIterable<ABI::Windows::ApplicationModel::Chat::RcsEndUserMessageAction*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::RcsEndUserMessageAction*, ABI::Windows::ApplicationModel::Chat::IRcsEndUserMessageAction*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Chat.RcsEndUserMessageAction>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Chat::RcsEndUserMessageAction*> __FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_t;
#define __FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                typedef enum ChatMessageStatus : int ChatMessageStatus;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_USE
#define DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4f426a27-6d23-58b6-9f29-4f88ed274bcd"))
IKeyValuePair<HSTRING, enum ABI::Windows::ApplicationModel::Chat::ChatMessageStatus> : IKeyValuePair_impl<HSTRING, enum ABI::Windows::ApplicationModel::Chat::ChatMessageStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Chat.ChatMessageStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, enum ABI::Windows::ApplicationModel::Chat::ChatMessageStatus> __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_t;
#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5b099e05-07e2-5346-b075-f4297b3e308b"))
IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Chat.ChatMessageStatus>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus*> __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("57d87c13-48e9-546f-9b4e-a3906e1e7c24"))
IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, Windows.ApplicationModel.Chat.ChatMessageStatus>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus*> __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_USE
#define DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6a44ff0f-a172-5285-9ba5-b9fdd699a0fe"))
IMapView<HSTRING, enum ABI::Windows::ApplicationModel::Chat::ChatMessageStatus> : IMapView_impl<HSTRING, enum ABI::Windows::ApplicationModel::Chat::ChatMessageStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, Windows.ApplicationModel.Chat.ChatMessageStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, enum ABI::Windows::ApplicationModel::Chat::ChatMessageStatus> __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_t;
#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_USE */

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

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("50f5c38b-749f-5bf6-9560-11a9876f20d1"))
IVectorView<ABI::Windows::ApplicationModel::Chat::ChatMessageAttachment*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageAttachment*, ABI::Windows::ApplicationModel::Chat::IChatMessageAttachment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.ChatMessageAttachment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Chat::ChatMessageAttachment*> __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_t;
#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7897ffd3-7d86-5c89-bbe2-ca708d1a3398"))
IVectorView<ABI::Windows::ApplicationModel::Chat::ChatRecipientDeliveryInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatRecipientDeliveryInfo*, ABI::Windows::ApplicationModel::Chat::IChatRecipientDeliveryInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Chat::ChatRecipientDeliveryInfo*> __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_t;
#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9607e377-e873-5091-9e32-8695e8f50e7a"))
IVectorView<ABI::Windows::ApplicationModel::Chat::RcsEndUserMessageAction*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::RcsEndUserMessageAction*, ABI::Windows::ApplicationModel::Chat::IRcsEndUserMessageAction*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Chat.RcsEndUserMessageAction>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Chat::RcsEndUserMessageAction*> __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_t;
#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_USE */

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

#ifndef DEF___FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_USE
#define DEF___FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ad2127f1-3216-58fb-8154-b241a60b4252"))
IVector<ABI::Windows::ApplicationModel::Chat::ChatMessageAttachment*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageAttachment*, ABI::Windows::ApplicationModel::Chat::IChatMessageAttachment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.ApplicationModel.Chat.ChatMessageAttachment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::ApplicationModel::Chat::ChatMessageAttachment*> __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_t;
#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_USE
#define DEF___FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("14640361-3f8d-5606-8fcb-973208b76d72"))
IVector<ABI::Windows::ApplicationModel::Chat::ChatRecipientDeliveryInfo*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatRecipientDeliveryInfo*, ABI::Windows::ApplicationModel::Chat::IChatRecipientDeliveryInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::ApplicationModel::Chat::ChatRecipientDeliveryInfo*> __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_t;
#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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
        namespace ApplicationModel {
            namespace Chat {
                class RemoteParticipantComposingChangedEventArgs;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("09e43bb2-692d-5330-b9e7-adf69ad5c1a9"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Chat::ChatConversation*, ABI::Windows::ApplicationModel::Chat::RemoteParticipantComposingChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatConversation*, ABI::Windows::ApplicationModel::Chat::IChatConversation*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::RemoteParticipantComposingChangedEventArgs*, ABI::Windows::ApplicationModel::Chat::IRemoteParticipantComposingChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Chat.ChatConversation, Windows.ApplicationModel.Chat.RemoteParticipantComposingChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Chat::ChatConversation*, ABI::Windows::ApplicationModel::Chat::RemoteParticipantComposingChangedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatMessageChangedEventArgs;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2a4ed3d1-0d01-5133-b9e4-ddf68f099485"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Chat::ChatMessageStore*, ABI::Windows::ApplicationModel::Chat::ChatMessageChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageStore*, ABI::Windows::ApplicationModel::Chat::IChatMessageStore*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageChangedEventArgs*, ABI::Windows::ApplicationModel::Chat::IChatMessageChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Chat.ChatMessageStore, Windows.ApplicationModel.Chat.ChatMessageChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Chat::ChatMessageStore*, ABI::Windows::ApplicationModel::Chat::ChatMessageChangedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatMessageStoreChangedEventArgs;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3b5903d7-a037-5c7c-8336-88423d81e408"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Chat::ChatMessageStore*, ABI::Windows::ApplicationModel::Chat::ChatMessageStoreChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageStore*, ABI::Windows::ApplicationModel::Chat::IChatMessageStore*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::ChatMessageStoreChangedEventArgs*, ABI::Windows::ApplicationModel::Chat::IChatMessageStoreChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Chat.ChatMessageStore, Windows.ApplicationModel.Chat.ChatMessageStoreChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Chat::ChatMessageStore*, ABI::Windows::ApplicationModel::Chat::ChatMessageStoreChangedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class RcsEndUserMessageManager;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class RcsEndUserMessageAvailableEventArgs;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8d6beb36-f6ce-5769-96c2-25326eb463f6"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Chat::RcsEndUserMessageManager*, ABI::Windows::ApplicationModel::Chat::RcsEndUserMessageAvailableEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::RcsEndUserMessageManager*, ABI::Windows::ApplicationModel::Chat::IRcsEndUserMessageManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::RcsEndUserMessageAvailableEventArgs*, ABI::Windows::ApplicationModel::Chat::IRcsEndUserMessageAvailableEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Chat.RcsEndUserMessageManager, Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Chat::RcsEndUserMessageManager*, ABI::Windows::ApplicationModel::Chat::RcsEndUserMessageAvailableEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class RcsServiceKindSupportedChangedEventArgs;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e5f1c3a4-e498-50b0-91fe-94ebb01de0ab"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Chat::RcsTransport*, ABI::Windows::ApplicationModel::Chat::RcsServiceKindSupportedChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::RcsTransport*, ABI::Windows::ApplicationModel::Chat::IRcsTransport*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Chat::RcsServiceKindSupportedChangedEventArgs*, ABI::Windows::ApplicationModel::Chat::IRcsServiceKindSupportedChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Chat.RcsTransport, Windows.ApplicationModel.Chat.RcsServiceKindSupportedChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Chat::RcsTransport*, ABI::Windows::ApplicationModel::Chat::RcsServiceKindSupportedChangedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs_USE */

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
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                class MediaEncodingProfile;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingProfile;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile ABI::Windows::Media::MediaProperties::IMediaEncodingProfile

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                class WebAccount;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Credentials {
                interface IWebAccount;
            } /* Credentials */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount ABI::Windows::Security::Credentials::IWebAccount

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamReference;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference ABI::Windows::Storage::Streams::IRandomAccessStreamReference

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                typedef enum ChatConversationThreadingKind : int ChatConversationThreadingKind;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                typedef enum ChatItemKind : int ChatItemKind;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                typedef enum ChatMessageChangeType : int ChatMessageChangeType;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                typedef enum ChatMessageKind : int ChatMessageKind;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                typedef enum ChatMessageOperatorKind : int ChatMessageOperatorKind;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                typedef enum ChatMessageTransportKind : int ChatMessageTransportKind;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                typedef enum ChatMessageValidationStatus : int ChatMessageValidationStatus;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                typedef enum ChatRestoreHistorySpan : int ChatRestoreHistorySpan;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                typedef enum ChatStoreChangedEventKind : int ChatStoreChangedEventKind;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                typedef enum ChatTransportErrorCodeCategory : int ChatTransportErrorCodeCategory;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                typedef enum ChatTransportInterpretedErrorCode : int ChatTransportInterpretedErrorCode;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                typedef enum RcsServiceKind : int RcsServiceKind;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatConversationReader;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatConversationThreadingInfo;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatMessageChangeReader;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatMessageChangeTracker;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatMessageChangedDeferral;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatMessageReader;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatMessageTransportConfiguration;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatMessageValidationResult;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatQueryOptions;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatSearchReader;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class ChatSyncConfiguration;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class RcsEndUserMessage;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                class RcsTransportConfiguration;
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatConversationThreadingKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                enum ChatConversationThreadingKind : int
                {
                    ChatConversationThreadingKind_Participants = 0,
                    ChatConversationThreadingKind_ContactId = 1,
                    ChatConversationThreadingKind_ConversationId = 2,
                    ChatConversationThreadingKind_Custom = 3,
                };
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatItemKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                enum ChatItemKind : int
                {
                    ChatItemKind_Message = 0,
                    ChatItemKind_Conversation = 1,
                };
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatMessageChangeType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                enum ChatMessageChangeType : int
                {
                    ChatMessageChangeType_MessageCreated = 0,
                    ChatMessageChangeType_MessageModified = 1,
                    ChatMessageChangeType_MessageDeleted = 2,
                    ChatMessageChangeType_ChangeTrackingLost = 3,
                };
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatMessageKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                enum ChatMessageKind : int
                {
                    ChatMessageKind_Standard = 0,
                    ChatMessageKind_FileTransferRequest = 1,
                    ChatMessageKind_TransportCustom = 2,
                    ChatMessageKind_JoinedConversation = 3,
                    ChatMessageKind_LeftConversation = 4,
                    ChatMessageKind_OtherParticipantJoinedConversation = 5,
                    ChatMessageKind_OtherParticipantLeftConversation = 6,
                };
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatMessageOperatorKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                enum ChatMessageOperatorKind : int
                {
                    ChatMessageOperatorKind_Unspecified = 0,
                    ChatMessageOperatorKind_Sms = 1,
                    ChatMessageOperatorKind_Mms = 2,
                    ChatMessageOperatorKind_Rcs = 3,
                };
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatMessageStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                enum ChatMessageStatus : int
                {
                    ChatMessageStatus_Draft = 0,
                    ChatMessageStatus_Sending = 1,
                    ChatMessageStatus_Sent = 2,
                    ChatMessageStatus_SendRetryNeeded = 3,
                    ChatMessageStatus_SendFailed = 4,
                    ChatMessageStatus_Received = 5,
                    ChatMessageStatus_ReceiveDownloadNeeded = 6,
                    ChatMessageStatus_ReceiveDownloadFailed = 7,
                    ChatMessageStatus_ReceiveDownloading = 8,
                    ChatMessageStatus_Deleted = 9,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ChatMessageStatus_Declined = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ChatMessageStatus_Cancelled = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ChatMessageStatus_Recalled = 12,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    ChatMessageStatus_ReceiveRetryNeeded = 13,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatMessageTransportKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                enum ChatMessageTransportKind : int
                {
                    ChatMessageTransportKind_Text = 0,
                    ChatMessageTransportKind_Untriaged = 1,
                    ChatMessageTransportKind_Blocked = 2,
                    ChatMessageTransportKind_Custom = 3,
                };
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatMessageValidationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                enum ChatMessageValidationStatus : int
                {
                    ChatMessageValidationStatus_Valid = 0,
                    ChatMessageValidationStatus_NoRecipients = 1,
                    ChatMessageValidationStatus_InvalidData = 2,
                    ChatMessageValidationStatus_MessageTooLarge = 3,
                    ChatMessageValidationStatus_TooManyRecipients = 4,
                    ChatMessageValidationStatus_TransportInactive = 5,
                    ChatMessageValidationStatus_TransportNotFound = 6,
                    ChatMessageValidationStatus_TooManyAttachments = 7,
                    ChatMessageValidationStatus_InvalidRecipients = 8,
                    ChatMessageValidationStatus_InvalidBody = 9,
                    ChatMessageValidationStatus_InvalidOther = 10,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    ChatMessageValidationStatus_ValidWithLargeMessage = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    ChatMessageValidationStatus_VoiceRoamingRestriction = 12,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    ChatMessageValidationStatus_DataRoamingRestriction = 13,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                };
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatRestoreHistorySpan
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                enum ChatRestoreHistorySpan : int
                {
                    ChatRestoreHistorySpan_LastMonth = 0,
                    ChatRestoreHistorySpan_LastYear = 1,
                    ChatRestoreHistorySpan_AnyTime = 2,
                };
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatStoreChangedEventKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                enum ChatStoreChangedEventKind : int
                {
                    ChatStoreChangedEventKind_NotificationsMissed = 0,
                    ChatStoreChangedEventKind_StoreModified = 1,
                    ChatStoreChangedEventKind_MessageCreated = 2,
                    ChatStoreChangedEventKind_MessageModified = 3,
                    ChatStoreChangedEventKind_MessageDeleted = 4,
                    ChatStoreChangedEventKind_ConversationModified = 5,
                    ChatStoreChangedEventKind_ConversationDeleted = 6,
                    ChatStoreChangedEventKind_ConversationTransportDeleted = 7,
                };
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatTransportErrorCodeCategory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                enum ChatTransportErrorCodeCategory : int
                {
                    ChatTransportErrorCodeCategory_None = 0,
                    ChatTransportErrorCodeCategory_Http = 1,
                    ChatTransportErrorCodeCategory_Network = 2,
                    ChatTransportErrorCodeCategory_MmsServer = 3,
                };
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatTransportInterpretedErrorCode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                enum ChatTransportInterpretedErrorCode : int
                {
                    ChatTransportInterpretedErrorCode_None = 0,
                    ChatTransportInterpretedErrorCode_Unknown = 1,
                    ChatTransportInterpretedErrorCode_InvalidRecipientAddress = 2,
                    ChatTransportInterpretedErrorCode_NetworkConnectivity = 3,
                    ChatTransportInterpretedErrorCode_ServiceDenied = 4,
                    ChatTransportInterpretedErrorCode_Timeout = 5,
                };
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.RcsServiceKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                enum RcsServiceKind : int
                {
                    RcsServiceKind_Chat = 0,
                    RcsServiceKind_GroupChat = 1,
                    RcsServiceKind_FileTransfer = 2,
                    RcsServiceKind_Capability = 3,
                };
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatCapabilities[] = L"Windows.ApplicationModel.Chat.IChatCapabilities";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("3aff77bc-39c9-4dd1-ad2d-3964dd9d403f")
                IChatCapabilities : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsOnline(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsChatCapable(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsFileTransferCapable(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsGeoLocationPushCapable(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsIntegratedMessagingCapable(
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatCapabilities = __uuidof(IChatCapabilities);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatCapabilitiesManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatCapabilitiesManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatCapabilitiesManagerStatics[] = L"Windows.ApplicationModel.Chat.IChatCapabilitiesManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("b57a2f30-7041-458e-b0cf-7c0d9fea333a")
                IChatCapabilitiesManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetCachedCapabilitiesAsync(
                        HSTRING address,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCapabilitiesFromNetworkAsync(
                        HSTRING address,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatCapabilitiesManagerStatics = __uuidof(IChatCapabilitiesManagerStatics);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatCapabilitiesManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatCapabilitiesManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatCapabilitiesManagerStatics2[] = L"Windows.ApplicationModel.Chat.IChatCapabilitiesManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("e30d4274-d5c1-4ac9-9ffc-40e69184fec8")
                IChatCapabilitiesManagerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetCachedCapabilitiesForTransportAsync(
                        HSTRING address,
                        HSTRING transportId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCapabilitiesFromNetworkForTransportAsync(
                        HSTRING address,
                        HSTRING transportId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatCapabilitiesManagerStatics2 = __uuidof(IChatCapabilitiesManagerStatics2);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatConversation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatConversation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatConversation[] = L"Windows.ApplicationModel.Chat.IChatConversation";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("a58c080d-1a6f-46dc-8f3d-f5028660b6ee")
                IChatConversation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_HasUnreadMessages(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Subject(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Subject(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsConversationMuted(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsConversationMuted(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MostRecentMessageId(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Participants(
                        __FIVector_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ThreadingInfo(
                        ABI::Windows::ApplicationModel::Chat::IChatConversationThreadingInfo** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteAsync(
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetMessageReader(
                        ABI::Windows::ApplicationModel::Chat::IChatMessageReader** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE MarkAllMessagesAsReadAsync(
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE MarkMessagesAsReadAsync(
                        ABI::Windows::Foundation::DateTime value,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SaveAsync(
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyLocalParticipantComposing(
                        HSTRING transportId,
                        HSTRING participantAddress,
                        boolean isComposing
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE NotifyRemoteParticipantComposing(
                        HSTRING transportId,
                        HSTRING participantAddress,
                        boolean isComposing
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RemoteParticipantComposingChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RemoteParticipantComposingChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatConversation = __uuidof(IChatConversation);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatConversation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatConversation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatConversation2[] = L"Windows.ApplicationModel.Chat.IChatConversation2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("0a030cd1-983a-47aa-9a90-ee48ee997b59")
                IChatConversation2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CanModifyParticipants(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CanModifyParticipants(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatConversation2 = __uuidof(IChatConversation2);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatConversationReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatConversationReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatConversationReader[] = L"Windows.ApplicationModel.Chat.IChatConversationReader";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("055136d2-de32-4a47-a93a-b3dc0833852b")
                IChatConversationReader : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReadBatchAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadBatchWithCountAsync(
                        INT32 count,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatConversationReader = __uuidof(IChatConversationReader);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatConversationThreadingInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatConversationThreadingInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatConversationThreadingInfo[] = L"Windows.ApplicationModel.Chat.IChatConversationThreadingInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("331c21dc-7a07-4422-a32c-24be7c6dab24")
                IChatConversationThreadingInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContactId(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ContactId(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Custom(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Custom(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConversationId(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ConversationId(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Participants(
                        __FIVector_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::ApplicationModel::Chat::ChatConversationThreadingKind* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Kind(
                        ABI::Windows::ApplicationModel::Chat::ChatConversationThreadingKind value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatConversationThreadingInfo = __uuidof(IChatConversationThreadingInfo);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatItem[] = L"Windows.ApplicationModel.Chat.IChatItem";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("8751d000-ceb1-4243-b803-15d45a1dd428")
                IChatItem : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ItemKind(
                        ABI::Windows::ApplicationModel::Chat::ChatItemKind* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatItem = __uuidof(IChatItem);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatItem;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessage[] = L"Windows.ApplicationModel.Chat.IChatMessage";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("4b39052a-1142-5089-76da-f2db3d17cd05")
                IChatMessage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Attachments(
                        __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Body(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Body(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_From(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsForwardingDisabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsIncoming(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsRead(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LocalTimestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NetworkTimestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Recipients(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RecipientSendStatuses(
                        __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::ApplicationModel::Chat::ChatMessageStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Subject(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransportFriendlyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransportId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TransportId(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessage = __uuidof(IChatMessage);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessage2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessage3
 *     Windows.ApplicationModel.Chat.IChatMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessage2[] = L"Windows.ApplicationModel.Chat.IChatMessage2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("86668332-543f-49f5-ac71-6c2afc6565fd")
                IChatMessage2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_EstimatedDownloadSize(
                        UINT64* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_EstimatedDownloadSize(
                        UINT64 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_From(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsAutoReply(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsAutoReply(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsForwardingDisabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsReplyDisabled(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsIncoming(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsRead(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSeen(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsSeen(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSimMessage(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LocalTimestamp(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MessageKind(
                        ABI::Windows::ApplicationModel::Chat::ChatMessageKind* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MessageKind(
                        ABI::Windows::ApplicationModel::Chat::ChatMessageKind value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MessageOperatorKind(
                        ABI::Windows::ApplicationModel::Chat::ChatMessageOperatorKind* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MessageOperatorKind(
                        ABI::Windows::ApplicationModel::Chat::ChatMessageOperatorKind value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_NetworkTimestamp(
                        ABI::Windows::Foundation::DateTime value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsReceivedDuringQuietHours(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsReceivedDuringQuietHours(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RemoteId(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Status(
                        ABI::Windows::ApplicationModel::Chat::ChatMessageStatus value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Subject(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShouldSuppressNotification(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ShouldSuppressNotification(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ThreadingInfo(
                        ABI::Windows::ApplicationModel::Chat::IChatConversationThreadingInfo** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ThreadingInfo(
                        ABI::Windows::ApplicationModel::Chat::IChatConversationThreadingInfo* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RecipientsDeliveryInfos(
                        __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessage2 = __uuidof(IChatMessage2);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessage3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessage3[] = L"Windows.ApplicationModel.Chat.IChatMessage3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("74eb2fb0-3ba7-459f-8e0b-e8af0febd9ad")
                IChatMessage3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RemoteId(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessage3 = __uuidof(IChatMessage3);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessage4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessage4[] = L"Windows.ApplicationModel.Chat.IChatMessage4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("2d144b0f-d2bf-460c-aa68-6d3f8483c9bf")
                IChatMessage4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SyncId(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SyncId(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessage4 = __uuidof(IChatMessage4);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageAttachment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageAttachment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageAttachment[] = L"Windows.ApplicationModel.Chat.IChatMessageAttachment";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("c7c4fd74-bf63-58eb-508c-8b863ff16b67")
                IChatMessageAttachment : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DataStreamReference(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DataStreamReference(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_GroupId(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_GroupId(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MimeType(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MimeType(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Text(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageAttachment = __uuidof(IChatMessageAttachment);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageAttachment2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageAttachment
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessageAttachment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageAttachment2[] = L"Windows.ApplicationModel.Chat.IChatMessageAttachment2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("5ed99270-7dd1-4a87-a8ce-acdd87d80dc8")
                IChatMessageAttachment2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Thumbnail(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Thumbnail(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransferProgress(
                        DOUBLE* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TransferProgress(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OriginalFileName(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OriginalFileName(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageAttachment2 = __uuidof(IChatMessageAttachment2);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageAttachmentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageAttachment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageAttachmentFactory[] = L"Windows.ApplicationModel.Chat.IChatMessageAttachmentFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("205852a2-a356-5b71-6ca9-66c985b7d0d5")
                IChatMessageAttachmentFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateChatMessageAttachment(
                        HSTRING mimeType,
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference* dataStreamReference,
                        ABI::Windows::ApplicationModel::Chat::IChatMessageAttachment** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageAttachmentFactory = __uuidof(IChatMessageAttachmentFactory);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageBlockingStatic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageBlocking
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageBlockingStatic[] = L"Windows.ApplicationModel.Chat.IChatMessageBlockingStatic";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("f6b9a380-cdea-11e4-8830-0800200c9a66")
                IChatMessageBlockingStatic : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE MarkMessageAsBlockedAsync(
                        HSTRING localChatMessageId,
                        boolean blocked,
                        ABI::Windows::Foundation::IAsyncAction** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageBlockingStatic = __uuidof(IChatMessageBlockingStatic);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageChange
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageChange[] = L"Windows.ApplicationModel.Chat.IChatMessageChange";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("1c18c355-421e-54b8-6d38-6b3a6c82fccc")
                IChatMessageChange : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ChangeType(
                        ABI::Windows::ApplicationModel::Chat::ChatMessageChangeType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Message(
                        ABI::Windows::ApplicationModel::Chat::IChatMessage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageChange = __uuidof(IChatMessageChange);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageChangeReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageChangeReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageChangeReader[] = L"Windows.ApplicationModel.Chat.IChatMessageChangeReader";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("14267020-28ce-5f26-7b05-9a5c7cce87ca")
                IChatMessageChangeReader : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE AcceptChanges(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AcceptChangesThrough(
                        ABI::Windows::ApplicationModel::Chat::IChatMessageChange* lastChangeToAcknowledge
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadBatchAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageChangeReader = __uuidof(IChatMessageChangeReader);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageChangeTracker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageChangeTracker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageChangeTracker[] = L"Windows.ApplicationModel.Chat.IChatMessageChangeTracker";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("60b7f066-70a0-5224-508c-242ef7c1d06f")
                IChatMessageChangeTracker : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Enable(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetChangeReader(
                        ABI::Windows::ApplicationModel::Chat::IChatMessageChangeReader** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Reset(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageChangeTracker = __uuidof(IChatMessageChangeTracker);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageChangedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageChangedDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageChangedDeferral[] = L"Windows.ApplicationModel.Chat.IChatMessageChangedDeferral";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("fbc6b30c-788c-4dcc-ace7-6282382968cf")
                IChatMessageChangedDeferral : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageChangedDeferral = __uuidof(IChatMessageChangedDeferral);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageChangedEventArgs[] = L"Windows.ApplicationModel.Chat.IChatMessageChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("b6b73e2d-691c-4edf-8660-6eb9896892e3")
                IChatMessageChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::ApplicationModel::Chat::IChatMessageChangedDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageChangedEventArgs = __uuidof(IChatMessageChangedEventArgs);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageManager2Statics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessageManagerStatic
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageManager2Statics[] = L"Windows.ApplicationModel.Chat.IChatMessageManager2Statics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("1d45390f-9f4f-4e35-964e-1b9ca61ac044")
                IChatMessageManager2Statics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RegisterTransportAsync(
                        __FIAsyncOperation_1_HSTRING** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTransportAsync(
                        HSTRING transportId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageManager2Statics = __uuidof(IChatMessageManager2Statics);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageManagerStatic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageManagerStatic[] = L"Windows.ApplicationModel.Chat.IChatMessageManagerStatic";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("f15c60f7-d5e8-5e92-556d-e03b60253104")
                IChatMessageManagerStatic : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetTransportsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestStoreAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowComposeSmsMessageAsync(
                        ABI::Windows::ApplicationModel::Chat::IChatMessage* message,
                        ABI::Windows::Foundation::IAsyncAction** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ShowSmsSettings(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageManagerStatic = __uuidof(IChatMessageManagerStatic);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageManagerStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessageManagerStatic
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageManagerStatics3[] = L"Windows.ApplicationModel.Chat.IChatMessageManagerStatics3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("208b830d-6755-48cc-9ab3-fd03c463fc92")
                IChatMessageManagerStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE RequestSyncManagerAsync(
                        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageManagerStatics3 = __uuidof(IChatMessageManagerStatics3);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageNotificationTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageNotificationTriggerDetails[] = L"Windows.ApplicationModel.Chat.IChatMessageNotificationTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("fd344dfb-3063-4e17-8586-c6c08262e6c0")
                IChatMessageNotificationTriggerDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ChatMessage(
                        ABI::Windows::ApplicationModel::Chat::IChatMessage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageNotificationTriggerDetails = __uuidof(IChatMessageNotificationTriggerDetails);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageNotificationTriggerDetails2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessageNotificationTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageNotificationTriggerDetails2[] = L"Windows.ApplicationModel.Chat.IChatMessageNotificationTriggerDetails2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("6bb522e0-aa07-4fd1-9471-77934fb75ee6")
                IChatMessageNotificationTriggerDetails2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ShouldDisplayToast(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShouldUpdateDetailText(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShouldUpdateBadge(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ShouldUpdateActionCenter(
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageNotificationTriggerDetails2 = __uuidof(IChatMessageNotificationTriggerDetails2);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageReader[] = L"Windows.ApplicationModel.Chat.IChatMessageReader";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("b6ea78ce-4489-56f9-76aa-e204682514cf")
                IChatMessageReader : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReadBatchAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageReader = __uuidof(IChatMessageReader);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageReader2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageReader2[] = L"Windows.ApplicationModel.Chat.IChatMessageReader2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("89643683-64bb-470d-9df4-0de8be1a05bf")
                IChatMessageReader2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReadBatchWithCountAsync(
                        INT32 count,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageReader2 = __uuidof(IChatMessageReader2);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageStore[] = L"Windows.ApplicationModel.Chat.IChatMessageStore";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("31f2fd01-ccf6-580b-4976-0a07dd5d3b47")
                IChatMessageStore : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ChangeTracker(
                        ABI::Windows::ApplicationModel::Chat::IChatMessageChangeTracker** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DeleteMessageAsync(
                        HSTRING localMessageId,
                        ABI::Windows::Foundation::IAsyncAction** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DownloadMessageAsync(
                        HSTRING localChatMessageId,
                        ABI::Windows::Foundation::IAsyncAction** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetMessageAsync(
                        HSTRING localChatMessageId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetMessageReader1(
                        ABI::Windows::ApplicationModel::Chat::IChatMessageReader** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetMessageReader2(
                        ABI::Windows::Foundation::TimeSpan recentTimeLimit,
                        ABI::Windows::ApplicationModel::Chat::IChatMessageReader** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE MarkMessageReadAsync(
                        HSTRING localChatMessageId,
                        ABI::Windows::Foundation::IAsyncAction** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RetrySendMessageAsync(
                        HSTRING localChatMessageId,
                        ABI::Windows::Foundation::IAsyncAction** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendMessageAsync(
                        ABI::Windows::ApplicationModel::Chat::IChatMessage* chatMessage,
                        ABI::Windows::Foundation::IAsyncAction** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ValidateMessage(
                        ABI::Windows::ApplicationModel::Chat::IChatMessage* chatMessage,
                        ABI::Windows::ApplicationModel::Chat::IChatMessageValidationResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_MessageChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs* value,
                        EventRegistrationToken* returnValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_MessageChanged(
                        EventRegistrationToken value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageStore = __uuidof(IChatMessageStore);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageStore2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageStore
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessageStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageStore2[] = L"Windows.ApplicationModel.Chat.IChatMessageStore2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("ad4dc4ee-3ad4-491b-b311-abdf9bb22768")
                IChatMessageStore2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ForwardMessageAsync(
                        HSTRING localChatMessageId,
                        __FIIterable_1_HSTRING* addresses,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetConversationAsync(
                        HSTRING conversationId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetConversationForTransportsAsync(
                        HSTRING conversationId,
                        __FIIterable_1_HSTRING* transportIds,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetConversationFromThreadingInfoAsync(
                        ABI::Windows::ApplicationModel::Chat::IChatConversationThreadingInfo* threadingInfo,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetConversationReader(
                        ABI::Windows::ApplicationModel::Chat::IChatConversationReader** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetConversationForTransportsReader(
                        __FIIterable_1_HSTRING* transportIds,
                        ABI::Windows::ApplicationModel::Chat::IChatConversationReader** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetMessageByRemoteIdAsync(
                        HSTRING transportId,
                        HSTRING remoteId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetUnseenCountAsync(
                        __FIAsyncOperation_1_int** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetUnseenCountForTransportsReaderAsync(
                        __FIIterable_1_HSTRING* transportIds,
                        __FIAsyncOperation_1_int** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE MarkAsSeenAsync(
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE MarkAsSeenForTransportsAsync(
                        __FIIterable_1_HSTRING* transportIds,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetSearchReader(
                        ABI::Windows::ApplicationModel::Chat::IChatQueryOptions* value,
                        ABI::Windows::ApplicationModel::Chat::IChatSearchReader** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SaveMessageAsync(
                        ABI::Windows::ApplicationModel::Chat::IChatMessage* chatMessage,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryCancelDownloadMessageAsync(
                        HSTRING localChatMessageId,
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryCancelSendMessageAsync(
                        HSTRING localChatMessageId,
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_StoreChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StoreChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageStore2 = __uuidof(IChatMessageStore2);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageStore3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageStore
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessageStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageStore3[] = L"Windows.ApplicationModel.Chat.IChatMessageStore3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("9adbbb09-4345-4ec1-8b74-b7338243719c")
                IChatMessageStore3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetMessageBySyncIdAsync(
                        HSTRING syncId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageStore3 = __uuidof(IChatMessageStore3);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageStoreChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageStoreChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageStoreChangedEventArgs[] = L"Windows.ApplicationModel.Chat.IChatMessageStoreChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("65c66fac-fe8c-46d4-9119-57b8410311d5")
                IChatMessageStoreChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::ApplicationModel::Chat::ChatStoreChangedEventKind* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageStoreChangedEventArgs = __uuidof(IChatMessageStoreChangedEventArgs);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageTransport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageTransport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageTransport[] = L"Windows.ApplicationModel.Chat.IChatMessageTransport";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("63a9dbf8-e6b3-5c9a-5f85-d47925b9bd18")
                IChatMessageTransport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsAppSetAsNotificationProvider(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsActive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransportFriendlyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransportId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestSetAsNotificationProviderAsync(
                        ABI::Windows::Foundation::IAsyncAction** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageTransport = __uuidof(IChatMessageTransport);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageTransport2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageTransport
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessageTransport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageTransport2[] = L"Windows.ApplicationModel.Chat.IChatMessageTransport2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("90a75622-d84a-4c22-a94d-544444edc8a1")
                IChatMessageTransport2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Configuration(
                        ABI::Windows::ApplicationModel::Chat::IChatMessageTransportConfiguration** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransportKind(
                        ABI::Windows::ApplicationModel::Chat::ChatMessageTransportKind* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageTransport2 = __uuidof(IChatMessageTransport2);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageTransportConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageTransportConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageTransportConfiguration[] = L"Windows.ApplicationModel.Chat.IChatMessageTransportConfiguration";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("879ff725-1a08-4aca-a075-3355126312e6")
                IChatMessageTransportConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MaxAttachmentCount(
                        INT32* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxMessageSizeInKilobytes(
                        INT32* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxRecipientCount(
                        INT32* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportedVideoFormat(
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedProperties(
                        __FIMapView_2_HSTRING_IInspectable** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageTransportConfiguration = __uuidof(IChatMessageTransportConfiguration);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageValidationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageValidationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageValidationResult[] = L"Windows.ApplicationModel.Chat.IChatMessageValidationResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("25e93a03-28ec-5889-569b-7e486b126f18")
                IChatMessageValidationResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MaxPartCount(
                        __FIReference_1_UINT32** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PartCount(
                        __FIReference_1_UINT32** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RemainingCharacterCountInPart(
                        __FIReference_1_UINT32** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::ApplicationModel::Chat::ChatMessageValidationStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatMessageValidationResult = __uuidof(IChatMessageValidationResult);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatQueryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatQueryOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatQueryOptions[] = L"Windows.ApplicationModel.Chat.IChatQueryOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("2fd364a6-bf36-42f7-b7e7-923c0aabfe16")
                IChatQueryOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SearchString(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SearchString(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatQueryOptions = __uuidof(IChatQueryOptions);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatRecipientDeliveryInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatRecipientDeliveryInfo[] = L"Windows.ApplicationModel.Chat.IChatRecipientDeliveryInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("ffc7b2a2-283c-4c0a-8a0e-8c33bdbf0545")
                IChatRecipientDeliveryInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TransportAddress(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TransportAddress(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeliveryTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DeliveryTime(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReadTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ReadTime(
                        __FIReference_1_Windows__CFoundation__CDateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransportErrorCodeCategory(
                        ABI::Windows::ApplicationModel::Chat::ChatTransportErrorCodeCategory* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransportInterpretedErrorCode(
                        ABI::Windows::ApplicationModel::Chat::ChatTransportInterpretedErrorCode* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransportErrorCode(
                        INT32* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsErrorPermanent(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::ApplicationModel::Chat::ChatMessageStatus* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatRecipientDeliveryInfo = __uuidof(IChatRecipientDeliveryInfo);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatSearchReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatSearchReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatSearchReader[] = L"Windows.ApplicationModel.Chat.IChatSearchReader";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("4665fe49-9020-4752-980d-39612325f589")
                IChatSearchReader : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReadBatchAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReadBatchWithCountAsync(
                        INT32 count,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatSearchReader = __uuidof(IChatSearchReader);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatSyncConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatSyncConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatSyncConfiguration[] = L"Windows.ApplicationModel.Chat.IChatSyncConfiguration";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("09f869b2-69f4-4aff-82b6-06992ff402d2")
                IChatSyncConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsSyncEnabled(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsSyncEnabled(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RestoreHistorySpan(
                        ABI::Windows::ApplicationModel::Chat::ChatRestoreHistorySpan* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RestoreHistorySpan(
                        ABI::Windows::ApplicationModel::Chat::ChatRestoreHistorySpan value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatSyncConfiguration = __uuidof(IChatSyncConfiguration);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatSyncManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatSyncManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatSyncManager[] = L"Windows.ApplicationModel.Chat.IChatSyncManager";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("7ba52c63-2650-486f-b4b4-6bd9d3d63c84")
                IChatSyncManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Configuration(
                        ABI::Windows::ApplicationModel::Chat::IChatSyncConfiguration** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AssociateAccountAsync(
                        ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UnassociateAccountAsync(
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsAccountAssociated(
                        ABI::Windows::Security::Credentials::IWebAccount* webAccount,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartSync(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetConfigurationAsync(
                        ABI::Windows::ApplicationModel::Chat::IChatSyncConfiguration* configuration,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IChatSyncManager = __uuidof(IChatSyncManager);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsEndUserMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsEndUserMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsEndUserMessage[] = L"Windows.ApplicationModel.Chat.IRcsEndUserMessage";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("d7cda5eb-cbd7-4f3b-8526-b506dec35c53")
                IRcsEndUserMessage : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TransportId(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsPinRequired(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Actions(
                        __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendResponseAsync(
                        ABI::Windows::ApplicationModel::Chat::IRcsEndUserMessageAction* action,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SendResponseWithPinAsync(
                        ABI::Windows::ApplicationModel::Chat::IRcsEndUserMessageAction* action,
                        HSTRING pin,
                        ABI::Windows::Foundation::IAsyncAction** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRcsEndUserMessage = __uuidof(IRcsEndUserMessage);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsEndUserMessageAction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsEndUserMessageAction
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsEndUserMessageAction[] = L"Windows.ApplicationModel.Chat.IRcsEndUserMessageAction";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("92378737-9b42-46d3-9d5e-3c1b2dae7cb8")
                IRcsEndUserMessageAction : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Label(
                        HSTRING* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRcsEndUserMessageAction = __uuidof(IRcsEndUserMessageAction);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsEndUserMessageAvailableEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsEndUserMessageAvailableEventArgs[] = L"Windows.ApplicationModel.Chat.IRcsEndUserMessageAvailableEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("2d45ae01-3f89-41ea-9702-9e9ed411aa98")
                IRcsEndUserMessageAvailableEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsMessageAvailable(
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Message(
                        ABI::Windows::ApplicationModel::Chat::IRcsEndUserMessage** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRcsEndUserMessageAvailableEventArgs = __uuidof(IRcsEndUserMessageAvailableEventArgs);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsEndUserMessageAvailableTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsEndUserMessageAvailableTriggerDetails[] = L"Windows.ApplicationModel.Chat.IRcsEndUserMessageAvailableTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("5b97742d-351f-4692-b41e-1b035dc18986")
                IRcsEndUserMessageAvailableTriggerDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Text(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRcsEndUserMessageAvailableTriggerDetails = __uuidof(IRcsEndUserMessageAvailableTriggerDetails);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsEndUserMessageManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsEndUserMessageManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsEndUserMessageManager[] = L"Windows.ApplicationModel.Chat.IRcsEndUserMessageManager";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("3054ae5a-4d1f-4b59-9433-126c734e86a6")
                IRcsEndUserMessageManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_MessageAvailableChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_MessageAvailableChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRcsEndUserMessageManager = __uuidof(IRcsEndUserMessageManager);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsManagerStatics[] = L"Windows.ApplicationModel.Chat.IRcsManagerStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("7d270ac5-0abd-4f31-9b99-a59e71a7b731")
                IRcsManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetEndUserMessageManager(
                        ABI::Windows::ApplicationModel::Chat::IRcsEndUserMessageManager** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTransportsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTransportAsync(
                        HSTRING transportId,
                        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE LeaveConversationAsync(
                        ABI::Windows::ApplicationModel::Chat::IChatConversation* conversation,
                        ABI::Windows::Foundation::IAsyncAction** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRcsManagerStatics = __uuidof(IRcsManagerStatics);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsManagerStatics2[] = L"Windows.ApplicationModel.Chat.IRcsManagerStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("cd49ad18-ad8a-42aa-8eeb-a798a8808959")
                IRcsManagerStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_TransportListChanged(
                        __FIEventHandler_1_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_TransportListChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRcsManagerStatics2 = __uuidof(IRcsManagerStatics2);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsServiceKindSupportedChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsServiceKindSupportedChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsServiceKindSupportedChangedEventArgs[] = L"Windows.ApplicationModel.Chat.IRcsServiceKindSupportedChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("f47ea244-e783-4866-b3a7-4e5ccf023070")
                IRcsServiceKindSupportedChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ServiceKind(
                        ABI::Windows::ApplicationModel::Chat::RcsServiceKind* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRcsServiceKindSupportedChangedEventArgs = __uuidof(IRcsServiceKindSupportedChangedEventArgs);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsTransport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsTransport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsTransport[] = L"Windows.ApplicationModel.Chat.IRcsTransport";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("fea34759-f37c-4319-8546-ec84d21d30ff")
                IRcsTransport : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedProperties(
                        __FIMapView_2_HSTRING_IInspectable** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsActive(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransportFriendlyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransportId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Configuration(
                        ABI::Windows::ApplicationModel::Chat::IRcsTransportConfiguration** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsStoreAndForwardEnabled(
                        ABI::Windows::ApplicationModel::Chat::RcsServiceKind serviceKind,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsServiceKindSupported(
                        ABI::Windows::ApplicationModel::Chat::RcsServiceKind serviceKind,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ServiceKindSupportedChanged(
                        __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ServiceKindSupportedChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRcsTransport = __uuidof(IRcsTransport);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsTransportConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsTransportConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsTransportConfiguration[] = L"Windows.ApplicationModel.Chat.IRcsTransportConfiguration";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("1fccb102-2472-4bb9-9988-c1211c83e8a9")
                IRcsTransportConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MaxAttachmentCount(
                        INT32* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxMessageSizeInKilobytes(
                        INT32* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxGroupMessageSizeInKilobytes(
                        INT32* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxRecipientCount(
                        INT32* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxFileSizeInKilobytes(
                        INT32* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WarningFileSizeInKilobytes(
                        INT32* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRcsTransportConfiguration = __uuidof(IRcsTransportConfiguration);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRemoteParticipantComposingChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RemoteParticipantComposingChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRemoteParticipantComposingChangedEventArgs[] = L"Windows.ApplicationModel.Chat.IRemoteParticipantComposingChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Chat {
                MIDL_INTERFACE("1ec045a7-cfc9-45c9-9876-449f2bc180f5")
                IRemoteParticipantComposingChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TransportId(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ParticipantAddress(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsComposing(
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IRemoteParticipantComposingChangedEventArgs = __uuidof(IRemoteParticipantComposingChangedEventArgs);
            } /* Chat */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatCapabilities ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatCapabilities_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatCapabilities[] = L"Windows.ApplicationModel.Chat.ChatCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatCapabilitiesManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Chat.IChatCapabilitiesManagerStatics2 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Chat.IChatCapabilitiesManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatCapabilitiesManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatCapabilitiesManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatCapabilitiesManager[] = L"Windows.ApplicationModel.Chat.ChatCapabilitiesManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatConversation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatConversation ** Default Interface **
 *    Windows.ApplicationModel.Chat.IChatConversation2
 *    Windows.ApplicationModel.Chat.IChatItem
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatConversation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatConversation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatConversation[] = L"Windows.ApplicationModel.Chat.ChatConversation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatConversationReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatConversationReader ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatConversationReader_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatConversationReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatConversationReader[] = L"Windows.ApplicationModel.Chat.ChatConversationReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatConversationThreadingInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatConversationThreadingInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatConversationThreadingInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatConversationThreadingInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatConversationThreadingInfo[] = L"Windows.ApplicationModel.Chat.ChatConversationThreadingInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessage ** Default Interface **
 *    Windows.ApplicationModel.Chat.IChatMessage2
 *    Windows.ApplicationModel.Chat.IChatMessage3
 *    Windows.ApplicationModel.Chat.IChatMessage4
 *    Windows.ApplicationModel.Chat.IChatItem
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessage_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessage[] = L"Windows.ApplicationModel.Chat.ChatMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageAttachment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Chat.IChatMessageAttachmentFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageAttachment ** Default Interface **
 *    Windows.ApplicationModel.Chat.IChatMessageAttachment2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageAttachment_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageAttachment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageAttachment[] = L"Windows.ApplicationModel.Chat.ChatMessageAttachment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageBlocking
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Chat.IChatMessageBlockingStatic interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageBlocking_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageBlocking_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageBlocking[] = L"Windows.ApplicationModel.Chat.ChatMessageBlocking";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageChange ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChange_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChange_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageChange[] = L"Windows.ApplicationModel.Chat.ChatMessageChange";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageChangeReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageChangeReader ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChangeReader_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChangeReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageChangeReader[] = L"Windows.ApplicationModel.Chat.ChatMessageChangeReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageChangeTracker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageChangeTracker ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChangeTracker_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChangeTracker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageChangeTracker[] = L"Windows.ApplicationModel.Chat.ChatMessageChangeTracker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageChangedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageChangedDeferral ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChangedDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChangedDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageChangedDeferral[] = L"Windows.ApplicationModel.Chat.ChatMessageChangedDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageChangedEventArgs[] = L"Windows.ApplicationModel.Chat.ChatMessageChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Chat.IChatMessageManagerStatic interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Chat.IChatMessageManager2Statics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Chat.IChatMessageManagerStatics3 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageManager[] = L"Windows.ApplicationModel.Chat.ChatMessageManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageNotificationTriggerDetails ** Default Interface **
 *    Windows.ApplicationModel.Chat.IChatMessageNotificationTriggerDetails2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageNotificationTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageNotificationTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageNotificationTriggerDetails[] = L"Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageReader ** Default Interface **
 *    Windows.ApplicationModel.Chat.IChatMessageReader2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageReader_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageReader[] = L"Windows.ApplicationModel.Chat.ChatMessageReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageStore ** Default Interface **
 *    Windows.ApplicationModel.Chat.IChatMessageStore2
 *    Windows.ApplicationModel.Chat.IChatMessageStore3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageStore_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageStore[] = L"Windows.ApplicationModel.Chat.ChatMessageStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageStoreChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageStoreChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageStoreChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageStoreChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageStoreChangedEventArgs[] = L"Windows.ApplicationModel.Chat.ChatMessageStoreChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageTransport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageTransport ** Default Interface **
 *    Windows.ApplicationModel.Chat.IChatMessageTransport2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageTransport_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageTransport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageTransport[] = L"Windows.ApplicationModel.Chat.ChatMessageTransport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageTransportConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageTransportConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageTransportConfiguration_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageTransportConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageTransportConfiguration[] = L"Windows.ApplicationModel.Chat.ChatMessageTransportConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageValidationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageValidationResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageValidationResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageValidationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageValidationResult[] = L"Windows.ApplicationModel.Chat.ChatMessageValidationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatQueryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatQueryOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatQueryOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatQueryOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatQueryOptions[] = L"Windows.ApplicationModel.Chat.ChatQueryOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatRecipientDeliveryInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatRecipientDeliveryInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatRecipientDeliveryInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatRecipientDeliveryInfo[] = L"Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatSearchReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatSearchReader ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatSearchReader_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatSearchReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatSearchReader[] = L"Windows.ApplicationModel.Chat.ChatSearchReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatSyncConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatSyncConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatSyncConfiguration_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatSyncConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatSyncConfiguration[] = L"Windows.ApplicationModel.Chat.ChatSyncConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatSyncManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatSyncManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatSyncManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatSyncManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatSyncManager[] = L"Windows.ApplicationModel.Chat.ChatSyncManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsEndUserMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRcsEndUserMessage ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessage_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsEndUserMessage[] = L"Windows.ApplicationModel.Chat.RcsEndUserMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsEndUserMessageAction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRcsEndUserMessageAction ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessageAction_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessageAction_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsEndUserMessageAction[] = L"Windows.ApplicationModel.Chat.RcsEndUserMessageAction";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRcsEndUserMessageAvailableEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessageAvailableEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessageAvailableEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsEndUserMessageAvailableEventArgs[] = L"Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRcsEndUserMessageAvailableTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessageAvailableTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessageAvailableTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsEndUserMessageAvailableTriggerDetails[] = L"Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsEndUserMessageManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRcsEndUserMessageManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessageManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessageManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsEndUserMessageManager[] = L"Windows.ApplicationModel.Chat.RcsEndUserMessageManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Chat.IRcsManagerStatics2 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Chat.IRcsManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsManager[] = L"Windows.ApplicationModel.Chat.RcsManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsServiceKindSupportedChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRcsServiceKindSupportedChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsServiceKindSupportedChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsServiceKindSupportedChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsServiceKindSupportedChangedEventArgs[] = L"Windows.ApplicationModel.Chat.RcsServiceKindSupportedChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsTransport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRcsTransport ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsTransport_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsTransport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsTransport[] = L"Windows.ApplicationModel.Chat.RcsTransport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsTransportConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRcsTransportConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsTransportConfiguration_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsTransportConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsTransportConfiguration[] = L"Windows.ApplicationModel.Chat.RcsTransportConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RemoteParticipantComposingChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRemoteParticipantComposingChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RemoteParticipantComposingChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RemoteParticipantComposingChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RemoteParticipantComposingChangedEventArgs[] = L"Windows.ApplicationModel.Chat.RemoteParticipantComposingChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2 __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2 __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatItem __x_ABI_CWindows_CApplicationModel_CChat_CIChatItem;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2 __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3 __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4 __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2 __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3 __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2 __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2 __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2 __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3 __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2 __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions __x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader __x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2 __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs __x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_int __FIAsyncOperationCompletedHandler_1_int;

#if !defined(____FIAsyncOperation_1_int_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_int_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_int __FIAsyncOperation_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_int;

typedef struct __FIAsyncOperation_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_int* This,
        __FIAsyncOperationCompletedHandler_1_int* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_int* This,
        __FIAsyncOperationCompletedHandler_1_int** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_int* This,
        INT32* result);

    END_INTERFACE
} __FIAsyncOperation_1_intVtbl;

interface __FIAsyncOperation_1_int
{
    CONST_VTBL struct __FIAsyncOperation_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_int_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_int_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_int_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_int_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_int_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_int_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_int __FIAsyncOperationCompletedHandler_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_int;

typedef struct __FIAsyncOperationCompletedHandler_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_int* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_int* This,
        __FIAsyncOperation_1_int* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_intVtbl;

interface __FIAsyncOperationCompletedHandler_1_int
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_int_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_int_INTERFACE_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilitiesVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilitiesVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatCapabilities_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversationVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversationVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStoreVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStoreVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageStore_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManagerVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManagerVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CChatSyncManager_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation;

typedef struct __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversationVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation __FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation;

typedef struct __FIIterable_1_Windows__CApplicationModel__CChat__CChatConversationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        __FIIterator_1_Windows__CApplicationModel__CChat__CChatConversation** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CChat__CChatConversationVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CChat__CChatConversationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversationVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        __FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversationVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversationVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage;

typedef struct __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage;

typedef struct __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessage** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange;

typedef struct __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChangeVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange;

typedef struct __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageChange** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChangeVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageChange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChangeVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChangeVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChangeVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport;

typedef struct __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport;

typedef struct __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageTransport** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem;

typedef struct __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatItem** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatItem** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItemVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem __FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem;

typedef struct __FIIterable_1_Windows__CApplicationModel__CChat__CIChatItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        __FIIterator_1_Windows__CApplicationModel__CChat__CIChatItem** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CChat__CIChatItemVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CChat__CIChatItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CChat__CIChatItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatItem** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItemVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        __FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItemVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItemVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport;

typedef struct __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport __FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport;

typedef struct __FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        __FIIterator_1_Windows__CApplicationModel__CChat__CRcsTransport** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport_INTERFACE_DEFINED__
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment;

typedef struct __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachmentVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment;

typedef struct __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        __FIIterator_1_Windows__CApplicationModel__CChat__CChatMessageAttachment** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachmentVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo;

typedef struct __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfoVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo __FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo;

typedef struct __FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        __FIIterator_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfoVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction;

typedef struct __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageActionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageActionVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageActionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction __FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction;

typedef struct __FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageActionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        __FIIterator_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageActionVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageActionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

typedef enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageStatus __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus;

typedef struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageStatus* result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatusVtbl;

interface __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatusVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatusVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus;

typedef struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        HSTRING key,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageStatus* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus** first,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatusVtbl;

interface __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus
{
    CONST_VTBL struct __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus_INTERFACE_DEFINED__
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
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachmentVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfoVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageActionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageActionVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageActionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction_INTERFACE_DEFINED__
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
#if !defined(____FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment;

typedef struct __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        __FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageAttachment** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment** items);

    END_INTERFACE
} __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachmentVtbl;

interface __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment
{
    CONST_VTBL struct __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo;

typedef struct __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        __FIVectorView_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo** items);

    END_INTERFACE
} __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfoVtbl;

interface __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo
{
    CONST_VTBL struct __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* sender,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* sender,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* sender,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager* sender,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* sender,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs_INTERFACE_DEFINED__
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

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CChat_CChatConversationThreadingKind __x_ABI_CWindows_CApplicationModel_CChat_CChatConversationThreadingKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CChat_CChatItemKind __x_ABI_CWindows_CApplicationModel_CChat_CChatItemKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageChangeType __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageChangeType;

typedef enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageKind __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageOperatorKind __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageOperatorKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageTransportKind __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageTransportKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageValidationStatus __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageValidationStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CChat_CChatRestoreHistorySpan __x_ABI_CWindows_CApplicationModel_CChat_CChatRestoreHistorySpan;

typedef enum __x_ABI_CWindows_CApplicationModel_CChat_CChatStoreChangedEventKind __x_ABI_CWindows_CApplicationModel_CChat_CChatStoreChangedEventKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CChat_CChatTransportErrorCodeCategory __x_ABI_CWindows_CApplicationModel_CChat_CChatTransportErrorCodeCategory;

typedef enum __x_ABI_CWindows_CApplicationModel_CChat_CChatTransportInterpretedErrorCode __x_ABI_CWindows_CApplicationModel_CChat_CChatTransportInterpretedErrorCode;

typedef enum __x_ABI_CWindows_CApplicationModel_CChat_CRcsServiceKind __x_ABI_CWindows_CApplicationModel_CChat_CRcsServiceKind;

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatConversationThreadingKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CChat_CChatConversationThreadingKind
{
    ChatConversationThreadingKind_Participants = 0,
    ChatConversationThreadingKind_ContactId = 1,
    ChatConversationThreadingKind_ConversationId = 2,
    ChatConversationThreadingKind_Custom = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatItemKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CChat_CChatItemKind
{
    ChatItemKind_Message = 0,
    ChatItemKind_Conversation = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatMessageChangeType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageChangeType
{
    ChatMessageChangeType_MessageCreated = 0,
    ChatMessageChangeType_MessageModified = 1,
    ChatMessageChangeType_MessageDeleted = 2,
    ChatMessageChangeType_ChangeTrackingLost = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatMessageKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageKind
{
    ChatMessageKind_Standard = 0,
    ChatMessageKind_FileTransferRequest = 1,
    ChatMessageKind_TransportCustom = 2,
    ChatMessageKind_JoinedConversation = 3,
    ChatMessageKind_LeftConversation = 4,
    ChatMessageKind_OtherParticipantJoinedConversation = 5,
    ChatMessageKind_OtherParticipantLeftConversation = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatMessageOperatorKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageOperatorKind
{
    ChatMessageOperatorKind_Unspecified = 0,
    ChatMessageOperatorKind_Sms = 1,
    ChatMessageOperatorKind_Mms = 2,
    ChatMessageOperatorKind_Rcs = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatMessageStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageStatus
{
    ChatMessageStatus_Draft = 0,
    ChatMessageStatus_Sending = 1,
    ChatMessageStatus_Sent = 2,
    ChatMessageStatus_SendRetryNeeded = 3,
    ChatMessageStatus_SendFailed = 4,
    ChatMessageStatus_Received = 5,
    ChatMessageStatus_ReceiveDownloadNeeded = 6,
    ChatMessageStatus_ReceiveDownloadFailed = 7,
    ChatMessageStatus_ReceiveDownloading = 8,
    ChatMessageStatus_Deleted = 9,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ChatMessageStatus_Declined = 10,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ChatMessageStatus_Cancelled = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ChatMessageStatus_Recalled = 12,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    ChatMessageStatus_ReceiveRetryNeeded = 13,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatMessageTransportKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageTransportKind
{
    ChatMessageTransportKind_Text = 0,
    ChatMessageTransportKind_Untriaged = 1,
    ChatMessageTransportKind_Blocked = 2,
    ChatMessageTransportKind_Custom = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatMessageValidationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageValidationStatus
{
    ChatMessageValidationStatus_Valid = 0,
    ChatMessageValidationStatus_NoRecipients = 1,
    ChatMessageValidationStatus_InvalidData = 2,
    ChatMessageValidationStatus_MessageTooLarge = 3,
    ChatMessageValidationStatus_TooManyRecipients = 4,
    ChatMessageValidationStatus_TransportInactive = 5,
    ChatMessageValidationStatus_TransportNotFound = 6,
    ChatMessageValidationStatus_TooManyAttachments = 7,
    ChatMessageValidationStatus_InvalidRecipients = 8,
    ChatMessageValidationStatus_InvalidBody = 9,
    ChatMessageValidationStatus_InvalidOther = 10,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    ChatMessageValidationStatus_ValidWithLargeMessage = 11,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    ChatMessageValidationStatus_VoiceRoamingRestriction = 12,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    ChatMessageValidationStatus_DataRoamingRestriction = 13,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatRestoreHistorySpan
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CApplicationModel_CChat_CChatRestoreHistorySpan
{
    ChatRestoreHistorySpan_LastMonth = 0,
    ChatRestoreHistorySpan_LastYear = 1,
    ChatRestoreHistorySpan_AnyTime = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatStoreChangedEventKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CChat_CChatStoreChangedEventKind
{
    ChatStoreChangedEventKind_NotificationsMissed = 0,
    ChatStoreChangedEventKind_StoreModified = 1,
    ChatStoreChangedEventKind_MessageCreated = 2,
    ChatStoreChangedEventKind_MessageModified = 3,
    ChatStoreChangedEventKind_MessageDeleted = 4,
    ChatStoreChangedEventKind_ConversationModified = 5,
    ChatStoreChangedEventKind_ConversationDeleted = 6,
    ChatStoreChangedEventKind_ConversationTransportDeleted = 7,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatTransportErrorCodeCategory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CChat_CChatTransportErrorCodeCategory
{
    ChatTransportErrorCodeCategory_None = 0,
    ChatTransportErrorCodeCategory_Http = 1,
    ChatTransportErrorCodeCategory_Network = 2,
    ChatTransportErrorCodeCategory_MmsServer = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.ChatTransportInterpretedErrorCode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CChat_CChatTransportInterpretedErrorCode
{
    ChatTransportInterpretedErrorCode_None = 0,
    ChatTransportInterpretedErrorCode_Unknown = 1,
    ChatTransportInterpretedErrorCode_InvalidRecipientAddress = 2,
    ChatTransportInterpretedErrorCode_NetworkConnectivity = 3,
    ChatTransportInterpretedErrorCode_ServiceDenied = 4,
    ChatTransportInterpretedErrorCode_Timeout = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.Chat.RcsServiceKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CChat_CRcsServiceKind
{
    RcsServiceKind_Chat = 0,
    RcsServiceKind_GroupChat = 1,
    RcsServiceKind_FileTransfer = 2,
    RcsServiceKind_Capability = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatCapabilities
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatCapabilities[] = L"Windows.ApplicationModel.Chat.IChatCapabilities";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsOnline)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_IsChatCapable)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_IsFileTransferCapable)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_IsGeoLocationPushCapable)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_IsIntegratedMessagingCapable)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_get_IsOnline(This, result) \
    ((This)->lpVtbl->get_IsOnline(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_get_IsChatCapable(This, result) \
    ((This)->lpVtbl->get_IsChatCapable(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_get_IsFileTransferCapable(This, result) \
    ((This)->lpVtbl->get_IsFileTransferCapable(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_get_IsGeoLocationPushCapable(This, result) \
    ((This)->lpVtbl->get_IsGeoLocationPushCapable(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_get_IsIntegratedMessagingCapable(This, result) \
    ((This)->lpVtbl->get_IsIntegratedMessagingCapable(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilities_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatCapabilitiesManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatCapabilitiesManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatCapabilitiesManagerStatics[] = L"Windows.ApplicationModel.Chat.IChatCapabilitiesManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCachedCapabilitiesAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics* This,
        HSTRING address,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities** result);
    HRESULT (STDMETHODCALLTYPE* GetCapabilitiesFromNetworkAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics* This,
        HSTRING address,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_GetCachedCapabilitiesAsync(This, address, result) \
    ((This)->lpVtbl->GetCachedCapabilitiesAsync(This, address, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_GetCapabilitiesFromNetworkAsync(This, address, result) \
    ((This)->lpVtbl->GetCapabilitiesFromNetworkAsync(This, address, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatCapabilitiesManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatCapabilitiesManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatCapabilitiesManagerStatics2[] = L"Windows.ApplicationModel.Chat.IChatCapabilitiesManagerStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCachedCapabilitiesForTransportAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2* This,
        HSTRING address,
        HSTRING transportId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities** operation);
    HRESULT (STDMETHODCALLTYPE* GetCapabilitiesFromNetworkForTransportAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2* This,
        HSTRING address,
        HSTRING transportId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatCapabilities** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_GetCachedCapabilitiesForTransportAsync(This, address, transportId, operation) \
    ((This)->lpVtbl->GetCachedCapabilitiesForTransportAsync(This, address, transportId, operation))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_GetCapabilitiesFromNetworkForTransportAsync(This, address, transportId, operation) \
    ((This)->lpVtbl->GetCapabilitiesFromNetworkForTransportAsync(This, address, transportId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatCapabilitiesManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatConversation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatConversation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatConversation[] = L"Windows.ApplicationModel.Chat.IChatConversation";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HasUnreadMessages)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Subject)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* put_Subject)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IsConversationMuted)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* put_IsConversationMuted)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_MostRecentMessageId)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Participants)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        __FIVector_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_ThreadingInfo)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo** result);
    HRESULT (STDMETHODCALLTYPE* DeleteAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* GetMessageReader)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader** result);
    HRESULT (STDMETHODCALLTYPE* MarkAllMessagesAsReadAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* MarkMessagesAsReadAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* SaveAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* NotifyLocalParticipantComposing)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        HSTRING transportId,
        HSTRING participantAddress,
        boolean isComposing);
    HRESULT (STDMETHODCALLTYPE* NotifyRemoteParticipantComposing)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        HSTRING transportId,
        HSTRING participantAddress,
        boolean isComposing);
    HRESULT (STDMETHODCALLTYPE* add_RemoteParticipantComposingChanged)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatConversation_Windows__CApplicationModel__CChat__CRemoteParticipantComposingChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_RemoteParticipantComposingChanged)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_get_HasUnreadMessages(This, result) \
    ((This)->lpVtbl->get_HasUnreadMessages(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_get_Id(This, result) \
    ((This)->lpVtbl->get_Id(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_get_Subject(This, result) \
    ((This)->lpVtbl->get_Subject(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_put_Subject(This, value) \
    ((This)->lpVtbl->put_Subject(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_get_IsConversationMuted(This, result) \
    ((This)->lpVtbl->get_IsConversationMuted(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_put_IsConversationMuted(This, value) \
    ((This)->lpVtbl->put_IsConversationMuted(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_get_MostRecentMessageId(This, result) \
    ((This)->lpVtbl->get_MostRecentMessageId(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_get_Participants(This, result) \
    ((This)->lpVtbl->get_Participants(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_get_ThreadingInfo(This, result) \
    ((This)->lpVtbl->get_ThreadingInfo(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_DeleteAsync(This, result) \
    ((This)->lpVtbl->DeleteAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_GetMessageReader(This, result) \
    ((This)->lpVtbl->GetMessageReader(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_MarkAllMessagesAsReadAsync(This, result) \
    ((This)->lpVtbl->MarkAllMessagesAsReadAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_MarkMessagesAsReadAsync(This, value, result) \
    ((This)->lpVtbl->MarkMessagesAsReadAsync(This, value, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_SaveAsync(This, result) \
    ((This)->lpVtbl->SaveAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_NotifyLocalParticipantComposing(This, transportId, participantAddress, isComposing) \
    ((This)->lpVtbl->NotifyLocalParticipantComposing(This, transportId, participantAddress, isComposing))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_NotifyRemoteParticipantComposing(This, transportId, participantAddress, isComposing) \
    ((This)->lpVtbl->NotifyRemoteParticipantComposing(This, transportId, participantAddress, isComposing))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_add_RemoteParticipantComposingChanged(This, handler, token) \
    ((This)->lpVtbl->add_RemoteParticipantComposingChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_remove_RemoteParticipantComposingChanged(This, token) \
    ((This)->lpVtbl->remove_RemoteParticipantComposingChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatConversation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatConversation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatConversation2[] = L"Windows.ApplicationModel.Chat.IChatConversation2";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CanModifyParticipants)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* put_CanModifyParticipants)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_get_CanModifyParticipants(This, result) \
    ((This)->lpVtbl->get_CanModifyParticipants(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_put_CanModifyParticipants(This, value) \
    ((This)->lpVtbl->put_CanModifyParticipants(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatConversationReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatConversationReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatConversationReader[] = L"Windows.ApplicationModel.Chat.IChatConversationReader";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReadBatchAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation** result);
    HRESULT (STDMETHODCALLTYPE* ReadBatchWithCountAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader* This,
        INT32 count,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatConversation** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReaderVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_ReadBatchAsync(This, result) \
    ((This)->lpVtbl->ReadBatchAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_ReadBatchWithCountAsync(This, count, result) \
    ((This)->lpVtbl->ReadBatchWithCountAsync(This, count, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatConversationThreadingInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatConversationThreadingInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatConversationThreadingInfo[] = L"Windows.ApplicationModel.Chat.IChatConversationThreadingInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContactId)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* put_ContactId)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Custom)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* put_Custom)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ConversationId)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* put_ConversationId)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Participants)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* This,
        __FIVector_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatConversationThreadingKind* result);
    HRESULT (STDMETHODCALLTYPE* put_Kind)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatConversationThreadingKind value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_get_ContactId(This, result) \
    ((This)->lpVtbl->get_ContactId(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_put_ContactId(This, value) \
    ((This)->lpVtbl->put_ContactId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_get_Custom(This, result) \
    ((This)->lpVtbl->get_Custom(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_put_Custom(This, value) \
    ((This)->lpVtbl->put_Custom(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_get_ConversationId(This, result) \
    ((This)->lpVtbl->get_ConversationId(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_put_ConversationId(This, value) \
    ((This)->lpVtbl->put_ConversationId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_get_Participants(This, result) \
    ((This)->lpVtbl->get_Participants(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_get_Kind(This, result) \
    ((This)->lpVtbl->get_Kind(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_put_Kind(This, value) \
    ((This)->lpVtbl->put_Kind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatItem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatItem[] = L"Windows.ApplicationModel.Chat.IChatItem";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ItemKind)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatItem* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatItemKind* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatItemVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatItem
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_get_ItemKind(This, result) \
    ((This)->lpVtbl->get_ItemKind(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatItem;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessage[] = L"Windows.ApplicationModel.Chat.IChatMessage";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Attachments)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        __FIVector_1_Windows__CApplicationModel__CChat__CChatMessageAttachment** value);
    HRESULT (STDMETHODCALLTYPE* get_Body)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Body)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_From)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsForwardingDisabled)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsIncoming)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsRead)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_LocalTimestamp)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_NetworkTimestamp)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Recipients)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        __FIVector_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_RecipientSendStatuses)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        __FIMapView_2_HSTRING_Windows__CApplicationModel__CChat__CChatMessageStatus** value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Subject)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TransportFriendlyName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TransportId)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_TransportId)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_get_Attachments(This, value) \
    ((This)->lpVtbl->get_Attachments(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_get_Body(This, value) \
    ((This)->lpVtbl->get_Body(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_put_Body(This, value) \
    ((This)->lpVtbl->put_Body(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_get_From(This, value) \
    ((This)->lpVtbl->get_From(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_get_IsForwardingDisabled(This, value) \
    ((This)->lpVtbl->get_IsForwardingDisabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_get_IsIncoming(This, value) \
    ((This)->lpVtbl->get_IsIncoming(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_get_IsRead(This, value) \
    ((This)->lpVtbl->get_IsRead(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_get_LocalTimestamp(This, value) \
    ((This)->lpVtbl->get_LocalTimestamp(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_get_NetworkTimestamp(This, value) \
    ((This)->lpVtbl->get_NetworkTimestamp(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_get_Recipients(This, value) \
    ((This)->lpVtbl->get_Recipients(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_get_RecipientSendStatuses(This, value) \
    ((This)->lpVtbl->get_RecipientSendStatuses(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_get_Subject(This, value) \
    ((This)->lpVtbl->get_Subject(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_get_TransportFriendlyName(This, value) \
    ((This)->lpVtbl->get_TransportFriendlyName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_get_TransportId(This, value) \
    ((This)->lpVtbl->get_TransportId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_put_TransportId(This, value) \
    ((This)->lpVtbl->put_TransportId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessage2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessage3
 *     Windows.ApplicationModel.Chat.IChatMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessage2[] = L"Windows.ApplicationModel.Chat.IChatMessage2";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EstimatedDownloadSize)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        UINT64* result);
    HRESULT (STDMETHODCALLTYPE* put_EstimatedDownloadSize)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        UINT64 value);
    HRESULT (STDMETHODCALLTYPE* put_From)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_IsAutoReply)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* put_IsAutoReply)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* put_IsForwardingDisabled)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsReplyDisabled)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* put_IsIncoming)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* put_IsRead)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsSeen)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* put_IsSeen)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsSimMessage)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* put_LocalTimestamp)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* get_MessageKind)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageKind* result);
    HRESULT (STDMETHODCALLTYPE* put_MessageKind)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageKind value);
    HRESULT (STDMETHODCALLTYPE* get_MessageOperatorKind)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageOperatorKind* result);
    HRESULT (STDMETHODCALLTYPE* put_MessageOperatorKind)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageOperatorKind value);
    HRESULT (STDMETHODCALLTYPE* put_NetworkTimestamp)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime value);
    HRESULT (STDMETHODCALLTYPE* get_IsReceivedDuringQuietHours)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* put_IsReceivedDuringQuietHours)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* put_RemoteId)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* put_Status)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageStatus value);
    HRESULT (STDMETHODCALLTYPE* put_Subject)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ShouldSuppressNotification)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* put_ShouldSuppressNotification)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_ThreadingInfo)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo** result);
    HRESULT (STDMETHODCALLTYPE* put_ThreadingInfo)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* value);
    HRESULT (STDMETHODCALLTYPE* get_RecipientsDeliveryInfos)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2* This,
        __FIVector_1_Windows__CApplicationModel__CChat__CChatRecipientDeliveryInfo** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_get_EstimatedDownloadSize(This, result) \
    ((This)->lpVtbl->get_EstimatedDownloadSize(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_EstimatedDownloadSize(This, value) \
    ((This)->lpVtbl->put_EstimatedDownloadSize(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_From(This, value) \
    ((This)->lpVtbl->put_From(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_get_IsAutoReply(This, result) \
    ((This)->lpVtbl->get_IsAutoReply(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_IsAutoReply(This, value) \
    ((This)->lpVtbl->put_IsAutoReply(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_IsForwardingDisabled(This, value) \
    ((This)->lpVtbl->put_IsForwardingDisabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_get_IsReplyDisabled(This, result) \
    ((This)->lpVtbl->get_IsReplyDisabled(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_IsIncoming(This, value) \
    ((This)->lpVtbl->put_IsIncoming(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_IsRead(This, value) \
    ((This)->lpVtbl->put_IsRead(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_get_IsSeen(This, result) \
    ((This)->lpVtbl->get_IsSeen(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_IsSeen(This, value) \
    ((This)->lpVtbl->put_IsSeen(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_get_IsSimMessage(This, result) \
    ((This)->lpVtbl->get_IsSimMessage(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_LocalTimestamp(This, value) \
    ((This)->lpVtbl->put_LocalTimestamp(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_get_MessageKind(This, result) \
    ((This)->lpVtbl->get_MessageKind(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_MessageKind(This, value) \
    ((This)->lpVtbl->put_MessageKind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_get_MessageOperatorKind(This, result) \
    ((This)->lpVtbl->get_MessageOperatorKind(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_MessageOperatorKind(This, value) \
    ((This)->lpVtbl->put_MessageOperatorKind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_NetworkTimestamp(This, value) \
    ((This)->lpVtbl->put_NetworkTimestamp(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_get_IsReceivedDuringQuietHours(This, result) \
    ((This)->lpVtbl->get_IsReceivedDuringQuietHours(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_IsReceivedDuringQuietHours(This, value) \
    ((This)->lpVtbl->put_IsReceivedDuringQuietHours(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_RemoteId(This, value) \
    ((This)->lpVtbl->put_RemoteId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_Status(This, value) \
    ((This)->lpVtbl->put_Status(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_Subject(This, value) \
    ((This)->lpVtbl->put_Subject(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_get_ShouldSuppressNotification(This, result) \
    ((This)->lpVtbl->get_ShouldSuppressNotification(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_ShouldSuppressNotification(This, value) \
    ((This)->lpVtbl->put_ShouldSuppressNotification(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_get_ThreadingInfo(This, result) \
    ((This)->lpVtbl->get_ThreadingInfo(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_put_ThreadingInfo(This, value) \
    ((This)->lpVtbl->put_ThreadingInfo(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_get_RecipientsDeliveryInfos(This, result) \
    ((This)->lpVtbl->get_RecipientsDeliveryInfos(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessage3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessage3[] = L"Windows.ApplicationModel.Chat.IChatMessage3";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RemoteId)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_get_RemoteId(This, value) \
    ((This)->lpVtbl->get_RemoteId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessage4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessage
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessage4[] = L"Windows.ApplicationModel.Chat.IChatMessage4";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SyncId)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* put_SyncId)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_get_SyncId(This, result) \
    ((This)->lpVtbl->get_SyncId(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_put_SyncId(This, value) \
    ((This)->lpVtbl->put_SyncId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageAttachment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageAttachment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageAttachment[] = L"Windows.ApplicationModel.Chat.IChatMessageAttachment";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DataStreamReference)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* put_DataStreamReference)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value);
    HRESULT (STDMETHODCALLTYPE* get_GroupId)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_GroupId)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_MimeType)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_MimeType)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Text)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_get_DataStreamReference(This, value) \
    ((This)->lpVtbl->get_DataStreamReference(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_put_DataStreamReference(This, value) \
    ((This)->lpVtbl->put_DataStreamReference(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_get_GroupId(This, value) \
    ((This)->lpVtbl->get_GroupId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_put_GroupId(This, value) \
    ((This)->lpVtbl->put_GroupId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_get_MimeType(This, value) \
    ((This)->lpVtbl->get_MimeType(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_put_MimeType(This, value) \
    ((This)->lpVtbl->put_MimeType(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_put_Text(This, value) \
    ((This)->lpVtbl->put_Text(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageAttachment2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageAttachment
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessageAttachment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageAttachment2[] = L"Windows.ApplicationModel.Chat.IChatMessageAttachment2";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Thumbnail)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** result);
    HRESULT (STDMETHODCALLTYPE* put_Thumbnail)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value);
    HRESULT (STDMETHODCALLTYPE* get_TransferProgress)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2* This,
        DOUBLE* result);
    HRESULT (STDMETHODCALLTYPE* put_TransferProgress)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_OriginalFileName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* put_OriginalFileName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_get_Thumbnail(This, result) \
    ((This)->lpVtbl->get_Thumbnail(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_put_Thumbnail(This, value) \
    ((This)->lpVtbl->put_Thumbnail(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_get_TransferProgress(This, result) \
    ((This)->lpVtbl->get_TransferProgress(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_put_TransferProgress(This, value) \
    ((This)->lpVtbl->put_TransferProgress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_get_OriginalFileName(This, result) \
    ((This)->lpVtbl->get_OriginalFileName(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_put_OriginalFileName(This, value) \
    ((This)->lpVtbl->put_OriginalFileName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageAttachmentFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageAttachment
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageAttachmentFactory[] = L"Windows.ApplicationModel.Chat.IChatMessageAttachmentFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateChatMessageAttachment)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory* This,
        HSTRING mimeType,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* dataStreamReference,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachment** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_CreateChatMessageAttachment(This, mimeType, dataStreamReference, value) \
    ((This)->lpVtbl->CreateChatMessageAttachment(This, mimeType, dataStreamReference, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageAttachmentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageBlockingStatic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageBlocking
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageBlockingStatic[] = L"Windows.ApplicationModel.Chat.IChatMessageBlockingStatic";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStaticVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* MarkMessageAsBlockedAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic* This,
        HSTRING localChatMessageId,
        boolean blocked,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStaticVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStaticVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_MarkMessageAsBlockedAsync(This, localChatMessageId, blocked, value) \
    ((This)->lpVtbl->MarkMessageAsBlockedAsync(This, localChatMessageId, blocked, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageBlockingStatic_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageChange
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageChange[] = L"Windows.ApplicationModel.Chat.IChatMessageChange";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChangeType)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageChangeType* value);
    HRESULT (STDMETHODCALLTYPE* get_Message)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_get_ChangeType(This, value) \
    ((This)->lpVtbl->get_ChangeType(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_get_Message(This, value) \
    ((This)->lpVtbl->get_Message(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageChangeReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageChangeReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageChangeReader[] = L"Windows.ApplicationModel.Chat.IChatMessageChangeReader";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AcceptChanges)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader* This);
    HRESULT (STDMETHODCALLTYPE* AcceptChangesThrough)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChange* lastChangeToAcknowledge);
    HRESULT (STDMETHODCALLTYPE* ReadBatchAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageChange** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReaderVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_AcceptChanges(This) \
    ((This)->lpVtbl->AcceptChanges(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_AcceptChangesThrough(This, lastChangeToAcknowledge) \
    ((This)->lpVtbl->AcceptChangesThrough(This, lastChangeToAcknowledge))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_ReadBatchAsync(This, value) \
    ((This)->lpVtbl->ReadBatchAsync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageChangeTracker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageChangeTracker
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageChangeTracker[] = L"Windows.ApplicationModel.Chat.IChatMessageChangeTracker";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTrackerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Enable)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker* This);
    HRESULT (STDMETHODCALLTYPE* GetChangeReader)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeReader** value);
    HRESULT (STDMETHODCALLTYPE* Reset)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTrackerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTrackerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_Enable(This) \
    ((This)->lpVtbl->Enable(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_GetChangeReader(This, value) \
    ((This)->lpVtbl->GetChangeReader(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_Reset(This) \
    ((This)->lpVtbl->Reset(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageChangedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageChangedDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageChangedDeferral[] = L"Windows.ApplicationModel.Chat.IChatMessageChangedDeferral";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferralVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageChangedEventArgs[] = L"Windows.ApplicationModel.Chat.IChatMessageChangedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageManager2Statics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessageManagerStatic
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageManager2Statics[] = L"Windows.ApplicationModel.Chat.IChatMessageManager2Statics";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2StaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RegisterTransportAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics* This,
        __FIAsyncOperation_1_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* GetTransportAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics* This,
        HSTRING transportId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageTransport** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2StaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2StaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_RegisterTransportAsync(This, result) \
    ((This)->lpVtbl->RegisterTransportAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_GetTransportAsync(This, transportId, result) \
    ((This)->lpVtbl->GetTransportAsync(This, transportId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManager2Statics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageManagerStatic
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageManagerStatic[] = L"Windows.ApplicationModel.Chat.IChatMessageManagerStatic";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStaticVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetTransportsAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessageTransport** value);
    HRESULT (STDMETHODCALLTYPE* RequestStoreAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessageStore** value);
    HRESULT (STDMETHODCALLTYPE* ShowComposeSmsMessageAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* message,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* ShowSmsSettings)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStaticVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStaticVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_GetTransportsAsync(This, value) \
    ((This)->lpVtbl->GetTransportsAsync(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_RequestStoreAsync(This, value) \
    ((This)->lpVtbl->RequestStoreAsync(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_ShowComposeSmsMessageAsync(This, message, value) \
    ((This)->lpVtbl->ShowComposeSmsMessageAsync(This, message, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_ShowSmsSettings(This) \
    ((This)->lpVtbl->ShowSmsSettings(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatic_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageManagerStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessageManagerStatic
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageManagerStatics3[] = L"Windows.ApplicationModel.Chat.IChatMessageManagerStatics3";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestSyncManagerAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatSyncManager** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_RequestSyncManagerAsync(This, result) \
    ((This)->lpVtbl->RequestSyncManagerAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageManagerStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageNotificationTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageNotificationTriggerDetails[] = L"Windows.ApplicationModel.Chat.IChatMessageNotificationTriggerDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChatMessage)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_get_ChatMessage(This, value) \
    ((This)->lpVtbl->get_ChatMessage(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageNotificationTriggerDetails2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessageNotificationTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageNotificationTriggerDetails2[] = L"Windows.ApplicationModel.Chat.IChatMessageNotificationTriggerDetails2";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ShouldDisplayToast)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_ShouldUpdateDetailText)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_ShouldUpdateBadge)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_ShouldUpdateActionCenter)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_get_ShouldDisplayToast(This, result) \
    ((This)->lpVtbl->get_ShouldDisplayToast(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_get_ShouldUpdateDetailText(This, result) \
    ((This)->lpVtbl->get_ShouldUpdateDetailText(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_get_ShouldUpdateBadge(This, result) \
    ((This)->lpVtbl->get_ShouldUpdateBadge(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_get_ShouldUpdateActionCenter(This, result) \
    ((This)->lpVtbl->get_ShouldUpdateActionCenter(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageNotificationTriggerDetails2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageReader[] = L"Windows.ApplicationModel.Chat.IChatMessageReader";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReadBatchAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReaderVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_ReadBatchAsync(This, value) \
    ((This)->lpVtbl->ReadBatchAsync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageReader2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageReader2[] = L"Windows.ApplicationModel.Chat.IChatMessageReader2";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReadBatchWithCountAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2* This,
        INT32 count,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CChatMessage** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_ReadBatchWithCountAsync(This, count, result) \
    ((This)->lpVtbl->ReadBatchWithCountAsync(This, count, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageStore[] = L"Windows.ApplicationModel.Chat.IChatMessageStore";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ChangeTracker)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageChangeTracker** value);
    HRESULT (STDMETHODCALLTYPE* DeleteMessageAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This,
        HSTRING localMessageId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* DownloadMessageAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This,
        HSTRING localChatMessageId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* GetMessageAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This,
        HSTRING localChatMessageId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage** value);
    HRESULT (STDMETHODCALLTYPE* GetMessageReader1)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader** value);
    HRESULT (STDMETHODCALLTYPE* GetMessageReader2)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan recentTimeLimit,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageReader** value);
    HRESULT (STDMETHODCALLTYPE* MarkMessageReadAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This,
        HSTRING localChatMessageId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* RetrySendMessageAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This,
        HSTRING localChatMessageId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* SendMessageAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* chatMessage,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* ValidateMessage)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* chatMessage,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult** value);
    HRESULT (STDMETHODCALLTYPE* add_MessageChanged)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageChangedEventArgs* value,
        EventRegistrationToken* returnValue);
    HRESULT (STDMETHODCALLTYPE* remove_MessageChanged)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore* This,
        EventRegistrationToken value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_get_ChangeTracker(This, value) \
    ((This)->lpVtbl->get_ChangeTracker(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_DeleteMessageAsync(This, localMessageId, value) \
    ((This)->lpVtbl->DeleteMessageAsync(This, localMessageId, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_DownloadMessageAsync(This, localChatMessageId, value) \
    ((This)->lpVtbl->DownloadMessageAsync(This, localChatMessageId, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_GetMessageAsync(This, localChatMessageId, value) \
    ((This)->lpVtbl->GetMessageAsync(This, localChatMessageId, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_GetMessageReader1(This, value) \
    ((This)->lpVtbl->GetMessageReader1(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_GetMessageReader2(This, recentTimeLimit, value) \
    ((This)->lpVtbl->GetMessageReader2(This, recentTimeLimit, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_MarkMessageReadAsync(This, localChatMessageId, value) \
    ((This)->lpVtbl->MarkMessageReadAsync(This, localChatMessageId, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_RetrySendMessageAsync(This, localChatMessageId, value) \
    ((This)->lpVtbl->RetrySendMessageAsync(This, localChatMessageId, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_SendMessageAsync(This, chatMessage, value) \
    ((This)->lpVtbl->SendMessageAsync(This, chatMessage, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_ValidateMessage(This, chatMessage, value) \
    ((This)->lpVtbl->ValidateMessage(This, chatMessage, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_add_MessageChanged(This, value, returnValue) \
    ((This)->lpVtbl->add_MessageChanged(This, value, returnValue))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_remove_MessageChanged(This, value) \
    ((This)->lpVtbl->remove_MessageChanged(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageStore2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageStore
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessageStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageStore2[] = L"Windows.ApplicationModel.Chat.IChatMessageStore2";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ForwardMessageAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        HSTRING localChatMessageId,
        __FIIterable_1_HSTRING* addresses,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage** result);
    HRESULT (STDMETHODCALLTYPE* GetConversationAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        HSTRING conversationId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation** result);
    HRESULT (STDMETHODCALLTYPE* GetConversationForTransportsAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        HSTRING conversationId,
        __FIIterable_1_HSTRING* transportIds,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation** result);
    HRESULT (STDMETHODCALLTYPE* GetConversationFromThreadingInfoAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationThreadingInfo* threadingInfo,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatConversation** result);
    HRESULT (STDMETHODCALLTYPE* GetConversationReader)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader** result);
    HRESULT (STDMETHODCALLTYPE* GetConversationForTransportsReader)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        __FIIterable_1_HSTRING* transportIds,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversationReader** result);
    HRESULT (STDMETHODCALLTYPE* GetMessageByRemoteIdAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        HSTRING transportId,
        HSTRING remoteId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage** result);
    HRESULT (STDMETHODCALLTYPE* GetUnseenCountAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        __FIAsyncOperation_1_int** result);
    HRESULT (STDMETHODCALLTYPE* GetUnseenCountForTransportsReaderAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        __FIIterable_1_HSTRING* transportIds,
        __FIAsyncOperation_1_int** result);
    HRESULT (STDMETHODCALLTYPE* MarkAsSeenAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* MarkAsSeenForTransportsAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        __FIIterable_1_HSTRING* transportIds,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* GetSearchReader)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions* value,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader** result);
    HRESULT (STDMETHODCALLTYPE* SaveMessageAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessage* chatMessage,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* TryCancelDownloadMessageAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        HSTRING localChatMessageId,
        __FIAsyncOperation_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* TryCancelSendMessageAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        HSTRING localChatMessageId,
        __FIAsyncOperation_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* add_StoreChanged)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CChatMessageStore_Windows__CApplicationModel__CChat__CChatMessageStoreChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StoreChanged)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_ForwardMessageAsync(This, localChatMessageId, addresses, result) \
    ((This)->lpVtbl->ForwardMessageAsync(This, localChatMessageId, addresses, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_GetConversationAsync(This, conversationId, result) \
    ((This)->lpVtbl->GetConversationAsync(This, conversationId, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_GetConversationForTransportsAsync(This, conversationId, transportIds, result) \
    ((This)->lpVtbl->GetConversationForTransportsAsync(This, conversationId, transportIds, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_GetConversationFromThreadingInfoAsync(This, threadingInfo, result) \
    ((This)->lpVtbl->GetConversationFromThreadingInfoAsync(This, threadingInfo, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_GetConversationReader(This, result) \
    ((This)->lpVtbl->GetConversationReader(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_GetConversationForTransportsReader(This, transportIds, result) \
    ((This)->lpVtbl->GetConversationForTransportsReader(This, transportIds, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_GetMessageByRemoteIdAsync(This, transportId, remoteId, result) \
    ((This)->lpVtbl->GetMessageByRemoteIdAsync(This, transportId, remoteId, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_GetUnseenCountAsync(This, result) \
    ((This)->lpVtbl->GetUnseenCountAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_GetUnseenCountForTransportsReaderAsync(This, transportIds, result) \
    ((This)->lpVtbl->GetUnseenCountForTransportsReaderAsync(This, transportIds, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_MarkAsSeenAsync(This, result) \
    ((This)->lpVtbl->MarkAsSeenAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_MarkAsSeenForTransportsAsync(This, transportIds, result) \
    ((This)->lpVtbl->MarkAsSeenForTransportsAsync(This, transportIds, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_GetSearchReader(This, value, result) \
    ((This)->lpVtbl->GetSearchReader(This, value, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_SaveMessageAsync(This, chatMessage, result) \
    ((This)->lpVtbl->SaveMessageAsync(This, chatMessage, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_TryCancelDownloadMessageAsync(This, localChatMessageId, result) \
    ((This)->lpVtbl->TryCancelDownloadMessageAsync(This, localChatMessageId, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_TryCancelSendMessageAsync(This, localChatMessageId, result) \
    ((This)->lpVtbl->TryCancelSendMessageAsync(This, localChatMessageId, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_add_StoreChanged(This, handler, token) \
    ((This)->lpVtbl->add_StoreChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_remove_StoreChanged(This, token) \
    ((This)->lpVtbl->remove_StoreChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageStore3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageStore
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessageStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageStore3[] = L"Windows.ApplicationModel.Chat.IChatMessageStore3";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetMessageBySyncIdAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3* This,
        HSTRING syncId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CChatMessage** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_GetMessageBySyncIdAsync(This, syncId, result) \
    ((This)->lpVtbl->GetMessageBySyncIdAsync(This, syncId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStore3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageStoreChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageStoreChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageStoreChangedEventArgs[] = L"Windows.ApplicationModel.Chat.IChatMessageStoreChangedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatStoreChangedEventKind* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_get_Id(This, result) \
    ((This)->lpVtbl->get_Id(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_get_Kind(This, result) \
    ((This)->lpVtbl->get_Kind(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageStoreChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageTransport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageTransport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageTransport[] = L"Windows.ApplicationModel.Chat.IChatMessageTransport";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsAppSetAsNotificationProvider)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsActive)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_TransportFriendlyName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TransportId)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* RequestSetAsNotificationProviderAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_get_IsAppSetAsNotificationProvider(This, value) \
    ((This)->lpVtbl->get_IsAppSetAsNotificationProvider(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_get_IsActive(This, value) \
    ((This)->lpVtbl->get_IsActive(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_get_TransportFriendlyName(This, value) \
    ((This)->lpVtbl->get_TransportFriendlyName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_get_TransportId(This, value) \
    ((This)->lpVtbl->get_TransportId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_RequestSetAsNotificationProviderAsync(This, value) \
    ((This)->lpVtbl->RequestSetAsNotificationProviderAsync(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageTransport2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageTransport
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.Chat.IChatMessageTransport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageTransport2[] = L"Windows.ApplicationModel.Chat.IChatMessageTransport2";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Configuration)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* get_TransportKind)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageTransportKind* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_get_Configuration(This, result) \
    ((This)->lpVtbl->get_Configuration(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_get_TransportKind(This, result) \
    ((This)->lpVtbl->get_TransportKind(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransport2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageTransportConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageTransportConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageTransportConfiguration[] = L"Windows.ApplicationModel.Chat.IChatMessageTransportConfiguration";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxAttachmentCount)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration* This,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_MaxMessageSizeInKilobytes)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration* This,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_MaxRecipientCount)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration* This,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_SupportedVideoFormat)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile** result);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedProperties)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration* This,
        __FIMapView_2_HSTRING_IInspectable** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfigurationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_get_MaxAttachmentCount(This, result) \
    ((This)->lpVtbl->get_MaxAttachmentCount(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_get_MaxMessageSizeInKilobytes(This, result) \
    ((This)->lpVtbl->get_MaxMessageSizeInKilobytes(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_get_MaxRecipientCount(This, result) \
    ((This)->lpVtbl->get_MaxRecipientCount(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_get_SupportedVideoFormat(This, result) \
    ((This)->lpVtbl->get_SupportedVideoFormat(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_get_ExtendedProperties(This, result) \
    ((This)->lpVtbl->get_ExtendedProperties(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageTransportConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatMessageValidationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatMessageValidationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatMessageValidationResult[] = L"Windows.ApplicationModel.Chat.IChatMessageValidationResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxPartCount)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* get_PartCount)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* get_RemainingCharacterCountInPart)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageValidationStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_get_MaxPartCount(This, value) \
    ((This)->lpVtbl->get_MaxPartCount(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_get_PartCount(This, value) \
    ((This)->lpVtbl->get_PartCount(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_get_RemainingCharacterCountInPart(This, value) \
    ((This)->lpVtbl->get_RemainingCharacterCountInPart(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatMessageValidationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatQueryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatQueryOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatQueryOptions[] = L"Windows.ApplicationModel.Chat.IChatQueryOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SearchString)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* put_SearchString)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_get_SearchString(This, result) \
    ((This)->lpVtbl->get_SearchString(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_put_SearchString(This, value) \
    ((This)->lpVtbl->put_SearchString(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatQueryOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatRecipientDeliveryInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatRecipientDeliveryInfo[] = L"Windows.ApplicationModel.Chat.IChatRecipientDeliveryInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TransportAddress)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* put_TransportAddress)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DeliveryTime)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This,
        __FIReference_1_Windows__CFoundation__CDateTime** result);
    HRESULT (STDMETHODCALLTYPE* put_DeliveryTime)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_ReadTime)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This,
        __FIReference_1_Windows__CFoundation__CDateTime** result);
    HRESULT (STDMETHODCALLTYPE* put_ReadTime)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This,
        __FIReference_1_Windows__CFoundation__CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_TransportErrorCodeCategory)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatTransportErrorCodeCategory* result);
    HRESULT (STDMETHODCALLTYPE* get_TransportInterpretedErrorCode)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatTransportInterpretedErrorCode* result);
    HRESULT (STDMETHODCALLTYPE* get_TransportErrorCode)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_IsErrorPermanent)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatMessageStatus* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_get_TransportAddress(This, result) \
    ((This)->lpVtbl->get_TransportAddress(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_put_TransportAddress(This, value) \
    ((This)->lpVtbl->put_TransportAddress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_get_DeliveryTime(This, result) \
    ((This)->lpVtbl->get_DeliveryTime(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_put_DeliveryTime(This, value) \
    ((This)->lpVtbl->put_DeliveryTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_get_ReadTime(This, result) \
    ((This)->lpVtbl->get_ReadTime(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_put_ReadTime(This, value) \
    ((This)->lpVtbl->put_ReadTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_get_TransportErrorCodeCategory(This, result) \
    ((This)->lpVtbl->get_TransportErrorCodeCategory(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_get_TransportInterpretedErrorCode(This, result) \
    ((This)->lpVtbl->get_TransportInterpretedErrorCode(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_get_TransportErrorCode(This, result) \
    ((This)->lpVtbl->get_TransportErrorCode(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_get_IsErrorPermanent(This, result) \
    ((This)->lpVtbl->get_IsErrorPermanent(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_get_Status(This, result) \
    ((This)->lpVtbl->get_Status(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatRecipientDeliveryInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatSearchReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatSearchReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatSearchReader[] = L"Windows.ApplicationModel.Chat.IChatSearchReader";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReadBatchAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem** result);
    HRESULT (STDMETHODCALLTYPE* ReadBatchWithCountAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader* This,
        INT32 count,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CIChatItem** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReaderVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_ReadBatchAsync(This, result) \
    ((This)->lpVtbl->ReadBatchAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_ReadBatchWithCountAsync(This, count, result) \
    ((This)->lpVtbl->ReadBatchWithCountAsync(This, count, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatSearchReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatSyncConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatSyncConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatSyncConfiguration[] = L"Windows.ApplicationModel.Chat.IChatSyncConfiguration";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsSyncEnabled)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* put_IsSyncEnabled)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_RestoreHistorySpan)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatRestoreHistorySpan* result);
    HRESULT (STDMETHODCALLTYPE* put_RestoreHistorySpan)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CChatRestoreHistorySpan value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfigurationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_get_IsSyncEnabled(This, result) \
    ((This)->lpVtbl->get_IsSyncEnabled(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_put_IsSyncEnabled(This, value) \
    ((This)->lpVtbl->put_IsSyncEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_get_RestoreHistorySpan(This, result) \
    ((This)->lpVtbl->get_RestoreHistorySpan(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_put_RestoreHistorySpan(This, value) \
    ((This)->lpVtbl->put_RestoreHistorySpan(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IChatSyncManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.ChatSyncManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IChatSyncManager[] = L"Windows.ApplicationModel.Chat.IChatSyncManager";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Configuration)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* AssociateAccountAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* UnassociateAccountAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* IsAccountAssociated)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIWebAccount* webAccount,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* StartSync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager* This);
    HRESULT (STDMETHODCALLTYPE* SetConfigurationAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncConfiguration* configuration,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManagerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_get_Configuration(This, result) \
    ((This)->lpVtbl->get_Configuration(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_AssociateAccountAsync(This, webAccount, result) \
    ((This)->lpVtbl->AssociateAccountAsync(This, webAccount, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_UnassociateAccountAsync(This, result) \
    ((This)->lpVtbl->UnassociateAccountAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_IsAccountAssociated(This, webAccount, result) \
    ((This)->lpVtbl->IsAccountAssociated(This, webAccount, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_StartSync(This) \
    ((This)->lpVtbl->StartSync(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_SetConfigurationAsync(This, configuration, result) \
    ((This)->lpVtbl->SetConfigurationAsync(This, configuration, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIChatSyncManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsEndUserMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsEndUserMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsEndUserMessage[] = L"Windows.ApplicationModel.Chat.IRcsEndUserMessage";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TransportId)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_IsPinRequired)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_Actions)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage* This,
        __FIVectorView_1_Windows__CApplicationModel__CChat__CRcsEndUserMessageAction** result);
    HRESULT (STDMETHODCALLTYPE* SendResponseAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction* action,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* SendResponseWithPinAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction* action,
        HSTRING pin,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_get_TransportId(This, result) \
    ((This)->lpVtbl->get_TransportId(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_get_Title(This, result) \
    ((This)->lpVtbl->get_Title(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_get_Text(This, result) \
    ((This)->lpVtbl->get_Text(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_get_IsPinRequired(This, result) \
    ((This)->lpVtbl->get_IsPinRequired(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_get_Actions(This, result) \
    ((This)->lpVtbl->get_Actions(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_SendResponseAsync(This, action, result) \
    ((This)->lpVtbl->SendResponseAsync(This, action, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_SendResponseWithPinAsync(This, action, pin, result) \
    ((This)->lpVtbl->SendResponseWithPinAsync(This, action, pin, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsEndUserMessageAction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsEndUserMessageAction
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsEndUserMessageAction[] = L"Windows.ApplicationModel.Chat.IRcsEndUserMessageAction";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageActionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Label)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction* This,
        HSTRING* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageActionVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageActionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_get_Label(This, result) \
    ((This)->lpVtbl->get_Label(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAction_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsEndUserMessageAvailableEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsEndUserMessageAvailableEventArgs[] = L"Windows.ApplicationModel.Chat.IRcsEndUserMessageAvailableEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsMessageAvailable)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_Message)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessage** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_get_IsMessageAvailable(This, result) \
    ((This)->lpVtbl->get_IsMessageAvailable(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_get_Message(This, result) \
    ((This)->lpVtbl->get_Message(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsEndUserMessageAvailableTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsEndUserMessageAvailableTriggerDetails[] = L"Windows.ApplicationModel.Chat.IRcsEndUserMessageAvailableTriggerDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageAvailableTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsEndUserMessageManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsEndUserMessageManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsEndUserMessageManager[] = L"Windows.ApplicationModel.Chat.IRcsEndUserMessageManager";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_MessageAvailableChanged)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsEndUserMessageManager_Windows__CApplicationModel__CChat__CRcsEndUserMessageAvailableEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_MessageAvailableChanged)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManagerVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_add_MessageAvailableChanged(This, handler, token) \
    ((This)->lpVtbl->add_MessageAvailableChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_remove_MessageAvailableChanged(This, token) \
    ((This)->lpVtbl->remove_MessageAvailableChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsManagerStatics[] = L"Windows.ApplicationModel.Chat.IRcsManagerStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetEndUserMessageManager)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsEndUserMessageManager** result);
    HRESULT (STDMETHODCALLTYPE* GetTransportsAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CChat__CRcsTransport** value);
    HRESULT (STDMETHODCALLTYPE* GetTransportAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics* This,
        HSTRING transportId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CChat__CRcsTransport** result);
    HRESULT (STDMETHODCALLTYPE* LeaveConversationAsync)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIChatConversation* conversation,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_GetEndUserMessageManager(This, result) \
    ((This)->lpVtbl->GetEndUserMessageManager(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_GetTransportsAsync(This, value) \
    ((This)->lpVtbl->GetTransportsAsync(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_GetTransportAsync(This, transportId, result) \
    ((This)->lpVtbl->GetTransportAsync(This, transportId, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_LeaveConversationAsync(This, conversation, value) \
    ((This)->lpVtbl->LeaveConversationAsync(This, conversation, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsManagerStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsManagerStatics2[] = L"Windows.ApplicationModel.Chat.IRcsManagerStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_TransportListChanged)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_TransportListChanged)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_add_TransportListChanged(This, handler, token) \
    ((This)->lpVtbl->add_TransportListChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_remove_TransportListChanged(This, token) \
    ((This)->lpVtbl->remove_TransportListChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsManagerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsServiceKindSupportedChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsServiceKindSupportedChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsServiceKindSupportedChangedEventArgs[] = L"Windows.ApplicationModel.Chat.IRcsServiceKindSupportedChangedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ServiceKind)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CRcsServiceKind* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_get_ServiceKind(This, result) \
    ((This)->lpVtbl->get_ServiceKind(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsServiceKindSupportedChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsTransport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsTransport
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsTransport[] = L"Windows.ApplicationModel.Chat.IRcsTransport";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedProperties)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* This,
        __FIMapView_2_HSTRING_IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_IsActive)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_TransportFriendlyName)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TransportId)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Configuration)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* This,
        __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* IsStoreAndForwardEnabled)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CRcsServiceKind serviceKind,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* IsServiceKindSupported)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* This,
        enum __x_ABI_CWindows_CApplicationModel_CChat_CRcsServiceKind serviceKind,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* add_ServiceKindSupportedChanged)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CChat__CRcsTransport_Windows__CApplicationModel__CChat__CRcsServiceKindSupportedChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ServiceKindSupportedChanged)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_get_ExtendedProperties(This, value) \
    ((This)->lpVtbl->get_ExtendedProperties(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_get_IsActive(This, value) \
    ((This)->lpVtbl->get_IsActive(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_get_TransportFriendlyName(This, value) \
    ((This)->lpVtbl->get_TransportFriendlyName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_get_TransportId(This, value) \
    ((This)->lpVtbl->get_TransportId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_get_Configuration(This, result) \
    ((This)->lpVtbl->get_Configuration(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_IsStoreAndForwardEnabled(This, serviceKind, result) \
    ((This)->lpVtbl->IsStoreAndForwardEnabled(This, serviceKind, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_IsServiceKindSupported(This, serviceKind, result) \
    ((This)->lpVtbl->IsServiceKindSupported(This, serviceKind, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_add_ServiceKindSupportedChanged(This, handler, token) \
    ((This)->lpVtbl->add_ServiceKindSupportedChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_remove_ServiceKindSupportedChanged(This, token) \
    ((This)->lpVtbl->remove_ServiceKindSupportedChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransport_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRcsTransportConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RcsTransportConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRcsTransportConfiguration[] = L"Windows.ApplicationModel.Chat.IRcsTransportConfiguration";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxAttachmentCount)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration* This,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_MaxMessageSizeInKilobytes)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration* This,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_MaxGroupMessageSizeInKilobytes)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration* This,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_MaxRecipientCount)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration* This,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_MaxFileSizeInKilobytes)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration* This,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_WarningFileSizeInKilobytes)(__x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration* This,
        INT32* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfigurationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_get_MaxAttachmentCount(This, result) \
    ((This)->lpVtbl->get_MaxAttachmentCount(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_get_MaxMessageSizeInKilobytes(This, result) \
    ((This)->lpVtbl->get_MaxMessageSizeInKilobytes(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_get_MaxGroupMessageSizeInKilobytes(This, result) \
    ((This)->lpVtbl->get_MaxGroupMessageSizeInKilobytes(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_get_MaxRecipientCount(This, result) \
    ((This)->lpVtbl->get_MaxRecipientCount(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_get_MaxFileSizeInKilobytes(This, result) \
    ((This)->lpVtbl->get_MaxFileSizeInKilobytes(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_get_WarningFileSizeInKilobytes(This, result) \
    ((This)->lpVtbl->get_WarningFileSizeInKilobytes(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRcsTransportConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Chat.IRemoteParticipantComposingChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Chat.RemoteParticipantComposingChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Chat_IRemoteParticipantComposingChangedEventArgs[] = L"Windows.ApplicationModel.Chat.IRemoteParticipantComposingChangedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TransportId)(__x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_ParticipantAddress)(__x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_IsComposing)(__x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_get_TransportId(This, result) \
    ((This)->lpVtbl->get_TransportId(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_get_ParticipantAddress(This, result) \
    ((This)->lpVtbl->get_ParticipantAddress(This, result))

#define __x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_get_IsComposing(This, result) \
    ((This)->lpVtbl->get_IsComposing(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CChat_CIRemoteParticipantComposingChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatCapabilities
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatCapabilities ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatCapabilities_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatCapabilities_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatCapabilities[] = L"Windows.ApplicationModel.Chat.ChatCapabilities";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatCapabilitiesManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Chat.IChatCapabilitiesManagerStatics2 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Chat.IChatCapabilitiesManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatCapabilitiesManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatCapabilitiesManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatCapabilitiesManager[] = L"Windows.ApplicationModel.Chat.ChatCapabilitiesManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatConversation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatConversation ** Default Interface **
 *    Windows.ApplicationModel.Chat.IChatConversation2
 *    Windows.ApplicationModel.Chat.IChatItem
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatConversation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatConversation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatConversation[] = L"Windows.ApplicationModel.Chat.ChatConversation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatConversationReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatConversationReader ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatConversationReader_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatConversationReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatConversationReader[] = L"Windows.ApplicationModel.Chat.ChatConversationReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatConversationThreadingInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatConversationThreadingInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatConversationThreadingInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatConversationThreadingInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatConversationThreadingInfo[] = L"Windows.ApplicationModel.Chat.ChatConversationThreadingInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessage ** Default Interface **
 *    Windows.ApplicationModel.Chat.IChatMessage2
 *    Windows.ApplicationModel.Chat.IChatMessage3
 *    Windows.ApplicationModel.Chat.IChatMessage4
 *    Windows.ApplicationModel.Chat.IChatItem
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessage_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessage[] = L"Windows.ApplicationModel.Chat.ChatMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageAttachment
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.Chat.IChatMessageAttachmentFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageAttachment ** Default Interface **
 *    Windows.ApplicationModel.Chat.IChatMessageAttachment2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageAttachment_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageAttachment_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageAttachment[] = L"Windows.ApplicationModel.Chat.ChatMessageAttachment";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageBlocking
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Chat.IChatMessageBlockingStatic interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageBlocking_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageBlocking_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageBlocking[] = L"Windows.ApplicationModel.Chat.ChatMessageBlocking";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageChange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageChange ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChange_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChange_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageChange[] = L"Windows.ApplicationModel.Chat.ChatMessageChange";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageChangeReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageChangeReader ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChangeReader_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChangeReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageChangeReader[] = L"Windows.ApplicationModel.Chat.ChatMessageChangeReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageChangeTracker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageChangeTracker ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChangeTracker_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChangeTracker_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageChangeTracker[] = L"Windows.ApplicationModel.Chat.ChatMessageChangeTracker";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageChangedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageChangedDeferral ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChangedDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChangedDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageChangedDeferral[] = L"Windows.ApplicationModel.Chat.ChatMessageChangedDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageChangedEventArgs[] = L"Windows.ApplicationModel.Chat.ChatMessageChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Chat.IChatMessageManagerStatic interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Chat.IChatMessageManager2Statics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Chat.IChatMessageManagerStatics3 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageManager[] = L"Windows.ApplicationModel.Chat.ChatMessageManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageNotificationTriggerDetails ** Default Interface **
 *    Windows.ApplicationModel.Chat.IChatMessageNotificationTriggerDetails2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageNotificationTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageNotificationTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageNotificationTriggerDetails[] = L"Windows.ApplicationModel.Chat.ChatMessageNotificationTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageReader ** Default Interface **
 *    Windows.ApplicationModel.Chat.IChatMessageReader2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageReader_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageReader[] = L"Windows.ApplicationModel.Chat.ChatMessageReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageStore ** Default Interface **
 *    Windows.ApplicationModel.Chat.IChatMessageStore2
 *    Windows.ApplicationModel.Chat.IChatMessageStore3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageStore_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageStore_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageStore[] = L"Windows.ApplicationModel.Chat.ChatMessageStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageStoreChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageStoreChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageStoreChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageStoreChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageStoreChangedEventArgs[] = L"Windows.ApplicationModel.Chat.ChatMessageStoreChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageTransport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageTransport ** Default Interface **
 *    Windows.ApplicationModel.Chat.IChatMessageTransport2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageTransport_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageTransport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageTransport[] = L"Windows.ApplicationModel.Chat.ChatMessageTransport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageTransportConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageTransportConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageTransportConfiguration_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageTransportConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageTransportConfiguration[] = L"Windows.ApplicationModel.Chat.ChatMessageTransportConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatMessageValidationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatMessageValidationResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageValidationResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatMessageValidationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatMessageValidationResult[] = L"Windows.ApplicationModel.Chat.ChatMessageValidationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatQueryOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatQueryOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatQueryOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatQueryOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatQueryOptions[] = L"Windows.ApplicationModel.Chat.ChatQueryOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatRecipientDeliveryInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatRecipientDeliveryInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatRecipientDeliveryInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatRecipientDeliveryInfo[] = L"Windows.ApplicationModel.Chat.ChatRecipientDeliveryInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatSearchReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatSearchReader ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatSearchReader_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatSearchReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatSearchReader[] = L"Windows.ApplicationModel.Chat.ChatSearchReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatSyncConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatSyncConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatSyncConfiguration_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatSyncConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatSyncConfiguration[] = L"Windows.ApplicationModel.Chat.ChatSyncConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Chat.ChatSyncManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IChatSyncManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatSyncManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_ChatSyncManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_ChatSyncManager[] = L"Windows.ApplicationModel.Chat.ChatSyncManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsEndUserMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRcsEndUserMessage ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessage_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsEndUserMessage[] = L"Windows.ApplicationModel.Chat.RcsEndUserMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsEndUserMessageAction
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRcsEndUserMessageAction ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessageAction_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessageAction_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsEndUserMessageAction[] = L"Windows.ApplicationModel.Chat.RcsEndUserMessageAction";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRcsEndUserMessageAvailableEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessageAvailableEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessageAvailableEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsEndUserMessageAvailableEventArgs[] = L"Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRcsEndUserMessageAvailableTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessageAvailableTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessageAvailableTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsEndUserMessageAvailableTriggerDetails[] = L"Windows.ApplicationModel.Chat.RcsEndUserMessageAvailableTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsEndUserMessageManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRcsEndUserMessageManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessageManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsEndUserMessageManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsEndUserMessageManager[] = L"Windows.ApplicationModel.Chat.RcsEndUserMessageManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Chat.IRcsManagerStatics2 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.Chat.IRcsManagerStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsManager_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsManager[] = L"Windows.ApplicationModel.Chat.RcsManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsServiceKindSupportedChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRcsServiceKindSupportedChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsServiceKindSupportedChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsServiceKindSupportedChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsServiceKindSupportedChangedEventArgs[] = L"Windows.ApplicationModel.Chat.RcsServiceKindSupportedChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsTransport
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRcsTransport ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsTransport_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsTransport_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsTransport[] = L"Windows.ApplicationModel.Chat.RcsTransport";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RcsTransportConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRcsTransportConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsTransportConfiguration_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RcsTransportConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RcsTransportConfiguration[] = L"Windows.ApplicationModel.Chat.RcsTransportConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Chat.RemoteParticipantComposingChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Chat.IRemoteParticipantComposingChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Chat_RemoteParticipantComposingChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Chat_RemoteParticipantComposingChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Chat_RemoteParticipantComposingChangedEventArgs[] = L"Windows.ApplicationModel.Chat.RemoteParticipantComposingChangedEventArgs";
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
#endif // __windows2Eapplicationmodel2Echat_p_h__

#endif // __windows2Eapplicationmodel2Echat_h__
