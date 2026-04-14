
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
#ifndef __windows2Eapplicationmodel2Econtacts2Edataprovider_h__
#define __windows2Eapplicationmodel2Econtacts2Edataprovider_h__
#ifndef __windows2Eapplicationmodel2Econtacts2Edataprovider_p_h__
#define __windows2Eapplicationmodel2Econtacts2Edataprovider_p_h__


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
#include "Windows.ApplicationModel.Contacts.h"

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    interface IContactDataProviderConnection;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactDataProviderConnection

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    interface IContactDataProviderConnection2;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2 ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactDataProviderConnection2

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    interface IContactDataProviderTriggerDetails;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactDataProviderTriggerDetails

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    interface IContactListCreateOrUpdateContactRequest;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactListCreateOrUpdateContactRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    interface IContactListCreateOrUpdateContactRequestEventArgs;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactListCreateOrUpdateContactRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    interface IContactListDeleteContactRequest;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactListDeleteContactRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    interface IContactListDeleteContactRequestEventArgs;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactListDeleteContactRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    interface IContactListServerSearchReadBatchRequest;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactListServerSearchReadBatchRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    interface IContactListServerSearchReadBatchRequestEventArgs;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactListServerSearchReadBatchRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    interface IContactListSyncManagerSyncRequest;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactListSyncManagerSyncRequest

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    interface IContactListSyncManagerSyncRequestEventArgs;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactListSyncManagerSyncRequestEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    class ContactDataProviderConnection;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    class ContactListCreateOrUpdateContactRequestEventArgs;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9b90aab6-7ba3-5169-b73c-7e6413d2bd57"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactDataProviderConnection*, ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactListCreateOrUpdateContactRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactDataProviderConnection*, ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactListCreateOrUpdateContactRequestEventArgs*, ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactListCreateOrUpdateContactRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection, Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactDataProviderConnection*, ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactListCreateOrUpdateContactRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    class ContactListDeleteContactRequestEventArgs;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9ff3c767-b488-53e2-a494-32706161ca01"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactDataProviderConnection*, ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactListDeleteContactRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactDataProviderConnection*, ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactListDeleteContactRequestEventArgs*, ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactListDeleteContactRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection, Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactDataProviderConnection*, ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactListDeleteContactRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    class ContactListServerSearchReadBatchRequestEventArgs;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("baee1b2f-a5b6-5a03-ae59-fb18f3e025b7"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactDataProviderConnection*, ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactListServerSearchReadBatchRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactDataProviderConnection*, ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactListServerSearchReadBatchRequestEventArgs*, ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactListServerSearchReadBatchRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection, Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactDataProviderConnection*, ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactListServerSearchReadBatchRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    class ContactListSyncManagerSyncRequestEventArgs;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bb9f410f-a739-5280-9bb7-b6a938c7a620"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactDataProviderConnection*, ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactListSyncManagerSyncRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactDataProviderConnection*, ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactDataProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactListSyncManagerSyncRequestEventArgs*, ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactListSyncManagerSyncRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection, Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactDataProviderConnection*, ABI::Windows::ApplicationModel::Contacts::DataProvider::ContactListSyncManagerSyncRequestEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

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
                typedef enum ContactBatchStatus : int ContactBatchStatus;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                class ContactQueryOptions;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactQueryOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactQueryOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                interface IContactQueryOptions;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CIContactQueryOptions ABI::Windows::ApplicationModel::Contacts::IContactQueryOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactQueryOptions_FWD_DEFINED__

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
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    class ContactListCreateOrUpdateContactRequest;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    class ContactListDeleteContactRequest;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    class ContactListServerSearchReadBatchRequest;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    class ContactListSyncManagerSyncRequest;
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactDataProviderConnection[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderConnection";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    MIDL_INTERFACE("1a398a52-8c9d-4d6f-a4e0-111e9a125a30")
                    IContactDataProviderConnection : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_SyncRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SyncRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ServerSearchReadBatchRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ServerSearchReadBatchRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContactDataProviderConnection = __uuidof(IContactDataProviderConnection);
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderConnection2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactDataProviderConnection2[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderConnection2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    MIDL_INTERFACE("a1d327b0-196c-4bfd-8f0f-c68d67f249d3")
                    IContactDataProviderConnection2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_CreateOrUpdateContactRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_CreateOrUpdateContactRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_DeleteContactRequested(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_DeleteContactRequested(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContactDataProviderConnection2 = __uuidof(IContactDataProviderConnection2);
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactDataProviderTriggerDetails[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    MIDL_INTERFACE("527104be-3c62-43c8-9ae7-db531685cd99")
                    IContactDataProviderTriggerDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Connection(
                            ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactDataProviderConnection** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContactDataProviderTriggerDetails = __uuidof(IContactDataProviderTriggerDetails);
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactListCreateOrUpdateContactRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactListCreateOrUpdateContactRequest[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactListCreateOrUpdateContactRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    MIDL_INTERFACE("b4af411f-c849-47d0-b119-91cf605b2f2a")
                    IContactListCreateOrUpdateContactRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ContactListId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Contact(
                            ABI::Windows::ApplicationModel::Contacts::IContact** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::ApplicationModel::Contacts::IContact* createdOrUpdatedContact,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContactListCreateOrUpdateContactRequest = __uuidof(IContactListCreateOrUpdateContactRequest);
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactListCreateOrUpdateContactRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactListCreateOrUpdateContactRequestEventArgs[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactListCreateOrUpdateContactRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    MIDL_INTERFACE("851c1690-1a51-4b0c-aeef-1240ac5bed75")
                    IContactListCreateOrUpdateContactRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactListCreateOrUpdateContactRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContactListCreateOrUpdateContactRequestEventArgs = __uuidof(IContactListCreateOrUpdateContactRequestEventArgs);
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactListDeleteContactRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactListDeleteContactRequest[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactListDeleteContactRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    MIDL_INTERFACE("5e114687-ce03-4de5-8557-9ccf552d472a")
                    IContactListDeleteContactRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ContactListId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContactId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContactListDeleteContactRequest = __uuidof(IContactListDeleteContactRequest);
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactListDeleteContactRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactListDeleteContactRequestEventArgs[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactListDeleteContactRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    MIDL_INTERFACE("b22054a1-e8fa-4db5-9389-2d12ee7d15ee")
                    IContactListDeleteContactRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactListDeleteContactRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContactListDeleteContactRequestEventArgs = __uuidof(IContactListDeleteContactRequestEventArgs);
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactListServerSearchReadBatchRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactListServerSearchReadBatchRequest[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactListServerSearchReadBatchRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    MIDL_INTERFACE("ba776a97-4030-4925-9fb4-143b295e653b")
                    IContactListServerSearchReadBatchRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SessionId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContactListId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Options(
                            ABI::Windows::ApplicationModel::Contacts::IContactQueryOptions** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SuggestedBatchSize(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SaveContactAsync(
                            ABI::Windows::ApplicationModel::Contacts::IContact* contact,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::ApplicationModel::Contacts::ContactBatchStatus batchStatus,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContactListServerSearchReadBatchRequest = __uuidof(IContactListServerSearchReadBatchRequest);
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactListServerSearchReadBatchRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactListServerSearchReadBatchRequestEventArgs[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactListServerSearchReadBatchRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    MIDL_INTERFACE("1a27e87b-69d7-4e4e-8042-861cba61471e")
                    IContactListServerSearchReadBatchRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactListServerSearchReadBatchRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContactListServerSearchReadBatchRequestEventArgs = __uuidof(IContactListServerSearchReadBatchRequestEventArgs);
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactListSyncManagerSyncRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactListSyncManagerSyncRequest[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactListSyncManagerSyncRequest";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    MIDL_INTERFACE("3c0e57a4-c4e7-4970-9a8f-9a66a2bb6c1a")
                    IContactListSyncManagerSyncRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ContactListId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContactListSyncManagerSyncRequest = __uuidof(IContactListSyncManagerSyncRequest);
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactListSyncManagerSyncRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactListSyncManagerSyncRequestEventArgs[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactListSyncManagerSyncRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace DataProvider {
                    MIDL_INTERFACE("158e4dac-446d-4f10-afc2-02683ec533a6")
                    IContactListSyncManagerSyncRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::ApplicationModel::Contacts::DataProvider::IContactListSyncManagerSyncRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContactListSyncManagerSyncRequestEventArgs = __uuidof(IContactListSyncManagerSyncRequestEventArgs);
                } /* DataProvider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderConnection ** Default Interface **
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderConnection2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactDataProviderConnection_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactDataProviderConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactDataProviderConnection[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactDataProviderTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactDataProviderTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactDataProviderTriggerDetails[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactListCreateOrUpdateContactRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListCreateOrUpdateContactRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListCreateOrUpdateContactRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactListCreateOrUpdateContactRequest[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactListCreateOrUpdateContactRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListCreateOrUpdateContactRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListCreateOrUpdateContactRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactListCreateOrUpdateContactRequestEventArgs[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactListDeleteContactRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListDeleteContactRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListDeleteContactRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactListDeleteContactRequest[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactListDeleteContactRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListDeleteContactRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListDeleteContactRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactListDeleteContactRequestEventArgs[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactListServerSearchReadBatchRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListServerSearchReadBatchRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListServerSearchReadBatchRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactListServerSearchReadBatchRequest[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactListServerSearchReadBatchRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListServerSearchReadBatchRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListServerSearchReadBatchRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactListServerSearchReadBatchRequestEventArgs[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactListSyncManagerSyncRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListSyncManagerSyncRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListSyncManagerSyncRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactListSyncManagerSyncRequest[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactListSyncManagerSyncRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListSyncManagerSyncRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListSyncManagerSyncRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactListSyncManagerSyncRequestEventArgs[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2 __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection* sender,
        __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CIContact __x_ABI_CWindows_CApplicationModel_CContacts_CIContact;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactBatchStatus __x_ABI_CWindows_CApplicationModel_CContacts_CContactBatchStatus;

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactQueryOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactQueryOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CIContactQueryOptions __x_ABI_CWindows_CApplicationModel_CContacts_CIContactQueryOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContactQueryOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactDataProviderConnection[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderConnection";
typedef struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_SyncRequested)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListSyncManagerSyncRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SyncRequested)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ServerSearchReadBatchRequested)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListServerSearchReadBatchRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ServerSearchReadBatchRequested)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnectionVtbl;

interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_add_SyncRequested(This, handler, token) \
    ((This)->lpVtbl->add_SyncRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_remove_SyncRequested(This, token) \
    ((This)->lpVtbl->remove_SyncRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_add_ServerSearchReadBatchRequested(This, handler, token) \
    ((This)->lpVtbl->add_ServerSearchReadBatchRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_remove_ServerSearchReadBatchRequested(This, token) \
    ((This)->lpVtbl->remove_ServerSearchReadBatchRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_Start(This) \
    ((This)->lpVtbl->Start(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderConnection2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactDataProviderConnection2[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderConnection2";
typedef struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_CreateOrUpdateContactRequested)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListCreateOrUpdateContactRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CreateOrUpdateContactRequested)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DeleteContactRequested)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CDataProvider__CContactDataProviderConnection_Windows__CApplicationModel__CContacts__CDataProvider__CContactListDeleteContactRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DeleteContactRequested)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_add_CreateOrUpdateContactRequested(This, handler, token) \
    ((This)->lpVtbl->add_CreateOrUpdateContactRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_remove_CreateOrUpdateContactRequested(This, token) \
    ((This)->lpVtbl->remove_CreateOrUpdateContactRequested(This, token))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_add_DeleteContactRequested(This, handler, token) \
    ((This)->lpVtbl->add_DeleteContactRequested(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_remove_DeleteContactRequested(This, token) \
    ((This)->lpVtbl->remove_DeleteContactRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactDataProviderTriggerDetails[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderTriggerDetails";
typedef struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Connection)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderConnection** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetailsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_get_Connection(This, value) \
    ((This)->lpVtbl->get_Connection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactDataProviderTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactListCreateOrUpdateContactRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactListCreateOrUpdateContactRequest[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactListCreateOrUpdateContactRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContactListId)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Contact)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContact** value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContact* createdOrUpdatedContact,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_get_ContactListId(This, value) \
    ((This)->lpVtbl->get_ContactListId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_get_Contact(This, value) \
    ((This)->lpVtbl->get_Contact(This, value))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_ReportCompletedAsync(This, createdOrUpdatedContact, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, createdOrUpdatedContact, result))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactListCreateOrUpdateContactRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactListCreateOrUpdateContactRequestEventArgs[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactListCreateOrUpdateContactRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListCreateOrUpdateContactRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactListDeleteContactRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactListDeleteContactRequest[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactListDeleteContactRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContactListId)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ContactId)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_get_ContactListId(This, value) \
    ((This)->lpVtbl->get_ContactListId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_get_ContactId(This, value) \
    ((This)->lpVtbl->get_ContactId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactListDeleteContactRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactListDeleteContactRequestEventArgs[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactListDeleteContactRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListDeleteContactRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactListServerSearchReadBatchRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactListServerSearchReadBatchRequest[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactListServerSearchReadBatchRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SessionId)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ContactListId)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Options)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContactQueryOptions** value);
    HRESULT (STDMETHODCALLTYPE* get_SuggestedBatchSize)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* SaveContactAsync)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContact* contact,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest* This,
        enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactBatchStatus batchStatus,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_get_SessionId(This, value) \
    ((This)->lpVtbl->get_SessionId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_get_ContactListId(This, value) \
    ((This)->lpVtbl->get_ContactListId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_get_Options(This, value) \
    ((This)->lpVtbl->get_Options(This, value))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_get_SuggestedBatchSize(This, value) \
    ((This)->lpVtbl->get_SuggestedBatchSize(This, value))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_SaveContactAsync(This, contact, result) \
    ((This)->lpVtbl->SaveContactAsync(This, contact, result))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_ReportFailedAsync(This, batchStatus, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, batchStatus, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactListServerSearchReadBatchRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactListServerSearchReadBatchRequestEventArgs[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactListServerSearchReadBatchRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListServerSearchReadBatchRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactListSyncManagerSyncRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactListSyncManagerSyncRequest[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactListSyncManagerSyncRequest";
typedef struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContactListId)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestVtbl;

interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_get_ContactListId(This, value) \
    ((This)->lpVtbl->get_ContactListId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.DataProvider.IContactListSyncManagerSyncRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_DataProvider_IContactListSyncManagerSyncRequestEventArgs[] = L"Windows.ApplicationModel.Contacts.DataProvider.IContactListSyncManagerSyncRequestEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CDataProvider_CIContactListSyncManagerSyncRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderConnection ** Default Interface **
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderConnection2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactDataProviderConnection_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactDataProviderConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactDataProviderConnection[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactDataProviderTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactDataProviderTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactDataProviderTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactDataProviderTriggerDetails[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactDataProviderTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactListCreateOrUpdateContactRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListCreateOrUpdateContactRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListCreateOrUpdateContactRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactListCreateOrUpdateContactRequest[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactListCreateOrUpdateContactRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListCreateOrUpdateContactRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListCreateOrUpdateContactRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactListCreateOrUpdateContactRequestEventArgs[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactListCreateOrUpdateContactRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactListDeleteContactRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListDeleteContactRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListDeleteContactRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactListDeleteContactRequest[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactListDeleteContactRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListDeleteContactRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListDeleteContactRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactListDeleteContactRequestEventArgs[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactListDeleteContactRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactListServerSearchReadBatchRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListServerSearchReadBatchRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListServerSearchReadBatchRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactListServerSearchReadBatchRequest[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactListServerSearchReadBatchRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListServerSearchReadBatchRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListServerSearchReadBatchRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactListServerSearchReadBatchRequestEventArgs[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactListServerSearchReadBatchRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactListSyncManagerSyncRequest ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListSyncManagerSyncRequest_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListSyncManagerSyncRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactListSyncManagerSyncRequest[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.DataProvider.IContactListSyncManagerSyncRequestEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListSyncManagerSyncRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_DataProvider_ContactListSyncManagerSyncRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_DataProvider_ContactListSyncManagerSyncRequestEventArgs[] = L"Windows.ApplicationModel.Contacts.DataProvider.ContactListSyncManagerSyncRequestEventArgs";
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
#endif // __windows2Eapplicationmodel2Econtacts2Edataprovider_p_h__

#endif // __windows2Eapplicationmodel2Econtacts2Edataprovider_h__
