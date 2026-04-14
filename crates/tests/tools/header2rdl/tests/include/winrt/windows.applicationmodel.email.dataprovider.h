
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
#ifndef __windows2Eapplicationmodel2Eemail2Edataprovider_h__
#define __windows2Eapplicationmodel2Eemail2Edataprovider_h__
#ifndef __windows2Eapplicationmodel2Eemail2Edataprovider_p_h__
#define __windows2Eapplicationmodel2Eemail2Edataprovider_p_h__


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
#include "Windows.ApplicationModel.Email.h"
#include "Windows.Security.Cryptography.Certificates.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailDataProviderConnection;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailDataProviderTriggerDetails;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderTriggerDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxCreateFolderRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxCreateFolderRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxCreateFolderRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxCreateFolderRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxDeleteFolderRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxDeleteFolderRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxDeleteFolderRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxDeleteFolderRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxDownloadAttachmentRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxDownloadAttachmentRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxDownloadAttachmentRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxDownloadAttachmentRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxDownloadMessageRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxDownloadMessageRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxDownloadMessageRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxDownloadMessageRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxEmptyFolderRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxEmptyFolderRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxEmptyFolderRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxEmptyFolderRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxForwardMeetingRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxForwardMeetingRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxForwardMeetingRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxForwardMeetingRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxGetAutoReplySettingsRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxGetAutoReplySettingsRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxGetAutoReplySettingsRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxGetAutoReplySettingsRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxMoveFolderRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxMoveFolderRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxMoveFolderRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxMoveFolderRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxProposeNewTimeForMeetingRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxProposeNewTimeForMeetingRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxProposeNewTimeForMeetingRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxProposeNewTimeForMeetingRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxResolveRecipientsRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxResolveRecipientsRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxResolveRecipientsRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxResolveRecipientsRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxServerSearchReadBatchRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxServerSearchReadBatchRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxServerSearchReadBatchRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxServerSearchReadBatchRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxSetAutoReplySettingsRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxSetAutoReplySettingsRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxSetAutoReplySettingsRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxSetAutoReplySettingsRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxSyncManagerSyncRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxSyncManagerSyncRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxSyncManagerSyncRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxSyncManagerSyncRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxUpdateMeetingResponseRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxUpdateMeetingResponseRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxUpdateMeetingResponseRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxUpdateMeetingResponseRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxValidateCertificatesRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxValidateCertificatesRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    interface IEmailMailboxValidateCertificatesRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxValidateCertificatesRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

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
            namespace Email {
                typedef enum EmailCertificateValidationStatus : int EmailCertificateValidationStatus;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1cfe3d41-16a5-5026-a6fe-2cb0a303a605"))
IIterator<enum ABI::Windows::ApplicationModel::Email::EmailCertificateValidationStatus> : IIterator_impl<enum ABI::Windows::ApplicationModel::Email::EmailCertificateValidationStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Email.EmailCertificateValidationStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::ApplicationModel::Email::EmailCertificateValidationStatus> __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_t;
#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7e326530-7449-51a7-b1bc-c43533a78e06"))
IIterable<enum ABI::Windows::ApplicationModel::Email::EmailCertificateValidationStatus> : IIterable_impl<enum ABI::Windows::ApplicationModel::Email::EmailCertificateValidationStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Email.EmailCertificateValidationStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::ApplicationModel::Email::EmailCertificateValidationStatus> __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_t;
#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                class EmailRecipient;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipient_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipient_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                interface IEmailRecipient;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipient ABI::Windows::ApplicationModel::Email::IEmailRecipient

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipient_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("12238d88-1a2f-5e7a-89b1-8dc140536bac"))
IIterator<ABI::Windows::ApplicationModel::Email::EmailRecipient*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::EmailRecipient*, ABI::Windows::ApplicationModel::Email::IEmailRecipient*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Email.EmailRecipient>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Email::EmailRecipient*> __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_t;
#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5f18cab2-236d-5ec5-bc64-e3e63d29e774"))
IIterable<ABI::Windows::ApplicationModel::Email::EmailRecipient*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::EmailRecipient*, ABI::Windows::ApplicationModel::Email::IEmailRecipient*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Email.EmailRecipient>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Email::EmailRecipient*> __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient_t;
#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                class EmailRecipientResolutionResult;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipientResolutionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipientResolutionResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                interface IEmailRecipientResolutionResult;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipientResolutionResult ABI::Windows::ApplicationModel::Email::IEmailRecipientResolutionResult

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipientResolutionResult_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5c040cd6-9593-5e74-9a5e-7284cd1b7200"))
IIterator<ABI::Windows::ApplicationModel::Email::EmailRecipientResolutionResult*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::EmailRecipientResolutionResult*, ABI::Windows::ApplicationModel::Email::IEmailRecipientResolutionResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Email.EmailRecipientResolutionResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Email::EmailRecipientResolutionResult*> __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_t;
#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cae3c1c4-c689-5787-976f-1a158ffdd16b"))
IIterable<ABI::Windows::ApplicationModel::Email::EmailRecipientResolutionResult*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::EmailRecipientResolutionResult*, ABI::Windows::ApplicationModel::Email::IEmailRecipientResolutionResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Email.EmailRecipientResolutionResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Email::EmailRecipientResolutionResult*> __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_t;
#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    class Certificate;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Cryptography {
                namespace Certificates {
                    interface ICertificate;
                } /* Certificates */
            } /* Cryptography */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate ABI::Windows::Security::Cryptography::Certificates::ICertificate

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#define DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("676fc159-f15c-58bd-91a7-28f7e795c756"))
IIterator<ABI::Windows::Security::Cryptography::Certificates::Certificate*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::Certificate*, ABI::Windows::Security::Cryptography::Certificates::ICertificate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Cryptography.Certificates.Certificate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Security::Cryptography::Certificates::Certificate*> __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t;
#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#define DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0c7d1423-e8fd-5a91-b55c-8bfbe7ac2d40"))
IIterable<ABI::Windows::Security::Cryptography::Certificates::Certificate*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::Certificate*, ABI::Windows::Security::Cryptography::Certificates::ICertificate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Cryptography.Certificates.Certificate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Security::Cryptography::Certificates::Certificate*> __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t;
#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE */

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

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f6d6af60-f11a-5c03-80cc-473407a5aabf"))
IVectorView<ABI::Windows::ApplicationModel::Email::EmailRecipient*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::EmailRecipient*, ABI::Windows::ApplicationModel::Email::IEmailRecipient*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Email.EmailRecipient>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Email::EmailRecipient*> __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_t;
#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("963f7013-77c2-51c5-8038-b5bcef633edb"))
IVectorView<ABI::Windows::Security::Cryptography::Certificates::Certificate*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Cryptography::Certificates::Certificate*, ABI::Windows::Security::Cryptography::Certificates::ICertificate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Cryptography.Certificates.Certificate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Security::Cryptography::Certificates::Certificate*> __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t;
#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailDataProviderConnection;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxCreateFolderRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8c7db52d-496e-5419-bd78-b8b657cf4e66"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxCreateFolderRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxCreateFolderRequestEventArgs*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxCreateFolderRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection, Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxCreateFolderRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxDeleteFolderRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d962a9b6-bbb4-5d82-84b4-8f703bf3086f"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxDeleteFolderRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxDeleteFolderRequestEventArgs*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxDeleteFolderRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection, Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxDeleteFolderRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxDownloadAttachmentRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d2e92019-b997-5cd6-8f88-4dbc6f969f15"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxDownloadAttachmentRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxDownloadAttachmentRequestEventArgs*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxDownloadAttachmentRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection, Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxDownloadAttachmentRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxDownloadMessageRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e1b59b2f-ddd5-5159-ae9a-14a866912095"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxDownloadMessageRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxDownloadMessageRequestEventArgs*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxDownloadMessageRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection, Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxDownloadMessageRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxEmptyFolderRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9a851b84-bcb1-5121-ab61-3efe568f683d"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxEmptyFolderRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxEmptyFolderRequestEventArgs*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxEmptyFolderRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection, Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxEmptyFolderRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxForwardMeetingRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9d6a017f-5a70-5d83-a680-d2806748ca0b"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxForwardMeetingRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxForwardMeetingRequestEventArgs*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxForwardMeetingRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection, Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxForwardMeetingRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxGetAutoReplySettingsRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("587c6f92-a969-57b3-895f-9a06b3650d3a"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxGetAutoReplySettingsRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxGetAutoReplySettingsRequestEventArgs*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxGetAutoReplySettingsRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection, Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxGetAutoReplySettingsRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxMoveFolderRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2c6bf2c8-42f3-523d-80db-170e4fb1567f"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxMoveFolderRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxMoveFolderRequestEventArgs*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxMoveFolderRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection, Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxMoveFolderRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxProposeNewTimeForMeetingRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("309d8bde-1e60-524b-828c-5a3d64a672aa"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxProposeNewTimeForMeetingRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxProposeNewTimeForMeetingRequestEventArgs*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxProposeNewTimeForMeetingRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection, Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxProposeNewTimeForMeetingRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxResolveRecipientsRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ec14e586-e4fb-5fc0-91fc-931ce17a3fc3"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxResolveRecipientsRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxResolveRecipientsRequestEventArgs*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxResolveRecipientsRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection, Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxResolveRecipientsRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxServerSearchReadBatchRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f8bf9067-7d11-56a0-a303-163435c14016"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxServerSearchReadBatchRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxServerSearchReadBatchRequestEventArgs*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxServerSearchReadBatchRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection, Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxServerSearchReadBatchRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxSetAutoReplySettingsRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("aa4f8fb3-05e0-54e6-afac-a28e853e756e"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxSetAutoReplySettingsRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxSetAutoReplySettingsRequestEventArgs*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxSetAutoReplySettingsRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection, Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxSetAutoReplySettingsRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxSyncManagerSyncRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b65fc3ec-9476-51c4-ba70-1505d79826b9"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxSyncManagerSyncRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxSyncManagerSyncRequestEventArgs*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxSyncManagerSyncRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection, Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxSyncManagerSyncRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxUpdateMeetingResponseRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3274fbfd-c10a-5b30-adea-2b4b860b4a0d"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxUpdateMeetingResponseRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxUpdateMeetingResponseRequestEventArgs*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxUpdateMeetingResponseRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection, Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxUpdateMeetingResponseRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxValidateCertificatesRequestEventArgs;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("de2625f7-e16f-512e-a8c6-b7445532bcc6"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxValidateCertificatesRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxValidateCertificatesRequestEventArgs*, ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxValidateCertificatesRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection, Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Email::DataProvider::EmailDataProviderConnection*, ABI::Windows::ApplicationModel::Email::DataProvider::EmailMailboxValidateCertificatesRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                typedef enum EmailBatchStatus : int EmailBatchStatus;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                class EmailFolder;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailFolder_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                interface IEmailFolder;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailFolder ABI::Windows::ApplicationModel::Email::IEmailFolder

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailFolder_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                typedef enum EmailMailboxAutoReplyMessageResponseKind : int EmailMailboxAutoReplyMessageResponseKind;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                class EmailMailboxAutoReplySettings;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailboxAutoReplySettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailboxAutoReplySettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                interface IEmailMailboxAutoReplySettings;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailboxAutoReplySettings ABI::Windows::ApplicationModel::Email::IEmailMailboxAutoReplySettings

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailboxAutoReplySettings_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                typedef enum EmailMailboxCreateFolderStatus : int EmailMailboxCreateFolderStatus;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                typedef enum EmailMailboxDeleteFolderStatus : int EmailMailboxDeleteFolderStatus;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                typedef enum EmailMailboxEmptyFolderStatus : int EmailMailboxEmptyFolderStatus;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                typedef enum EmailMeetingResponseType : int EmailMeetingResponseType;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                class EmailMessage;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                interface IEmailMessage;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMessage ABI::Windows::ApplicationModel::Email::IEmailMessage

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMessage_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                typedef enum EmailMessageBodyKind : int EmailMessageBodyKind;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                class EmailQueryOptions;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailQueryOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailQueryOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                interface IEmailQueryOptions;
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailQueryOptions ABI::Windows::ApplicationModel::Email::IEmailQueryOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailQueryOptions_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxCreateFolderRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxDeleteFolderRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxDownloadAttachmentRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxDownloadMessageRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxEmptyFolderRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxForwardMeetingRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxGetAutoReplySettingsRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxMoveFolderRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxProposeNewTimeForMeetingRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxResolveRecipientsRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxServerSearchReadBatchRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxSetAutoReplySettingsRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxSyncManagerSyncRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxUpdateMeetingResponseRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    class EmailMailboxValidateCertificatesRequest;
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailDataProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailDataProviderConnection[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailDataProviderConnection";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("3b9c9dc7-37b2-4bf0-ae30-7b644a1c96e1")
                    IEmailDataProviderConnection : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_MailboxSyncRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_MailboxSyncRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_DownloadMessageRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_DownloadMessageRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_DownloadAttachmentRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_DownloadAttachmentRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_CreateFolderRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_CreateFolderRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_DeleteFolderRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_DeleteFolderRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_EmptyFolderRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_EmptyFolderRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_MoveFolderRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_MoveFolderRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_UpdateMeetingResponseRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_UpdateMeetingResponseRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ForwardMeetingRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ForwardMeetingRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ProposeNewTimeForMeetingRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ProposeNewTimeForMeetingRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SetAutoReplySettingsRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SetAutoReplySettingsRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_GetAutoReplySettingsRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_GetAutoReplySettingsRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ResolveRecipientsRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ResolveRecipientsRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ValidateCertificatesRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ValidateCertificatesRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ServerSearchReadBatchRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ServerSearchReadBatchRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailDataProviderConnection = __uuidof(IEmailDataProviderConnection);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailDataProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailDataProviderTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailDataProviderTriggerDetails[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailDataProviderTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("8f3e4e50-341e-45f3-bba0-84a005e1319a")
                    IEmailDataProviderTriggerDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Connection(
                            ABI::Windows::ApplicationModel::Email::DataProvider::IEmailDataProviderConnection** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailDataProviderTriggerDetails = __uuidof(IEmailDataProviderTriggerDetails);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxCreateFolderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxCreateFolderRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxCreateFolderRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("184d3775-c921-4c39-a309-e16c9f22b04b")
                    IEmailMailboxCreateFolderRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMailboxId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ParentFolderId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::ApplicationModel::Email::IEmailFolder* folder,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::ApplicationModel::Email::EmailMailboxCreateFolderStatus status,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxCreateFolderRequest = __uuidof(IEmailMailboxCreateFolderRequest);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxCreateFolderRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxCreateFolderRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxCreateFolderRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("03e4c02c-241c-4ea9-a68f-ff20bc5afc85")
                    IEmailMailboxCreateFolderRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxCreateFolderRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxCreateFolderRequestEventArgs = __uuidof(IEmailMailboxCreateFolderRequestEventArgs);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDeleteFolderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxDeleteFolderRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDeleteFolderRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("9469e88a-a931-4779-923d-09a3ea292e29")
                    IEmailMailboxDeleteFolderRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMailboxId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EmailFolderId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::ApplicationModel::Email::EmailMailboxDeleteFolderStatus status,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxDeleteFolderRequest = __uuidof(IEmailMailboxDeleteFolderRequest);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDeleteFolderRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxDeleteFolderRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDeleteFolderRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("b4d32d06-2332-4678-8378-28b579336846")
                    IEmailMailboxDeleteFolderRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxDeleteFolderRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxDeleteFolderRequestEventArgs = __uuidof(IEmailMailboxDeleteFolderRequestEventArgs);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadAttachmentRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxDownloadAttachmentRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadAttachmentRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("0b1dbbb4-750c-48e1-bce4-8d589684ffbc")
                    IEmailMailboxDownloadAttachmentRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMailboxId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMessageId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EmailAttachmentId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxDownloadAttachmentRequest = __uuidof(IEmailMailboxDownloadAttachmentRequest);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadAttachmentRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxDownloadAttachmentRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadAttachmentRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("ccddc46d-ffa8-4877-9f9d-fed7bcaf4104")
                    IEmailMailboxDownloadAttachmentRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxDownloadAttachmentRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxDownloadAttachmentRequestEventArgs = __uuidof(IEmailMailboxDownloadAttachmentRequestEventArgs);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadMessageRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxDownloadMessageRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadMessageRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("497b4187-5b4d-4b23-816c-f3842beb753e")
                    IEmailMailboxDownloadMessageRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMailboxId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMessageId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxDownloadMessageRequest = __uuidof(IEmailMailboxDownloadMessageRequest);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadMessageRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxDownloadMessageRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadMessageRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("470409ad-d0a0-4a5b-bb2a-37621039c53e")
                    IEmailMailboxDownloadMessageRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxDownloadMessageRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxDownloadMessageRequestEventArgs = __uuidof(IEmailMailboxDownloadMessageRequestEventArgs);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxEmptyFolderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxEmptyFolderRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxEmptyFolderRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("fe4b03ab-f86d-46d9-b4ce-bc8a6d9e9268")
                    IEmailMailboxEmptyFolderRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMailboxId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EmailFolderId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::ApplicationModel::Email::EmailMailboxEmptyFolderStatus status,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxEmptyFolderRequest = __uuidof(IEmailMailboxEmptyFolderRequest);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxEmptyFolderRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxEmptyFolderRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxEmptyFolderRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("7183f484-985a-4ac0-b33f-ee0e2627a3c0")
                    IEmailMailboxEmptyFolderRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxEmptyFolderRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxEmptyFolderRequestEventArgs = __uuidof(IEmailMailboxEmptyFolderRequestEventArgs);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxForwardMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxForwardMeetingRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxForwardMeetingRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("616d6af1-70d4-4832-b869-b80542ae9be8")
                    IEmailMailboxForwardMeetingRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMailboxId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMessageId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Recipients(
                            __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Subject(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ForwardHeaderType(
                            ABI::Windows::ApplicationModel::Email::EmailMessageBodyKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ForwardHeader(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Comment(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxForwardMeetingRequest = __uuidof(IEmailMailboxForwardMeetingRequest);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxForwardMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxForwardMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxForwardMeetingRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("2bd8f33a-2974-4759-a5a5-58f44d3c0275")
                    IEmailMailboxForwardMeetingRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxForwardMeetingRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxForwardMeetingRequestEventArgs = __uuidof(IEmailMailboxForwardMeetingRequestEventArgs);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxGetAutoReplySettingsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxGetAutoReplySettingsRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxGetAutoReplySettingsRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("9b380789-1e88-4e01-84cc-1386ad9a2c2f")
                    IEmailMailboxGetAutoReplySettingsRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMailboxId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RequestedFormat(
                            ABI::Windows::ApplicationModel::Email::EmailMailboxAutoReplyMessageResponseKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::ApplicationModel::Email::IEmailMailboxAutoReplySettings* autoReplySettings,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxGetAutoReplySettingsRequest = __uuidof(IEmailMailboxGetAutoReplySettingsRequest);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxGetAutoReplySettingsRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxGetAutoReplySettingsRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxGetAutoReplySettingsRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("d79f55c2-fd45-4004-8a91-9bacf38b7022")
                    IEmailMailboxGetAutoReplySettingsRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxGetAutoReplySettingsRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxGetAutoReplySettingsRequestEventArgs = __uuidof(IEmailMailboxGetAutoReplySettingsRequestEventArgs);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxMoveFolderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxMoveFolderRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxMoveFolderRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("10ba2856-4a95-4068-91cc-67cc7acf454f")
                    IEmailMailboxMoveFolderRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMailboxId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EmailFolderId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NewParentFolderId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NewFolderName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxMoveFolderRequest = __uuidof(IEmailMailboxMoveFolderRequest);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxMoveFolderRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxMoveFolderRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxMoveFolderRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("38623020-14ba-4c88-8698-7239e3c8aaa7")
                    IEmailMailboxMoveFolderRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxMoveFolderRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxMoveFolderRequestEventArgs = __uuidof(IEmailMailboxMoveFolderRequestEventArgs);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxProposeNewTimeForMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxProposeNewTimeForMeetingRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxProposeNewTimeForMeetingRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("5aeff152-9799-4f9f-a399-ff07f3eef04e")
                    IEmailMailboxProposeNewTimeForMeetingRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMailboxId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMessageId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NewStartTime(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NewDuration(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Subject(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Comment(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxProposeNewTimeForMeetingRequest = __uuidof(IEmailMailboxProposeNewTimeForMeetingRequest);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxProposeNewTimeForMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxProposeNewTimeForMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxProposeNewTimeForMeetingRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("fb480b98-33ad-4a67-8251-0f9c249b6a20")
                    IEmailMailboxProposeNewTimeForMeetingRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxProposeNewTimeForMeetingRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxProposeNewTimeForMeetingRequestEventArgs = __uuidof(IEmailMailboxProposeNewTimeForMeetingRequestEventArgs);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxResolveRecipientsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxResolveRecipientsRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxResolveRecipientsRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("efa4cf70-7b39-4c9b-811e-41eaf43a332d")
                    IEmailMailboxResolveRecipientsRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMailboxId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Recipients(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* resolutionResults,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxResolveRecipientsRequest = __uuidof(IEmailMailboxResolveRecipientsRequest);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxResolveRecipientsRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxResolveRecipientsRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxResolveRecipientsRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("260f9e02-b2cf-40f8-8c28-e3ed43b1e89a")
                    IEmailMailboxResolveRecipientsRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxResolveRecipientsRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxResolveRecipientsRequestEventArgs = __uuidof(IEmailMailboxResolveRecipientsRequestEventArgs);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxServerSearchReadBatchRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxServerSearchReadBatchRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxServerSearchReadBatchRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("090eebdf-5a96-41d3-8ad8-34912f9aa60e")
                    IEmailMailboxServerSearchReadBatchRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SessionId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMailboxId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EmailFolderId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Options(
                            ABI::Windows::ApplicationModel::Email::IEmailQueryOptions** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SuggestedBatchSize(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SaveMessageAsync(
                            ABI::Windows::ApplicationModel::Email::IEmailMessage* message,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::ApplicationModel::Email::EmailBatchStatus batchStatus,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxServerSearchReadBatchRequest = __uuidof(IEmailMailboxServerSearchReadBatchRequest);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxServerSearchReadBatchRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxServerSearchReadBatchRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxServerSearchReadBatchRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("14101b4e-ed9e-45d1-ad7a-cc9b7f643ae2")
                    IEmailMailboxServerSearchReadBatchRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxServerSearchReadBatchRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxServerSearchReadBatchRequestEventArgs = __uuidof(IEmailMailboxServerSearchReadBatchRequestEventArgs);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSetAutoReplySettingsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxSetAutoReplySettingsRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSetAutoReplySettingsRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("75a422d0-a88e-4e54-8dc7-c243186b774e")
                    IEmailMailboxSetAutoReplySettingsRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMailboxId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AutoReplySettings(
                            ABI::Windows::ApplicationModel::Email::IEmailMailboxAutoReplySettings** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxSetAutoReplySettingsRequest = __uuidof(IEmailMailboxSetAutoReplySettingsRequest);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSetAutoReplySettingsRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxSetAutoReplySettingsRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSetAutoReplySettingsRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("09da11ad-d7ca-4087-ac86-53fa67f76246")
                    IEmailMailboxSetAutoReplySettingsRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxSetAutoReplySettingsRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxSetAutoReplySettingsRequestEventArgs = __uuidof(IEmailMailboxSetAutoReplySettingsRequestEventArgs);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSyncManagerSyncRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxSyncManagerSyncRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSyncManagerSyncRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("4e10e8e4-7e67-405a-b673-dc60c91090fc")
                    IEmailMailboxSyncManagerSyncRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMailboxId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxSyncManagerSyncRequest = __uuidof(IEmailMailboxSyncManagerSyncRequest);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSyncManagerSyncRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxSyncManagerSyncRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSyncManagerSyncRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("439a031a-8fcc-4ae5-b9b5-d434e0a65aa8")
                    IEmailMailboxSyncManagerSyncRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxSyncManagerSyncRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxSyncManagerSyncRequestEventArgs = __uuidof(IEmailMailboxSyncManagerSyncRequestEventArgs);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxUpdateMeetingResponseRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxUpdateMeetingResponseRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxUpdateMeetingResponseRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("5b99ac93-b2cf-4888-ba4f-306b6b66df30")
                    IEmailMailboxUpdateMeetingResponseRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMailboxId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMessageId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Response(
                            ABI::Windows::ApplicationModel::Email::EmailMeetingResponseType* response
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Subject(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Comment(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SendUpdate(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxUpdateMeetingResponseRequest = __uuidof(IEmailMailboxUpdateMeetingResponseRequest);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxUpdateMeetingResponseRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxUpdateMeetingResponseRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxUpdateMeetingResponseRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("6898d761-56c9-4f17-be31-66fda94ba159")
                    IEmailMailboxUpdateMeetingResponseRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxUpdateMeetingResponseRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxUpdateMeetingResponseRequestEventArgs = __uuidof(IEmailMailboxUpdateMeetingResponseRequestEventArgs);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxValidateCertificatesRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxValidateCertificatesRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxValidateCertificatesRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("a94d3931-e11a-4f97-b81a-187a70a8f41a")
                    IEmailMailboxValidateCertificatesRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_EmailMailboxId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Certificates(
                            __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* validationStatuses,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxValidateCertificatesRequest = __uuidof(IEmailMailboxValidateCertificatesRequest);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxValidateCertificatesRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxValidateCertificatesRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxValidateCertificatesRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Email {
                namespace DataProvider {
                    MIDL_INTERFACE("2583bf17-02ff-49fe-a73c-03f37566c691")
                    IEmailMailboxValidateCertificatesRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Email::DataProvider::IEmailMailboxValidateCertificatesRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IEmailMailboxValidateCertificatesRequestEventArgs = __uuidof(IEmailMailboxValidateCertificatesRequestEventArgs);
                } /* DataProvider */
            } /* Email */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailDataProviderConnection ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailDataProviderConnection_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailDataProviderConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailDataProviderConnection[] = L"Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailDataProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailDataProviderTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailDataProviderTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailDataProviderTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailDataProviderTriggerDetails[] = L"Windows.ApplicationModel.Email.DataProvider.EmailDataProviderTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxCreateFolderRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxCreateFolderRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxCreateFolderRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxCreateFolderRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxCreateFolderRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxCreateFolderRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxCreateFolderRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxCreateFolderRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDeleteFolderRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDeleteFolderRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDeleteFolderRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDeleteFolderRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDeleteFolderRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDeleteFolderRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDeleteFolderRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDeleteFolderRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadAttachmentRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadAttachmentRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadAttachmentRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadAttachmentRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadAttachmentRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadAttachmentRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadAttachmentRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadAttachmentRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadMessageRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadMessageRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadMessageRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadMessageRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadMessageRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadMessageRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadMessageRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadMessageRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxEmptyFolderRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxEmptyFolderRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxEmptyFolderRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxEmptyFolderRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxEmptyFolderRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxEmptyFolderRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxEmptyFolderRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxEmptyFolderRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxForwardMeetingRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxForwardMeetingRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxForwardMeetingRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxForwardMeetingRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxForwardMeetingRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxForwardMeetingRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxForwardMeetingRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxForwardMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxGetAutoReplySettingsRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxGetAutoReplySettingsRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxGetAutoReplySettingsRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxGetAutoReplySettingsRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxGetAutoReplySettingsRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxGetAutoReplySettingsRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxGetAutoReplySettingsRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxGetAutoReplySettingsRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxMoveFolderRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxMoveFolderRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxMoveFolderRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxMoveFolderRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxMoveFolderRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxMoveFolderRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxMoveFolderRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxMoveFolderRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxProposeNewTimeForMeetingRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxProposeNewTimeForMeetingRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxProposeNewTimeForMeetingRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxProposeNewTimeForMeetingRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxProposeNewTimeForMeetingRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxProposeNewTimeForMeetingRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxProposeNewTimeForMeetingRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxProposeNewTimeForMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxResolveRecipientsRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxResolveRecipientsRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxResolveRecipientsRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxResolveRecipientsRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxResolveRecipientsRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxResolveRecipientsRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxResolveRecipientsRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxResolveRecipientsRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxServerSearchReadBatchRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxServerSearchReadBatchRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxServerSearchReadBatchRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxServerSearchReadBatchRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxServerSearchReadBatchRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxServerSearchReadBatchRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxServerSearchReadBatchRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxServerSearchReadBatchRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSetAutoReplySettingsRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSetAutoReplySettingsRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSetAutoReplySettingsRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSetAutoReplySettingsRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSetAutoReplySettingsRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSetAutoReplySettingsRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSetAutoReplySettingsRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSetAutoReplySettingsRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSyncManagerSyncRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSyncManagerSyncRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSyncManagerSyncRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSyncManagerSyncRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSyncManagerSyncRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSyncManagerSyncRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSyncManagerSyncRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSyncManagerSyncRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxUpdateMeetingResponseRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxUpdateMeetingResponseRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxUpdateMeetingResponseRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxUpdateMeetingResponseRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxUpdateMeetingResponseRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxUpdateMeetingResponseRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxUpdateMeetingResponseRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxUpdateMeetingResponseRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxValidateCertificatesRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxValidateCertificatesRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxValidateCertificatesRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxValidateCertificatesRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxValidateCertificatesRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxValidateCertificatesRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxValidateCertificatesRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxValidateCertificatesRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

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

typedef enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailCertificateValidationStatus __x_ABI_CWindows_CApplicationModel_CEmail_CEmailCertificateValidationStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus;

typedef struct __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This,
        enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailCertificateValidationStatus* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailCertificateValidationStatus* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatusVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus;

typedef struct __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* This,
        __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatusVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipient_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipient_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipient __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipient;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipient_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient;

typedef struct __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipient** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipient** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient;

typedef struct __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipient** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipient_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipientResolutionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipientResolutionResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipientResolutionResult __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipientResolutionResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipientResolutionResult_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult;

typedef struct __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipientResolutionResult** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipientResolutionResult** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResultVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult;

typedef struct __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* This,
        __FIIterator_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResultVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

#ifndef ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate;

#endif // ____x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

typedef struct __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl;

interface __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

typedef struct __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __FIIterator_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl;

interface __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__
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
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipientVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipient** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipient* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailRecipient** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipientVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipientVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate;

typedef struct __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CCryptography_CCertificates_CICertificate** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl;

interface __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailBatchStatus __x_ABI_CWindows_CApplicationModel_CEmail_CEmailBatchStatus;

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailFolder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailFolder __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailFolder;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailFolder_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMailboxAutoReplyMessageResponseKind __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMailboxAutoReplyMessageResponseKind;

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailboxAutoReplySettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailboxAutoReplySettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailboxAutoReplySettings __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailboxAutoReplySettings;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailboxAutoReplySettings_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMailboxCreateFolderStatus __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMailboxCreateFolderStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMailboxDeleteFolderStatus __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMailboxDeleteFolderStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMailboxEmptyFolderStatus __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMailboxEmptyFolderStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMeetingResponseType __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMeetingResponseType;

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMessage __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMessage;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMessage_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMessageBodyKind __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMessageBodyKind;

#ifndef ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailQueryOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailQueryOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailQueryOptions __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailQueryOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CEmail_CIEmailQueryOptions_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailDataProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailDataProviderConnection[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailDataProviderConnection";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_MailboxSyncRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSyncManagerSyncRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_MailboxSyncRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DownloadMessageRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadMessageRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DownloadMessageRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DownloadAttachmentRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDownloadAttachmentRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DownloadAttachmentRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_CreateFolderRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxCreateFolderRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CreateFolderRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DeleteFolderRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxDeleteFolderRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DeleteFolderRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_EmptyFolderRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxEmptyFolderRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EmptyFolderRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_MoveFolderRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxMoveFolderRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_MoveFolderRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_UpdateMeetingResponseRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxUpdateMeetingResponseRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_UpdateMeetingResponseRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ForwardMeetingRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxForwardMeetingRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ForwardMeetingRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ProposeNewTimeForMeetingRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxProposeNewTimeForMeetingRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ProposeNewTimeForMeetingRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SetAutoReplySettingsRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxSetAutoReplySettingsRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SetAutoReplySettingsRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_GetAutoReplySettingsRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxGetAutoReplySettingsRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GetAutoReplySettingsRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ResolveRecipientsRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxResolveRecipientsRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ResolveRecipientsRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ValidateCertificatesRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxValidateCertificatesRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ValidateCertificatesRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ServerSearchReadBatchRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CEmail__CDataProvider__CEmailDataProviderConnection_Windows__CApplicationModel__CEmail__CDataProvider__CEmailMailboxServerSearchReadBatchRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ServerSearchReadBatchRequested)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnectionVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_add_MailboxSyncRequested(This, handler, token) \
    ((This)->lpVtbl->add_MailboxSyncRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_remove_MailboxSyncRequested(This, token) \
    ((This)->lpVtbl->remove_MailboxSyncRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_add_DownloadMessageRequested(This, handler, token) \
    ((This)->lpVtbl->add_DownloadMessageRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_remove_DownloadMessageRequested(This, token) \
    ((This)->lpVtbl->remove_DownloadMessageRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_add_DownloadAttachmentRequested(This, handler, token) \
    ((This)->lpVtbl->add_DownloadAttachmentRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_remove_DownloadAttachmentRequested(This, token) \
    ((This)->lpVtbl->remove_DownloadAttachmentRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_add_CreateFolderRequested(This, handler, token) \
    ((This)->lpVtbl->add_CreateFolderRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_remove_CreateFolderRequested(This, token) \
    ((This)->lpVtbl->remove_CreateFolderRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_add_DeleteFolderRequested(This, handler, token) \
    ((This)->lpVtbl->add_DeleteFolderRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_remove_DeleteFolderRequested(This, token) \
    ((This)->lpVtbl->remove_DeleteFolderRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_add_EmptyFolderRequested(This, handler, token) \
    ((This)->lpVtbl->add_EmptyFolderRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_remove_EmptyFolderRequested(This, token) \
    ((This)->lpVtbl->remove_EmptyFolderRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_add_MoveFolderRequested(This, handler, token) \
    ((This)->lpVtbl->add_MoveFolderRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_remove_MoveFolderRequested(This, token) \
    ((This)->lpVtbl->remove_MoveFolderRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_add_UpdateMeetingResponseRequested(This, handler, token) \
    ((This)->lpVtbl->add_UpdateMeetingResponseRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_remove_UpdateMeetingResponseRequested(This, token) \
    ((This)->lpVtbl->remove_UpdateMeetingResponseRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_add_ForwardMeetingRequested(This, handler, token) \
    ((This)->lpVtbl->add_ForwardMeetingRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_remove_ForwardMeetingRequested(This, token) \
    ((This)->lpVtbl->remove_ForwardMeetingRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_add_ProposeNewTimeForMeetingRequested(This, handler, token) \
    ((This)->lpVtbl->add_ProposeNewTimeForMeetingRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_remove_ProposeNewTimeForMeetingRequested(This, token) \
    ((This)->lpVtbl->remove_ProposeNewTimeForMeetingRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_add_SetAutoReplySettingsRequested(This, handler, token) \
    ((This)->lpVtbl->add_SetAutoReplySettingsRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_remove_SetAutoReplySettingsRequested(This, token) \
    ((This)->lpVtbl->remove_SetAutoReplySettingsRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_add_GetAutoReplySettingsRequested(This, handler, token) \
    ((This)->lpVtbl->add_GetAutoReplySettingsRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_remove_GetAutoReplySettingsRequested(This, token) \
    ((This)->lpVtbl->remove_GetAutoReplySettingsRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_add_ResolveRecipientsRequested(This, handler, token) \
    ((This)->lpVtbl->add_ResolveRecipientsRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_remove_ResolveRecipientsRequested(This, token) \
    ((This)->lpVtbl->remove_ResolveRecipientsRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_add_ValidateCertificatesRequested(This, handler, token) \
    ((This)->lpVtbl->add_ValidateCertificatesRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_remove_ValidateCertificatesRequested(This, token) \
    ((This)->lpVtbl->remove_ValidateCertificatesRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_add_ServerSearchReadBatchRequested(This, handler, token) \
    ((This)->lpVtbl->add_ServerSearchReadBatchRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_remove_ServerSearchReadBatchRequested(This, token) \
    ((This)->lpVtbl->remove_ServerSearchReadBatchRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_Start(This) \
    ((This)->lpVtbl->Start(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailDataProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailDataProviderTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailDataProviderTriggerDetails[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailDataProviderTriggerDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Connection)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderConnection** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_get_Connection(This, value) \
    ((This)->lpVtbl->get_Connection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailDataProviderTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxCreateFolderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxCreateFolderRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxCreateFolderRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EmailMailboxId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ParentFolderId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailFolder* folder,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest* This,
        enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMailboxCreateFolderStatus status,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_get_EmailMailboxId(This, value) \
    ((This)->lpVtbl->get_EmailMailboxId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_get_ParentFolderId(This, value) \
    ((This)->lpVtbl->get_ParentFolderId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_ReportCompletedAsync(This, folder, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, folder, result))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_ReportFailedAsync(This, status, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, status, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxCreateFolderRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxCreateFolderRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxCreateFolderRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxCreateFolderRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDeleteFolderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxDeleteFolderRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDeleteFolderRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EmailMailboxId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EmailFolderId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest* This,
        enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMailboxDeleteFolderStatus status,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_get_EmailMailboxId(This, value) \
    ((This)->lpVtbl->get_EmailMailboxId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_get_EmailFolderId(This, value) \
    ((This)->lpVtbl->get_EmailFolderId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_ReportFailedAsync(This, status, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, status, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDeleteFolderRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxDeleteFolderRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDeleteFolderRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDeleteFolderRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadAttachmentRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxDownloadAttachmentRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadAttachmentRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EmailMailboxId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EmailMessageId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EmailAttachmentId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_get_EmailMailboxId(This, value) \
    ((This)->lpVtbl->get_EmailMailboxId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_get_EmailMessageId(This, value) \
    ((This)->lpVtbl->get_EmailMessageId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_get_EmailAttachmentId(This, value) \
    ((This)->lpVtbl->get_EmailAttachmentId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadAttachmentRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxDownloadAttachmentRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadAttachmentRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadAttachmentRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadMessageRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxDownloadMessageRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadMessageRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EmailMailboxId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EmailMessageId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_get_EmailMailboxId(This, value) \
    ((This)->lpVtbl->get_EmailMailboxId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_get_EmailMessageId(This, value) \
    ((This)->lpVtbl->get_EmailMessageId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadMessageRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxDownloadMessageRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadMessageRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxDownloadMessageRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxEmptyFolderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxEmptyFolderRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxEmptyFolderRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EmailMailboxId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EmailFolderId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest* This,
        enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMailboxEmptyFolderStatus status,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_get_EmailMailboxId(This, value) \
    ((This)->lpVtbl->get_EmailMailboxId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_get_EmailFolderId(This, value) \
    ((This)->lpVtbl->get_EmailFolderId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_ReportFailedAsync(This, status, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, status, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxEmptyFolderRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxEmptyFolderRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxEmptyFolderRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxEmptyFolderRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxForwardMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxForwardMeetingRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxForwardMeetingRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EmailMailboxId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EmailMessageId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Recipients)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest* This,
        __FIVectorView_1_Windows__CApplicationModel__CEmail__CEmailRecipient** value);
    HRESULT (STDMETHODCALLTYPE* get_Subject)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ForwardHeaderType)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest* This,
        enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMessageBodyKind* value);
    HRESULT (STDMETHODCALLTYPE* get_ForwardHeader)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Comment)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_get_EmailMailboxId(This, value) \
    ((This)->lpVtbl->get_EmailMailboxId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_get_EmailMessageId(This, value) \
    ((This)->lpVtbl->get_EmailMessageId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_get_Recipients(This, value) \
    ((This)->lpVtbl->get_Recipients(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_get_Subject(This, value) \
    ((This)->lpVtbl->get_Subject(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_get_ForwardHeaderType(This, value) \
    ((This)->lpVtbl->get_ForwardHeaderType(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_get_ForwardHeader(This, value) \
    ((This)->lpVtbl->get_ForwardHeader(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_get_Comment(This, value) \
    ((This)->lpVtbl->get_Comment(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxForwardMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxForwardMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxForwardMeetingRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxForwardMeetingRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxGetAutoReplySettingsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxGetAutoReplySettingsRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxGetAutoReplySettingsRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EmailMailboxId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_RequestedFormat)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest* This,
        enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMailboxAutoReplyMessageResponseKind* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailboxAutoReplySettings* autoReplySettings,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_get_EmailMailboxId(This, value) \
    ((This)->lpVtbl->get_EmailMailboxId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_get_RequestedFormat(This, value) \
    ((This)->lpVtbl->get_RequestedFormat(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_ReportCompletedAsync(This, autoReplySettings, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, autoReplySettings, result))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxGetAutoReplySettingsRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxGetAutoReplySettingsRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxGetAutoReplySettingsRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxGetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxMoveFolderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxMoveFolderRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxMoveFolderRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EmailMailboxId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EmailFolderId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NewParentFolderId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NewFolderName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_get_EmailMailboxId(This, value) \
    ((This)->lpVtbl->get_EmailMailboxId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_get_EmailFolderId(This, value) \
    ((This)->lpVtbl->get_EmailFolderId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_get_NewParentFolderId(This, value) \
    ((This)->lpVtbl->get_NewParentFolderId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_get_NewFolderName(This, value) \
    ((This)->lpVtbl->get_NewFolderName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxMoveFolderRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxMoveFolderRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxMoveFolderRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxMoveFolderRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxProposeNewTimeForMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxProposeNewTimeForMeetingRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxProposeNewTimeForMeetingRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EmailMailboxId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EmailMessageId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NewStartTime)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_NewDuration)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Subject)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Comment)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_get_EmailMailboxId(This, value) \
    ((This)->lpVtbl->get_EmailMailboxId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_get_EmailMessageId(This, value) \
    ((This)->lpVtbl->get_EmailMessageId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_get_NewStartTime(This, value) \
    ((This)->lpVtbl->get_NewStartTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_get_NewDuration(This, value) \
    ((This)->lpVtbl->get_NewDuration(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_get_Subject(This, value) \
    ((This)->lpVtbl->get_Subject(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_get_Comment(This, value) \
    ((This)->lpVtbl->get_Comment(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxProposeNewTimeForMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxProposeNewTimeForMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxProposeNewTimeForMeetingRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxResolveRecipientsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxResolveRecipientsRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxResolveRecipientsRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EmailMailboxId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Recipients)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest* This,
        __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailRecipientResolutionResult* resolutionResults,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_get_EmailMailboxId(This, value) \
    ((This)->lpVtbl->get_EmailMailboxId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_get_Recipients(This, value) \
    ((This)->lpVtbl->get_Recipients(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_ReportCompletedAsync(This, resolutionResults, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, resolutionResults, result))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxResolveRecipientsRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxResolveRecipientsRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxResolveRecipientsRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxResolveRecipientsRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxServerSearchReadBatchRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxServerSearchReadBatchRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxServerSearchReadBatchRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SessionId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EmailMailboxId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EmailFolderId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Options)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailQueryOptions** value);
    HRESULT (STDMETHODCALLTYPE* get_SuggestedBatchSize)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* SaveMessageAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMessage* message,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest* This,
        enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailBatchStatus batchStatus,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_get_SessionId(This, value) \
    ((This)->lpVtbl->get_SessionId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_get_EmailMailboxId(This, value) \
    ((This)->lpVtbl->get_EmailMailboxId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_get_EmailFolderId(This, value) \
    ((This)->lpVtbl->get_EmailFolderId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_get_Options(This, value) \
    ((This)->lpVtbl->get_Options(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_get_SuggestedBatchSize(This, value) \
    ((This)->lpVtbl->get_SuggestedBatchSize(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_SaveMessageAsync(This, message, result) \
    ((This)->lpVtbl->SaveMessageAsync(This, message, result))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_ReportFailedAsync(This, batchStatus, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, batchStatus, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxServerSearchReadBatchRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxServerSearchReadBatchRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxServerSearchReadBatchRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSetAutoReplySettingsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxSetAutoReplySettingsRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSetAutoReplySettingsRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EmailMailboxId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AutoReplySettings)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CIEmailMailboxAutoReplySettings** value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_get_EmailMailboxId(This, value) \
    ((This)->lpVtbl->get_EmailMailboxId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_get_AutoReplySettings(This, value) \
    ((This)->lpVtbl->get_AutoReplySettings(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSetAutoReplySettingsRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxSetAutoReplySettingsRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSetAutoReplySettingsRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSetAutoReplySettingsRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSyncManagerSyncRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxSyncManagerSyncRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSyncManagerSyncRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EmailMailboxId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_get_EmailMailboxId(This, value) \
    ((This)->lpVtbl->get_EmailMailboxId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSyncManagerSyncRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxSyncManagerSyncRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSyncManagerSyncRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxUpdateMeetingResponseRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxUpdateMeetingResponseRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxUpdateMeetingResponseRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EmailMailboxId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EmailMessageId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Response)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest* This,
        enum __x_ABI_CWindows_CApplicationModel_CEmail_CEmailMeetingResponseType* response);
    HRESULT (STDMETHODCALLTYPE* get_Subject)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Comment)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SendUpdate)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_get_EmailMailboxId(This, value) \
    ((This)->lpVtbl->get_EmailMailboxId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_get_EmailMessageId(This, value) \
    ((This)->lpVtbl->get_EmailMessageId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_get_Response(This, response) \
    ((This)->lpVtbl->get_Response(This, response))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_get_Subject(This, value) \
    ((This)->lpVtbl->get_Subject(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_get_Comment(This, value) \
    ((This)->lpVtbl->get_Comment(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_get_SendUpdate(This, value) \
    ((This)->lpVtbl->get_SendUpdate(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxUpdateMeetingResponseRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxUpdateMeetingResponseRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxUpdateMeetingResponseRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxValidateCertificatesRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxValidateCertificatesRequest[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxValidateCertificatesRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EmailMailboxId)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Certificates)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest* This,
        __FIVectorView_1_Windows__CSecurity__CCryptography__CCertificates__CCertificate** value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest* This,
        __FIIterable_1_Windows__CApplicationModel__CEmail__CEmailCertificateValidationStatus* validationStatuses,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_get_EmailMailboxId(This, value) \
    ((This)->lpVtbl->get_EmailMailboxId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_get_Certificates(This, value) \
    ((This)->lpVtbl->get_Certificates(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_ReportCompletedAsync(This, validationStatuses, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, validationStatuses, result))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Email.DataProvider.IEmailMailboxValidateCertificatesRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Email_DataProvider_IEmailMailboxValidateCertificatesRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.IEmailMailboxValidateCertificatesRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CEmail_CDataProvider_CIEmailMailboxValidateCertificatesRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailDataProviderConnection ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailDataProviderConnection_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailDataProviderConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailDataProviderConnection[] = L"Windows.ApplicationModel.Email.DataProvider.EmailDataProviderConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailDataProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailDataProviderTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailDataProviderTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailDataProviderTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailDataProviderTriggerDetails[] = L"Windows.ApplicationModel.Email.DataProvider.EmailDataProviderTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxCreateFolderRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxCreateFolderRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxCreateFolderRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxCreateFolderRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxCreateFolderRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxCreateFolderRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxCreateFolderRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxCreateFolderRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxCreateFolderRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDeleteFolderRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDeleteFolderRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDeleteFolderRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDeleteFolderRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDeleteFolderRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDeleteFolderRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDeleteFolderRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDeleteFolderRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxDeleteFolderRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadAttachmentRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadAttachmentRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadAttachmentRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadAttachmentRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadAttachmentRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadAttachmentRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadAttachmentRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadAttachmentRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadAttachmentRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadMessageRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadMessageRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadMessageRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadMessageRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadMessageRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadMessageRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadMessageRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxDownloadMessageRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxDownloadMessageRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxEmptyFolderRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxEmptyFolderRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxEmptyFolderRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxEmptyFolderRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxEmptyFolderRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxEmptyFolderRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxEmptyFolderRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxEmptyFolderRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxEmptyFolderRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxForwardMeetingRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxForwardMeetingRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxForwardMeetingRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxForwardMeetingRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxForwardMeetingRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxForwardMeetingRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxForwardMeetingRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxForwardMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxForwardMeetingRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxGetAutoReplySettingsRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxGetAutoReplySettingsRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxGetAutoReplySettingsRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxGetAutoReplySettingsRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxGetAutoReplySettingsRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxGetAutoReplySettingsRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxGetAutoReplySettingsRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxGetAutoReplySettingsRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxGetAutoReplySettingsRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxMoveFolderRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxMoveFolderRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxMoveFolderRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxMoveFolderRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxMoveFolderRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxMoveFolderRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxMoveFolderRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxMoveFolderRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxMoveFolderRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxProposeNewTimeForMeetingRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxProposeNewTimeForMeetingRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxProposeNewTimeForMeetingRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxProposeNewTimeForMeetingRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxProposeNewTimeForMeetingRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxProposeNewTimeForMeetingRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxProposeNewTimeForMeetingRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxProposeNewTimeForMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxProposeNewTimeForMeetingRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxResolveRecipientsRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxResolveRecipientsRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxResolveRecipientsRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxResolveRecipientsRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxResolveRecipientsRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxResolveRecipientsRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxResolveRecipientsRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxResolveRecipientsRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxResolveRecipientsRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxServerSearchReadBatchRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxServerSearchReadBatchRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxServerSearchReadBatchRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxServerSearchReadBatchRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxServerSearchReadBatchRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxServerSearchReadBatchRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxServerSearchReadBatchRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxServerSearchReadBatchRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxServerSearchReadBatchRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSetAutoReplySettingsRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSetAutoReplySettingsRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSetAutoReplySettingsRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSetAutoReplySettingsRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSetAutoReplySettingsRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSetAutoReplySettingsRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSetAutoReplySettingsRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSetAutoReplySettingsRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxSetAutoReplySettingsRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSyncManagerSyncRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSyncManagerSyncRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSyncManagerSyncRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSyncManagerSyncRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSyncManagerSyncRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSyncManagerSyncRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSyncManagerSyncRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxSyncManagerSyncRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxSyncManagerSyncRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxUpdateMeetingResponseRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxUpdateMeetingResponseRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxUpdateMeetingResponseRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxUpdateMeetingResponseRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxUpdateMeetingResponseRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxUpdateMeetingResponseRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxUpdateMeetingResponseRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxUpdateMeetingResponseRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxUpdateMeetingResponseRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxValidateCertificatesRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxValidateCertificatesRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxValidateCertificatesRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxValidateCertificatesRequest[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Email.DataProvider.IEmailMailboxValidateCertificatesRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxValidateCertificatesRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Email_DataProvider_EmailMailboxValidateCertificatesRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Email_DataProvider_EmailMailboxValidateCertificatesRequestEventArgs[] = L"Windows.ApplicationModel.Email.DataProvider.EmailMailboxValidateCertificatesRequestEventArgs";
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
#endif // __windows2Eapplicationmodel2Eemail2Edataprovider_p_h__

#endif // __windows2Eapplicationmodel2Eemail2Edataprovider_h__
