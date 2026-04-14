
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
#ifndef __windows2Eapplicationmodel2Ecalls2Ebackground_h__
#define __windows2Eapplicationmodel2Ecalls2Ebackground_h__
#ifndef __windows2Eapplicationmodel2Ecalls2Ebackground_p_h__
#define __windows2Eapplicationmodel2Ecalls2Ebackground_p_h__


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

#if !defined(WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION)

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
#include "Windows.ApplicationModel.Calls.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    interface IPhoneCallBlockedTriggerDetails;
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails ABI::Windows::ApplicationModel::Calls::Background::IPhoneCallBlockedTriggerDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    interface IPhoneCallOriginDataRequestTriggerDetails;
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails ABI::Windows::ApplicationModel::Calls::Background::IPhoneCallOriginDataRequestTriggerDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    interface IPhoneIncomingCallDismissedTriggerDetails;
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails ABI::Windows::ApplicationModel::Calls::Background::IPhoneIncomingCallDismissedTriggerDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    interface IPhoneIncomingCallNotificationTriggerDetails;
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails ABI::Windows::ApplicationModel::Calls::Background::IPhoneIncomingCallNotificationTriggerDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    interface IPhoneLineChangedTriggerDetails;
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails ABI::Windows::ApplicationModel::Calls::Background::IPhoneLineChangedTriggerDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    interface IPhoneNewVoicemailMessageTriggerDetails;
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails ABI::Windows::ApplicationModel::Calls::Background::IPhoneNewVoicemailMessageTriggerDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    typedef enum PhoneCallBlockedReason : int PhoneCallBlockedReason;
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    typedef enum PhoneIncomingCallDismissedReason : int PhoneIncomingCallDismissedReason;
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    typedef enum PhoneLineChangeKind : int PhoneLineChangeKind;
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    typedef enum PhoneLineProperties : unsigned int PhoneLineProperties;
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Calls.Background.PhoneCallBlockedReason
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    enum PhoneCallBlockedReason : int
                    {
                        PhoneCallBlockedReason_InCallBlockingList = 0,
                        PhoneCallBlockedReason_PrivateNumber = 1,
                        PhoneCallBlockedReason_UnknownNumber = 2,
                    };
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Struct Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedReason
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 2.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    enum
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                    DEPRECATED("PhoneIncomingCallDismissedReason is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                    PhoneIncomingCallDismissedReason : int
                    {
                        PhoneIncomingCallDismissedReason_Unknown = 0,
                        PhoneIncomingCallDismissedReason_CallRejected = 1,
                        PhoneIncomingCallDismissedReason_TextReply = 2,
                        PhoneIncomingCallDismissedReason_ConnectionLost = 3,
                    };
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.ApplicationModel.Calls.Background.PhoneLineChangeKind
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    enum PhoneLineChangeKind : int
                    {
                        PhoneLineChangeKind_Added = 0,
                        PhoneLineChangeKind_Removed = 1,
                        PhoneLineChangeKind_PropertiesChanged = 2,
                    };
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Struct Windows.ApplicationModel.Calls.Background.PhoneLineProperties
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    enum PhoneLineProperties : unsigned int
                    {
                        PhoneLineProperties_None = 0,
                        PhoneLineProperties_BrandingOptions = 0x1,
                        PhoneLineProperties_CanDial = 0x2,
                        PhoneLineProperties_CellularDetails = 0x4,
                        PhoneLineProperties_DisplayColor = 0x8,
                        PhoneLineProperties_DisplayName = 0x10,
                        PhoneLineProperties_NetworkName = 0x20,
                        PhoneLineProperties_NetworkState = 0x40,
                        PhoneLineProperties_Transport = 0x80,
                        PhoneLineProperties_Voicemail = 0x100,
                    };

                    DEFINE_ENUM_FLAG_OPERATORS(PhoneLineProperties)
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Struct Windows.ApplicationModel.Calls.Background.PhoneTriggerType
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    enum PhoneTriggerType : int
                    {
                        PhoneTriggerType_NewVoicemailMessage = 0,
                        PhoneTriggerType_CallHistoryChanged = 1,
                        PhoneTriggerType_LineChanged = 2,
                        PhoneTriggerType_AirplaneModeDisabledForEmergencyCall = 3,
                        PhoneTriggerType_CallOriginDataRequest
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        DEPRECATEDENUMERATOR("CallOriginDataRequest is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        = 4,
                        PhoneTriggerType_CallBlocked = 5,
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x20000
                        PhoneTriggerType_IncomingCallDismissed
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        DEPRECATEDENUMERATOR("IncomingCallDismissed is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        = 6,
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x20000
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x30000
                        PhoneTriggerType_IncomingCallNotification = 7,
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x30000
                    };
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Background.IPhoneCallBlockedTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Background_IPhoneCallBlockedTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.IPhoneCallBlockedTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    MIDL_INTERFACE("a4a690a2-e4c1-427f-864e-e470477ddb67")
                    IPhoneCallBlockedTriggerDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PhoneNumber(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LineId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CallBlockedReason(
                            ABI::Windows::ApplicationModel::Calls::Background::PhoneCallBlockedReason* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPhoneCallBlockedTriggerDetails = __uuidof(IPhoneCallBlockedTriggerDetails);
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Background.IPhoneCallOriginDataRequestTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Background_IPhoneCallOriginDataRequestTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.IPhoneCallOriginDataRequestTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    MIDL_INTERFACE("6e9b5b3f-c54b-4e82-4cc9-e329a4184592")
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                    DEPRECATED("PhoneCallOriginDataRequestTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                    IPhoneCallOriginDataRequestTriggerDetails : public IInspectable
                    {
                    public:
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        DEPRECATED("PhoneCallOriginDataRequestTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_RequestId(
                            GUID* result
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        DEPRECATED("PhoneCallOriginDataRequestTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_PhoneNumber(
                            HSTRING* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPhoneCallOriginDataRequestTriggerDetails = __uuidof(IPhoneCallOriginDataRequestTriggerDetails);
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Background.IPhoneIncomingCallDismissedTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Background_IPhoneIncomingCallDismissedTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.IPhoneIncomingCallDismissedTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    MIDL_INTERFACE("bad30276-83b6-5732-9c38-0c206546196a")
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                    DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                    IPhoneIncomingCallDismissedTriggerDetails : public IInspectable
                    {
                    public:
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_LineId(
                            GUID* value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_PhoneNumber(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_DismissalTime(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_TextReplyMessage(
                            HSTRING* value
                            ) = 0;
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
                        virtual HRESULT STDMETHODCALLTYPE get_Reason(
                            ABI::Windows::ApplicationModel::Calls::Background::PhoneIncomingCallDismissedReason* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPhoneIncomingCallDismissedTriggerDetails = __uuidof(IPhoneIncomingCallDismissedTriggerDetails);
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Background.IPhoneIncomingCallNotificationTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Background_IPhoneIncomingCallNotificationTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.IPhoneIncomingCallNotificationTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    MIDL_INTERFACE("2b0e6044-9b32-5d42-8222-d2812e39fb21")
                    IPhoneIncomingCallNotificationTriggerDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_LineId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CallId(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPhoneIncomingCallNotificationTriggerDetails = __uuidof(IPhoneIncomingCallNotificationTriggerDetails);
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Background.IPhoneLineChangedTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Background_IPhoneLineChangedTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.IPhoneLineChangedTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    MIDL_INTERFACE("c6d321e7-d11d-40d8-b2b7-e40a01d66249")
                    IPhoneLineChangedTriggerDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_LineId(
                            GUID* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ChangeType(
                            ABI::Windows::ApplicationModel::Calls::Background::PhoneLineChangeKind* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE HasLinePropertyChanged(
                            ABI::Windows::ApplicationModel::Calls::Background::PhoneLineProperties lineProperty,
                            boolean* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPhoneLineChangedTriggerDetails = __uuidof(IPhoneLineChangedTriggerDetails);
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Background.IPhoneNewVoicemailMessageTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Background_IPhoneNewVoicemailMessageTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.IPhoneNewVoicemailMessageTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Calls {
                namespace Background {
                    MIDL_INTERFACE("13a8c01b-b831-48d3-8ba9-8d22a6580dcf")
                    IPhoneNewVoicemailMessageTriggerDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_LineId(
                            GUID* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VoicemailCount(
                            INT32* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_OperatorMessage(
                            HSTRING* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPhoneNewVoicemailMessageTriggerDetails = __uuidof(IPhoneNewVoicemailMessageTriggerDetails);
                } /* Background */
            } /* Calls */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Class Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.Background.IPhoneCallBlockedTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneCallBlockedTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneCallBlockedTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_Background_PhoneCallBlockedTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Class Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.Background.IPhoneCallOriginDataRequestTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneCallOriginDataRequestTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneCallOriginDataRequestTriggerDetails_DEFINED
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
DEPRECATED("PhoneCallOriginDataRequestTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_Background_PhoneCallOriginDataRequestTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Class Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.Background.IPhoneIncomingCallDismissedTriggerDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneIncomingCallDismissedTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneIncomingCallDismissedTriggerDetails_DEFINED
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_Background_PhoneIncomingCallDismissedTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.Background.IPhoneIncomingCallNotificationTriggerDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneIncomingCallNotificationTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneIncomingCallNotificationTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_Background_PhoneIncomingCallNotificationTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.Background.IPhoneLineChangedTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneLineChangedTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneLineChangedTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_Background_PhoneLineChangedTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Class Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.Background.IPhoneNewVoicemailMessageTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneNewVoicemailMessageTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneNewVoicemailMessageTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_Background_PhoneNewVoicemailMessageTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneCallBlockedReason __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneCallBlockedReason;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneIncomingCallDismissedReason __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneIncomingCallDismissedReason;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneLineChangeKind __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneLineChangeKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneLineProperties __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneLineProperties;

/*
 *
 * Struct Windows.ApplicationModel.Calls.Background.PhoneCallBlockedReason
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneCallBlockedReason
{
    PhoneCallBlockedReason_InCallBlockingList = 0,
    PhoneCallBlockedReason_PrivateNumber = 1,
    PhoneCallBlockedReason_UnknownNumber = 2,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Struct Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedReason
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 2.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x20000
enum
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
DEPRECATED("PhoneIncomingCallDismissedReason is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneIncomingCallDismissedReason
{
    PhoneIncomingCallDismissedReason_Unknown = 0,
    PhoneIncomingCallDismissedReason_CallRejected = 1,
    PhoneIncomingCallDismissedReason_TextReply = 2,
    PhoneIncomingCallDismissedReason_ConnectionLost = 3,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.ApplicationModel.Calls.Background.PhoneLineChangeKind
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneLineChangeKind
{
    PhoneLineChangeKind_Added = 0,
    PhoneLineChangeKind_Removed = 1,
    PhoneLineChangeKind_PropertiesChanged = 2,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Struct Windows.ApplicationModel.Calls.Background.PhoneLineProperties
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneLineProperties
{
    PhoneLineProperties_None = 0,
    PhoneLineProperties_BrandingOptions = 0x1,
    PhoneLineProperties_CanDial = 0x2,
    PhoneLineProperties_CellularDetails = 0x4,
    PhoneLineProperties_DisplayColor = 0x8,
    PhoneLineProperties_DisplayName = 0x10,
    PhoneLineProperties_NetworkName = 0x20,
    PhoneLineProperties_NetworkState = 0x40,
    PhoneLineProperties_Transport = 0x80,
    PhoneLineProperties_Voicemail = 0x100,
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Struct Windows.ApplicationModel.Calls.Background.PhoneTriggerType
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
enum __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneTriggerType
{
    PhoneTriggerType_NewVoicemailMessage = 0,
    PhoneTriggerType_CallHistoryChanged = 1,
    PhoneTriggerType_LineChanged = 2,
    PhoneTriggerType_AirplaneModeDisabledForEmergencyCall = 3,
    PhoneTriggerType_CallOriginDataRequest
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATEDENUMERATOR("CallOriginDataRequest is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    = 4,
    PhoneTriggerType_CallBlocked = 5,
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x20000
    PhoneTriggerType_IncomingCallDismissed
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATEDENUMERATOR("IncomingCallDismissed is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    = 6,
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x20000
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x30000
    PhoneTriggerType_IncomingCallNotification = 7,
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x30000
};
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Background.IPhoneCallBlockedTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Background_IPhoneCallBlockedTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.IPhoneCallBlockedTriggerDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PhoneNumber)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LineId)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_CallBlockedReason)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneCallBlockedReason* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_get_PhoneNumber(This, value) \
    ((This)->lpVtbl->get_PhoneNumber(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_get_LineId(This, value) \
    ((This)->lpVtbl->get_LineId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_get_CallBlockedReason(This, value) \
    ((This)->lpVtbl->get_CallBlockedReason(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallBlockedTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Background.IPhoneCallOriginDataRequestTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Background_IPhoneCallOriginDataRequestTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.IPhoneCallOriginDataRequestTriggerDetails";
typedef struct
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
DEPRECATED("PhoneCallOriginDataRequestTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATED("PhoneCallOriginDataRequestTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_RequestId)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails* This,
        GUID* result);
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATED("PhoneCallOriginDataRequestTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_PhoneNumber)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails* This,
        HSTRING* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATED("PhoneCallOriginDataRequestTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_get_RequestId(This, result) \
    ((This)->lpVtbl->get_RequestId(This, result))

#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATED("PhoneCallOriginDataRequestTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_get_PhoneNumber(This, result) \
    ((This)->lpVtbl->get_PhoneNumber(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneCallOriginDataRequestTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Background.IPhoneIncomingCallDismissedTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Background_IPhoneIncomingCallDismissedTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.IPhoneIncomingCallDismissedTriggerDetails";
typedef struct
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails* This,
        TrustLevel* trustLevel);
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_LineId)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails* This,
        GUID* value);
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_PhoneNumber)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_DismissalTime)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_TextReplyMessage)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails* This,
        HSTRING* value);
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    HRESULT (STDMETHODCALLTYPE* get_Reason)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneIncomingCallDismissedReason* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_get_LineId(This, value) \
    ((This)->lpVtbl->get_LineId(This, value))

#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_get_PhoneNumber(This, value) \
    ((This)->lpVtbl->get_PhoneNumber(This, value))

#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_get_DismissalTime(This, value) \
    ((This)->lpVtbl->get_DismissalTime(This, value))

#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_get_TextReplyMessage(This, value) \
    ((This)->lpVtbl->get_TextReplyMessage(This, value))

#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
    DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_get_Reason(This, value) \
    ((This)->lpVtbl->get_Reason(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallDismissedTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Background.IPhoneIncomingCallNotificationTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Background_IPhoneIncomingCallNotificationTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.IPhoneIncomingCallNotificationTriggerDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LineId)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_CallId)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_get_LineId(This, value) \
    ((This)->lpVtbl->get_LineId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_get_CallId(This, value) \
    ((This)->lpVtbl->get_CallId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneIncomingCallNotificationTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Background.IPhoneLineChangedTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Background_IPhoneLineChangedTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.IPhoneLineChangedTriggerDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LineId)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails* This,
        GUID* result);
    HRESULT (STDMETHODCALLTYPE* get_ChangeType)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneLineChangeKind* result);
    HRESULT (STDMETHODCALLTYPE* HasLinePropertyChanged)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails* This,
        enum __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CPhoneLineProperties lineProperty,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_get_LineId(This, result) \
    ((This)->lpVtbl->get_LineId(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_get_ChangeType(This, result) \
    ((This)->lpVtbl->get_ChangeType(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_HasLinePropertyChanged(This, lineProperty, result) \
    ((This)->lpVtbl->HasLinePropertyChanged(This, lineProperty, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneLineChangedTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Calls.Background.IPhoneNewVoicemailMessageTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Calls_Background_IPhoneNewVoicemailMessageTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.IPhoneNewVoicemailMessageTriggerDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LineId)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails* This,
        GUID* result);
    HRESULT (STDMETHODCALLTYPE* get_VoicemailCount)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails* This,
        INT32* result);
    HRESULT (STDMETHODCALLTYPE* get_OperatorMessage)(__x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails* This,
        HSTRING* result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_get_LineId(This, result) \
    ((This)->lpVtbl->get_LineId(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_get_VoicemailCount(This, result) \
    ((This)->lpVtbl->get_VoicemailCount(This, result))

#define __x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_get_OperatorMessage(This, result) \
    ((This)->lpVtbl->get_OperatorMessage(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CCalls_CBackground_CIPhoneNewVoicemailMessageTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Class Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.Background.IPhoneCallBlockedTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneCallBlockedTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneCallBlockedTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_Background_PhoneCallBlockedTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.PhoneCallBlockedTriggerDetails";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Class Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.Background.IPhoneCallOriginDataRequestTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneCallOriginDataRequestTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneCallOriginDataRequestTriggerDetails_DEFINED
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
DEPRECATED("PhoneCallOriginDataRequestTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_Background_PhoneCallOriginDataRequestTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.PhoneCallOriginDataRequestTriggerDetails";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Class Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.Background.IPhoneIncomingCallDismissedTriggerDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneIncomingCallDismissedTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneIncomingCallDismissedTriggerDetails_DEFINED
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
DEPRECATED("PhoneIncomingCallDismissedTriggerDetails is deprecated and might not work for all platforms. For more info, see MSDN.")
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x40000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_Background_PhoneIncomingCallDismissedTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.PhoneIncomingCallDismissedTriggerDetails";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.Background.IPhoneIncomingCallNotificationTriggerDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneIncomingCallNotificationTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneIncomingCallNotificationTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_Background_PhoneIncomingCallNotificationTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.PhoneIncomingCallNotificationTriggerDetails";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.Background.IPhoneLineChangedTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneLineChangedTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneLineChangedTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_Background_PhoneLineChangedTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.PhoneLineChangedTriggerDetails";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

/*
 *
 * Class Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails
 *
 * Introduced to Windows.ApplicationModel.Calls.Background.CallsBackgroundContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Calls.Background.IPhoneNewVoicemailMessageTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
    WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneNewVoicemailMessageTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Calls_Background_PhoneNewVoicemailMessageTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Calls_Background_PhoneNewVoicemailMessageTriggerDetails[] = L"Windows.ApplicationModel.Calls.Background.PhoneNewVoicemailMessageTriggerDetails";
#endif
#endif // WINDOWS_APPLICATIONMODEL_CALLS_BACKGROUND_CALLSBACKGROUNDCONTRACT_VERSION >= 0x10000 || \
       // WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION >= 0x10000 && WINDOWS_APPLICATIONMODEL_CALLS_CALLSPHONECONTRACT_VERSION < 0x30000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Eapplicationmodel2Ecalls2Ebackground_p_h__

#endif // __windows2Eapplicationmodel2Ecalls2Ebackground_h__
