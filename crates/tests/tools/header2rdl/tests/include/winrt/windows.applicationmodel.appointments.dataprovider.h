
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
#ifndef __windows2Eapplicationmodel2Eappointments2Edataprovider_h__
#define __windows2Eapplicationmodel2Eappointments2Edataprovider_h__
#ifndef __windows2Eapplicationmodel2Eappointments2Edataprovider_p_h__
#define __windows2Eapplicationmodel2Eappointments2Edataprovider_p_h__


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
#include "Windows.ApplicationModel.Appointments.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    interface IAppointmentCalendarCancelMeetingRequest;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarCancelMeetingRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    interface IAppointmentCalendarCancelMeetingRequestEventArgs;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarCancelMeetingRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    interface IAppointmentCalendarCreateOrUpdateAppointmentRequest;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarCreateOrUpdateAppointmentRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    interface IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    interface IAppointmentCalendarForwardMeetingRequest;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarForwardMeetingRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    interface IAppointmentCalendarForwardMeetingRequestEventArgs;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarForwardMeetingRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    interface IAppointmentCalendarProposeNewTimeForMeetingRequest;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarProposeNewTimeForMeetingRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    interface IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    interface IAppointmentCalendarSyncManagerSyncRequest;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarSyncManagerSyncRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    interface IAppointmentCalendarSyncManagerSyncRequestEventArgs;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarSyncManagerSyncRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    interface IAppointmentCalendarUpdateMeetingResponseRequest;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarUpdateMeetingResponseRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    interface IAppointmentCalendarUpdateMeetingResponseRequestEventArgs;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarUpdateMeetingResponseRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    interface IAppointmentDataProviderConnection;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentDataProviderConnection

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    interface IAppointmentDataProviderTriggerDetails;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentDataProviderTriggerDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_FWD_DEFINED__

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
            namespace Appointments {
                class AppointmentInvitee;
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

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
            namespace Appointments {
                namespace DataProvider {
                    class AppointmentDataProviderConnection;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    class AppointmentCalendarCancelMeetingRequestEventArgs;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0e9cc1d8-0d26-5207-b7d9-9ad1bf66e1e6"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarCancelMeetingRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarCancelMeetingRequestEventArgs*, ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarCancelMeetingRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection, Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarCancelMeetingRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    class AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b45150b9-df09-5c86-b57d-3e6deff24767"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs*, ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection, Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    class AppointmentCalendarForwardMeetingRequestEventArgs;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8961d738-ecdc-53c8-b0ec-e729d8109459"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarForwardMeetingRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarForwardMeetingRequestEventArgs*, ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarForwardMeetingRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection, Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarForwardMeetingRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    class AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4fe460a4-e875-5836-9eec-273d52c86ab3"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs*, ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection, Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    class AppointmentCalendarSyncManagerSyncRequestEventArgs;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("37f52677-5f3a-57e6-82f5-cbab2e4dbe8e"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarSyncManagerSyncRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarSyncManagerSyncRequestEventArgs*, ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarSyncManagerSyncRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection, Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarSyncManagerSyncRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    class AppointmentCalendarUpdateMeetingResponseRequestEventArgs;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5370df2b-62cd-5133-93e6-fc80a502af64"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarUpdateMeetingResponseRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarUpdateMeetingResponseRequestEventArgs*, ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarUpdateMeetingResponseRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection, Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentDataProviderConnection*, ABI::Windows::ApplicationModel::Appointments::DataProvider::AppointmentCalendarUpdateMeetingResponseRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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
            namespace Appointments {
                typedef enum AppointmentParticipantResponse : int AppointmentParticipantResponse;
            } /* Appointments */
        } /* ApplicationModel */
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
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    class AppointmentCalendarCancelMeetingRequest;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    class AppointmentCalendarCreateOrUpdateAppointmentRequest;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    class AppointmentCalendarForwardMeetingRequest;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    class AppointmentCalendarProposeNewTimeForMeetingRequest;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    class AppointmentCalendarSyncManagerSyncRequest;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    class AppointmentCalendarUpdateMeetingResponseRequest;
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCancelMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarCancelMeetingRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCancelMeetingRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    MIDL_INTERFACE("49460f8d-6434-40d7-ad46-6297419314d1")
                    IAppointmentCalendarCancelMeetingRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AppointmentCalendarLocalId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AppointmentLocalId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AppointmentOriginalStartTime(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Subject(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Comment(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NotifyInvitees(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppointmentCalendarCancelMeetingRequest = __uuidof(IAppointmentCalendarCancelMeetingRequest);
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCancelMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarCancelMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCancelMeetingRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    MIDL_INTERFACE("1a79be16-7f30-4e35-beef-9d2c7b6dcae1")
                    IAppointmentCalendarCancelMeetingRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarCancelMeetingRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppointmentCalendarCancelMeetingRequestEventArgs = __uuidof(IAppointmentCalendarCancelMeetingRequestEventArgs);
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCreateOrUpdateAppointmentRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarCreateOrUpdateAppointmentRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCreateOrUpdateAppointmentRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    MIDL_INTERFACE("2e62f2b2-ca96-48ac-9124-406b19fefa70")
                    IAppointmentCalendarCreateOrUpdateAppointmentRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AppointmentCalendarLocalId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Appointment(
                            ABI::Windows::ApplicationModel::Appointments::IAppointment** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NotifyInvitees(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ChangedProperties(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::ApplicationModel::Appointments::IAppointment* createdOrUpdatedAppointment,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppointmentCalendarCreateOrUpdateAppointmentRequest = __uuidof(IAppointmentCalendarCreateOrUpdateAppointmentRequest);
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    MIDL_INTERFACE("cf8ded28-002e-4bf7-8e9d-5e20d49aa3ba")
                    IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarCreateOrUpdateAppointmentRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs = __uuidof(IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs);
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarForwardMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarForwardMeetingRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarForwardMeetingRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    MIDL_INTERFACE("82e5ee56-26b6-4253-8a8f-6cf5f2ff7884")
                    IAppointmentCalendarForwardMeetingRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AppointmentCalendarLocalId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AppointmentLocalId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AppointmentOriginalStartTime(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Invitees(
                            __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Subject(
                            HSTRING* value
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

                    MIDL_CONST_ID IID& IID_IAppointmentCalendarForwardMeetingRequest = __uuidof(IAppointmentCalendarForwardMeetingRequest);
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarForwardMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarForwardMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarForwardMeetingRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    MIDL_INTERFACE("3109151a-23a2-42fd-9c82-c9a60d59f8a8")
                    IAppointmentCalendarForwardMeetingRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarForwardMeetingRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppointmentCalendarForwardMeetingRequestEventArgs = __uuidof(IAppointmentCalendarForwardMeetingRequestEventArgs);
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarProposeNewTimeForMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarProposeNewTimeForMeetingRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarProposeNewTimeForMeetingRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    MIDL_INTERFACE("ce1c63f5-edf6-43c3-82b7-be6b368c6900")
                    IAppointmentCalendarProposeNewTimeForMeetingRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AppointmentCalendarLocalId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AppointmentLocalId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AppointmentOriginalStartTime(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
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

                    MIDL_CONST_ID IID& IID_IAppointmentCalendarProposeNewTimeForMeetingRequest = __uuidof(IAppointmentCalendarProposeNewTimeForMeetingRequest);
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    MIDL_INTERFACE("d2d777d8-fed1-4280-a3ba-2e1f47609aa2")
                    IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarProposeNewTimeForMeetingRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs = __uuidof(IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs);
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarSyncManagerSyncRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarSyncManagerSyncRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarSyncManagerSyncRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    MIDL_INTERFACE("12ab382b-7163-4a56-9a4e-7223a84adf46")
                    IAppointmentCalendarSyncManagerSyncRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AppointmentCalendarLocalId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppointmentCalendarSyncManagerSyncRequest = __uuidof(IAppointmentCalendarSyncManagerSyncRequest);
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarSyncManagerSyncRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarSyncManagerSyncRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarSyncManagerSyncRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    MIDL_INTERFACE("ca17c6f7-0284-4edd-87ba-4d8f69dcf5c0")
                    IAppointmentCalendarSyncManagerSyncRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarSyncManagerSyncRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppointmentCalendarSyncManagerSyncRequestEventArgs = __uuidof(IAppointmentCalendarSyncManagerSyncRequestEventArgs);
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarUpdateMeetingResponseRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarUpdateMeetingResponseRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarUpdateMeetingResponseRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    MIDL_INTERFACE("a36d608c-c29d-4b94-b086-7e9ff7bd84a0")
                    IAppointmentCalendarUpdateMeetingResponseRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AppointmentCalendarLocalId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AppointmentLocalId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AppointmentOriginalStartTime(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Response(
                            ABI::Windows::ApplicationModel::Appointments::AppointmentParticipantResponse* response
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

                    MIDL_CONST_ID IID& IID_IAppointmentCalendarUpdateMeetingResponseRequest = __uuidof(IAppointmentCalendarUpdateMeetingResponseRequest);
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarUpdateMeetingResponseRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarUpdateMeetingResponseRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarUpdateMeetingResponseRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    MIDL_INTERFACE("88759883-97bf-479d-aed5-0be8ce567d1e")
                    IAppointmentCalendarUpdateMeetingResponseRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentCalendarUpdateMeetingResponseRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppointmentCalendarUpdateMeetingResponseRequestEventArgs = __uuidof(IAppointmentCalendarUpdateMeetingResponseRequestEventArgs);
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentDataProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentDataProviderConnection[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentDataProviderConnection";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    MIDL_INTERFACE("f3dd9d83-3254-465f-abdb-928046552cf4")
                    IAppointmentDataProviderConnection : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_SyncRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SyncRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_CreateOrUpdateAppointmentRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_CreateOrUpdateAppointmentRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_CancelMeetingRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_CancelMeetingRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ForwardMeetingRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ForwardMeetingRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ProposeNewTimeForMeetingRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ProposeNewTimeForMeetingRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_UpdateMeetingResponseRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_UpdateMeetingResponseRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppointmentDataProviderConnection = __uuidof(IAppointmentDataProviderConnection);
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentDataProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentDataProviderTriggerDetails[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentDataProviderTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Appointments {
                namespace DataProvider {
                    MIDL_INTERFACE("b3283c01-7e12-4e5e-b1ef-74fb68ac6f2a")
                    IAppointmentDataProviderTriggerDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Connection(
                            ABI::Windows::ApplicationModel::Appointments::DataProvider::IAppointmentDataProviderConnection** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAppointmentDataProviderTriggerDetails = __uuidof(IAppointmentDataProviderTriggerDetails);
                } /* DataProvider */
            } /* Appointments */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCancelMeetingRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCancelMeetingRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCancelMeetingRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCancelMeetingRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCancelMeetingRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCancelMeetingRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCancelMeetingRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCancelMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCreateOrUpdateAppointmentRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCreateOrUpdateAppointmentRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCreateOrUpdateAppointmentRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCreateOrUpdateAppointmentRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarForwardMeetingRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarForwardMeetingRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarForwardMeetingRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarForwardMeetingRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarForwardMeetingRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarForwardMeetingRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarForwardMeetingRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarForwardMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarProposeNewTimeForMeetingRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarProposeNewTimeForMeetingRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarProposeNewTimeForMeetingRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarProposeNewTimeForMeetingRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarSyncManagerSyncRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarSyncManagerSyncRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarSyncManagerSyncRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarSyncManagerSyncRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarSyncManagerSyncRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarSyncManagerSyncRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarSyncManagerSyncRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarSyncManagerSyncRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarUpdateMeetingResponseRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarUpdateMeetingResponseRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarUpdateMeetingResponseRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarUpdateMeetingResponseRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarUpdateMeetingResponseRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarUpdateMeetingResponseRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarUpdateMeetingResponseRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarUpdateMeetingResponseRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentDataProviderConnection ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentDataProviderConnection_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentDataProviderConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentDataProviderConnection[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentDataProviderTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentDataProviderTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentDataProviderTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentDataProviderTriggerDetails[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointmentInvitee_FWD_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentParticipantResponse __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentParticipantResponse;

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

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCancelMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarCancelMeetingRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCancelMeetingRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppointmentCalendarLocalId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AppointmentLocalId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AppointmentOriginalStartTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_Subject)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Comment)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_NotifyInvitees)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_get_AppointmentCalendarLocalId(This, value) \
    ((This)->lpVtbl->get_AppointmentCalendarLocalId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_get_AppointmentLocalId(This, value) \
    ((This)->lpVtbl->get_AppointmentLocalId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_get_AppointmentOriginalStartTime(This, value) \
    ((This)->lpVtbl->get_AppointmentOriginalStartTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_get_Subject(This, value) \
    ((This)->lpVtbl->get_Subject(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_get_Comment(This, value) \
    ((This)->lpVtbl->get_Comment(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_get_NotifyInvitees(This, value) \
    ((This)->lpVtbl->get_NotifyInvitees(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCancelMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarCancelMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCancelMeetingRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCancelMeetingRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCreateOrUpdateAppointmentRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarCreateOrUpdateAppointmentRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCreateOrUpdateAppointmentRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppointmentCalendarLocalId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Appointment)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment** value);
    HRESULT (STDMETHODCALLTYPE* get_NotifyInvitees)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ChangedProperties)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CIAppointment* createdOrUpdatedAppointment,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_get_AppointmentCalendarLocalId(This, value) \
    ((This)->lpVtbl->get_AppointmentCalendarLocalId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_get_Appointment(This, value) \
    ((This)->lpVtbl->get_Appointment(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_get_NotifyInvitees(This, value) \
    ((This)->lpVtbl->get_NotifyInvitees(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_get_ChangedProperties(This, value) \
    ((This)->lpVtbl->get_ChangedProperties(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_ReportCompletedAsync(This, createdOrUpdatedAppointment, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, createdOrUpdatedAppointment, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarForwardMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarForwardMeetingRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarForwardMeetingRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppointmentCalendarLocalId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AppointmentLocalId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AppointmentOriginalStartTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_Invitees)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest* This,
        __FIVectorView_1_Windows__CApplicationModel__CAppointments__CAppointmentInvitee** value);
    HRESULT (STDMETHODCALLTYPE* get_Subject)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ForwardHeader)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Comment)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_get_AppointmentCalendarLocalId(This, value) \
    ((This)->lpVtbl->get_AppointmentCalendarLocalId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_get_AppointmentLocalId(This, value) \
    ((This)->lpVtbl->get_AppointmentLocalId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_get_AppointmentOriginalStartTime(This, value) \
    ((This)->lpVtbl->get_AppointmentOriginalStartTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_get_Invitees(This, value) \
    ((This)->lpVtbl->get_Invitees(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_get_Subject(This, value) \
    ((This)->lpVtbl->get_Subject(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_get_ForwardHeader(This, value) \
    ((This)->lpVtbl->get_ForwardHeader(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_get_Comment(This, value) \
    ((This)->lpVtbl->get_Comment(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarForwardMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarForwardMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarForwardMeetingRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarForwardMeetingRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarProposeNewTimeForMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarProposeNewTimeForMeetingRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarProposeNewTimeForMeetingRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppointmentCalendarLocalId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AppointmentLocalId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AppointmentOriginalStartTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_NewStartTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_NewDuration)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Subject)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Comment)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_get_AppointmentCalendarLocalId(This, value) \
    ((This)->lpVtbl->get_AppointmentCalendarLocalId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_get_AppointmentLocalId(This, value) \
    ((This)->lpVtbl->get_AppointmentLocalId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_get_AppointmentOriginalStartTime(This, value) \
    ((This)->lpVtbl->get_AppointmentOriginalStartTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_get_NewStartTime(This, value) \
    ((This)->lpVtbl->get_NewStartTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_get_NewDuration(This, value) \
    ((This)->lpVtbl->get_NewDuration(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_get_Subject(This, value) \
    ((This)->lpVtbl->get_Subject(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_get_Comment(This, value) \
    ((This)->lpVtbl->get_Comment(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarSyncManagerSyncRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarSyncManagerSyncRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarSyncManagerSyncRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppointmentCalendarLocalId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_get_AppointmentCalendarLocalId(This, value) \
    ((This)->lpVtbl->get_AppointmentCalendarLocalId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarSyncManagerSyncRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarSyncManagerSyncRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarSyncManagerSyncRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarUpdateMeetingResponseRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarUpdateMeetingResponseRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarUpdateMeetingResponseRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AppointmentCalendarLocalId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AppointmentLocalId)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AppointmentOriginalStartTime)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_Response)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppointments_CAppointmentParticipantResponse* response);
    HRESULT (STDMETHODCALLTYPE* get_Subject)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Comment)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SendUpdate)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_get_AppointmentCalendarLocalId(This, value) \
    ((This)->lpVtbl->get_AppointmentCalendarLocalId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_get_AppointmentLocalId(This, value) \
    ((This)->lpVtbl->get_AppointmentLocalId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_get_AppointmentOriginalStartTime(This, value) \
    ((This)->lpVtbl->get_AppointmentOriginalStartTime(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_get_Response(This, response) \
    ((This)->lpVtbl->get_Response(This, response))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_get_Subject(This, value) \
    ((This)->lpVtbl->get_Subject(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_get_Comment(This, value) \
    ((This)->lpVtbl->get_Comment(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_get_SendUpdate(This, value) \
    ((This)->lpVtbl->get_SendUpdate(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarUpdateMeetingResponseRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentCalendarUpdateMeetingResponseRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarUpdateMeetingResponseRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentCalendarUpdateMeetingResponseRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentDataProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentDataProviderConnection[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentDataProviderConnection";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_SyncRequested)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarSyncManagerSyncRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SyncRequested)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_CreateOrUpdateAppointmentRequested)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CreateOrUpdateAppointmentRequested)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_CancelMeetingRequested)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarCancelMeetingRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CancelMeetingRequested)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ForwardMeetingRequested)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarForwardMeetingRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ForwardMeetingRequested)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ProposeNewTimeForMeetingRequested)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ProposeNewTimeForMeetingRequested)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_UpdateMeetingResponseRequested)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentDataProviderConnection_Windows__CApplicationModel__CAppointments__CDataProvider__CAppointmentCalendarUpdateMeetingResponseRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_UpdateMeetingResponseRequested)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnectionVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_add_SyncRequested(This, handler, token) \
    ((This)->lpVtbl->add_SyncRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_remove_SyncRequested(This, token) \
    ((This)->lpVtbl->remove_SyncRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_add_CreateOrUpdateAppointmentRequested(This, handler, token) \
    ((This)->lpVtbl->add_CreateOrUpdateAppointmentRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_remove_CreateOrUpdateAppointmentRequested(This, token) \
    ((This)->lpVtbl->remove_CreateOrUpdateAppointmentRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_add_CancelMeetingRequested(This, handler, token) \
    ((This)->lpVtbl->add_CancelMeetingRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_remove_CancelMeetingRequested(This, token) \
    ((This)->lpVtbl->remove_CancelMeetingRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_add_ForwardMeetingRequested(This, handler, token) \
    ((This)->lpVtbl->add_ForwardMeetingRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_remove_ForwardMeetingRequested(This, token) \
    ((This)->lpVtbl->remove_ForwardMeetingRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_add_ProposeNewTimeForMeetingRequested(This, handler, token) \
    ((This)->lpVtbl->add_ProposeNewTimeForMeetingRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_remove_ProposeNewTimeForMeetingRequested(This, token) \
    ((This)->lpVtbl->remove_ProposeNewTimeForMeetingRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_add_UpdateMeetingResponseRequested(This, handler, token) \
    ((This)->lpVtbl->add_UpdateMeetingResponseRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_remove_UpdateMeetingResponseRequested(This, token) \
    ((This)->lpVtbl->remove_UpdateMeetingResponseRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_Start(This) \
    ((This)->lpVtbl->Start(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Appointments.DataProvider.IAppointmentDataProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Appointments_DataProvider_IAppointmentDataProviderTriggerDetails[] = L"Windows.ApplicationModel.Appointments.DataProvider.IAppointmentDataProviderTriggerDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Connection)(__x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails* This,
        __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderConnection** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_get_Connection(This, value) \
    ((This)->lpVtbl->get_Connection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CAppointments_CDataProvider_CIAppointmentDataProviderTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCancelMeetingRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCancelMeetingRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCancelMeetingRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCancelMeetingRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCancelMeetingRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCancelMeetingRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCancelMeetingRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCancelMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCancelMeetingRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCreateOrUpdateAppointmentRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCreateOrUpdateAppointmentRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCreateOrUpdateAppointmentRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCreateOrUpdateAppointmentRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarCreateOrUpdateAppointmentRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarForwardMeetingRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarForwardMeetingRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarForwardMeetingRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarForwardMeetingRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarForwardMeetingRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarForwardMeetingRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarForwardMeetingRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarForwardMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarForwardMeetingRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarProposeNewTimeForMeetingRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarProposeNewTimeForMeetingRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarProposeNewTimeForMeetingRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarProposeNewTimeForMeetingRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarProposeNewTimeForMeetingRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarProposeNewTimeForMeetingRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarSyncManagerSyncRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarSyncManagerSyncRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarSyncManagerSyncRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarSyncManagerSyncRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarSyncManagerSyncRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarSyncManagerSyncRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarSyncManagerSyncRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarSyncManagerSyncRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarSyncManagerSyncRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarUpdateMeetingResponseRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarUpdateMeetingResponseRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarUpdateMeetingResponseRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarUpdateMeetingResponseRequest[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentCalendarUpdateMeetingResponseRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarUpdateMeetingResponseRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarUpdateMeetingResponseRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentCalendarUpdateMeetingResponseRequestEventArgs[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentCalendarUpdateMeetingResponseRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentDataProviderConnection ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentDataProviderConnection_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentDataProviderConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentDataProviderConnection[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Appointments.DataProvider.IAppointmentDataProviderTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentDataProviderTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Appointments_DataProvider_AppointmentDataProviderTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Appointments_DataProvider_AppointmentDataProviderTriggerDetails[] = L"Windows.ApplicationModel.Appointments.DataProvider.AppointmentDataProviderTriggerDetails";
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
#endif // __windows2Eapplicationmodel2Eappointments2Edataprovider_p_h__

#endif // __windows2Eapplicationmodel2Eappointments2Edataprovider_h__
