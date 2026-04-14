
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
#ifndef __windows2Eapplicationmodel2Econtacts2Eprovider_h__
#define __windows2Eapplicationmodel2Econtacts2Eprovider_h__
#ifndef __windows2Eapplicationmodel2Econtacts2Eprovider_p_h__
#define __windows2Eapplicationmodel2Econtacts2Eprovider_p_h__


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
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace Provider {
                    interface IContactPickerUI;
                } /* Provider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI ABI::Windows::ApplicationModel::Contacts::Provider::IContactPickerUI

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace Provider {
                    interface IContactPickerUI2;
                } /* Provider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2 ABI::Windows::ApplicationModel::Contacts::Provider::IContactPickerUI2

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace Provider {
                    interface IContactRemovedEventArgs;
                } /* Provider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs ABI::Windows::ApplicationModel::Contacts::Provider::IContactRemovedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_FWD_DEFINED__

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
            namespace Contacts {
                typedef enum ContactFieldType : int ContactFieldType;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2f6d856a-50d4-5173-abea-db6c6b8fc530"))
IIterator<enum ABI::Windows::ApplicationModel::Contacts::ContactFieldType> : IIterator_impl<enum ABI::Windows::ApplicationModel::Contacts::ContactFieldType>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Contacts.ContactFieldType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::ApplicationModel::Contacts::ContactFieldType> __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_t;
#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("384b8b1b-ce8e-5781-b3dc-0776d684f658"))
IIterable<enum ABI::Windows::ApplicationModel::Contacts::ContactFieldType> : IIterable_impl<enum ABI::Windows::ApplicationModel::Contacts::ContactFieldType>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Contacts.ContactFieldType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::ApplicationModel::Contacts::ContactFieldType> __FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType_t;
#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType_USE */

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

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("39d6abda-a00a-5777-8611-82d7c326c18d"))
IVectorView<enum ABI::Windows::ApplicationModel::Contacts::ContactFieldType> : IVectorView_impl<enum ABI::Windows::ApplicationModel::Contacts::ContactFieldType>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Contacts.ContactFieldType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::ApplicationModel::Contacts::ContactFieldType> __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_t;
#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_USE
#define DEF___FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6fdc2115-1649-54a4-8faa-3049cefb05a4"))
IVector<enum ABI::Windows::ApplicationModel::Contacts::ContactFieldType> : IVector_impl<enum ABI::Windows::ApplicationModel::Contacts::ContactFieldType>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.ApplicationModel.Contacts.ContactFieldType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<enum ABI::Windows::ApplicationModel::Contacts::ContactFieldType> __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_t;
#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace Provider {
                    class ContactPickerUI;
                } /* Provider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace Provider {
                    class ContactRemovedEventArgs;
                } /* Provider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a39aeb7e-765c-5e83-b231-84bead98e9a0"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Contacts::Provider::ContactPickerUI*, ABI::Windows::ApplicationModel::Contacts::Provider::ContactRemovedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::Provider::ContactPickerUI*, ABI::Windows::ApplicationModel::Contacts::Provider::IContactPickerUI*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Contacts::Provider::ContactRemovedEventArgs*, ABI::Windows::ApplicationModel::Contacts::Provider::IContactRemovedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Contacts.Provider.ContactPickerUI, Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Contacts::Provider::ContactPickerUI*, ABI::Windows::ApplicationModel::Contacts::Provider::ContactRemovedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
                typedef enum ContactSelectionMode : int ContactSelectionMode;
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace Provider {
                    typedef enum AddContactResult : int AddContactResult;
                } /* Provider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Contacts.Provider.AddContactResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace Provider {
                    enum AddContactResult : int
                    {
                        AddContactResult_Added = 0,
                        AddContactResult_AlreadyAdded = 1,
                        AddContactResult_Unavailable = 2,
                    };
                } /* Provider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.Provider.IContactPickerUI
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.Provider.ContactPickerUI
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_Provider_IContactPickerUI[] = L"Windows.ApplicationModel.Contacts.Provider.IContactPickerUI";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace Provider {
                    MIDL_INTERFACE("e2cc1366-cf66-43c4-a96a-a5a112db4746")
                    IContactPickerUI : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                        DEPRECATED("AddContact may be altered or unavailable for releases after Windows 8.1. Instead, use AddContact without the ID.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                        virtual HRESULT STDMETHODCALLTYPE AddContact(
                            HSTRING id,
                            ABI::Windows::ApplicationModel::Contacts::IContact* contact,
                            ABI::Windows::ApplicationModel::Contacts::Provider::AddContactResult* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE RemoveContact(
                            HSTRING id
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ContainsContact(
                            HSTRING id,
                            boolean* isContained
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                        DEPRECATED("DesiredFields may be altered or unavailable for releases after Windows 8.1. Instead, use DesiredFieldsWithContactFieldType.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                        virtual HRESULT STDMETHODCALLTYPE get_DesiredFields(
                            __FIVectorView_1_HSTRING** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SelectionMode(
                            ABI::Windows::ApplicationModel::Contacts::ContactSelectionMode* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_ContactRemoved(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_ContactRemoved(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContactPickerUI = __uuidof(IContactPickerUI);
                } /* Provider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.Provider.IContactPickerUI2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.Provider.ContactPickerUI
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_Provider_IContactPickerUI2[] = L"Windows.ApplicationModel.Contacts.Provider.IContactPickerUI2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace Provider {
                    MIDL_INTERFACE("6e449e28-7b25-4999-9b0b-875400a1e8c8")
                    IContactPickerUI2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE AddContact(
                            ABI::Windows::ApplicationModel::Contacts::IContact* contact,
                            ABI::Windows::ApplicationModel::Contacts::Provider::AddContactResult* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DesiredFieldsWithContactFieldType(
                            __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContactPickerUI2 = __uuidof(IContactPickerUI2);
                } /* Provider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.Provider.IContactRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_Provider_IContactRemovedEventArgs[] = L"Windows.ApplicationModel.Contacts.Provider.IContactRemovedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Contacts {
                namespace Provider {
                    MIDL_INTERFACE("6f354338-3302-4d13-ad8d-adcc0ff9e47c")
                    IContactRemovedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IContactRemovedEventArgs = __uuidof(IContactRemovedEventArgs);
                } /* Provider */
            } /* Contacts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Contacts.Provider.ContactPickerUI
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.Provider.IContactPickerUI ** Default Interface **
 *    Windows.ApplicationModel.Contacts.Provider.IContactPickerUI2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_Provider_ContactPickerUI_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_Provider_ContactPickerUI_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_Provider_ContactPickerUI[] = L"Windows.ApplicationModel.Contacts.Provider.ContactPickerUI";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.Provider.IContactRemovedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_Provider_ContactRemovedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_Provider_ContactRemovedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_Provider_ContactRemovedEventArgs[] = L"Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2 __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_FWD_DEFINED__

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

typedef enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactFieldType __x_ABI_CWindows_CApplicationModel_CContacts_CContactFieldType;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType;

typedef struct __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactFieldType* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactFieldType* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldTypeVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType __FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType;

typedef struct __FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        __FIIterator_1_Windows__CApplicationModel__CContacts__CContactFieldType** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldTypeVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CContacts__CContactFieldType_INTERFACE_DEFINED__
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
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        UINT32 index,
        enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactFieldType* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactFieldType value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactFieldType* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldTypeVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType;

typedef struct __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        UINT32 index,
        enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactFieldType* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        __FIVectorView_1_Windows__CApplicationModel__CContacts__CContactFieldType** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactFieldType value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        UINT32 index,
        enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactFieldType value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        UINT32 index,
        enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactFieldType value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactFieldType value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactFieldType* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactFieldType* items);

    END_INTERFACE
} __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldTypeVtbl;

interface __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType
{
    CONST_VTBL struct __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI* sender,
        __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CContacts_CIContact __x_ABI_CWindows_CApplicationModel_CContacts_CIContact;

#endif // ____x_ABI_CWindows_CApplicationModel_CContacts_CIContact_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactSelectionMode __x_ABI_CWindows_CApplicationModel_CContacts_CContactSelectionMode;

typedef enum __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CAddContactResult __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CAddContactResult;

/*
 *
 * Struct Windows.ApplicationModel.Contacts.Provider.AddContactResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CAddContactResult
{
    AddContactResult_Added = 0,
    AddContactResult_AlreadyAdded = 1,
    AddContactResult_Unavailable = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.Provider.IContactPickerUI
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.Provider.ContactPickerUI
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_Provider_IContactPickerUI[] = L"Windows.ApplicationModel.Contacts.Provider.IContactPickerUI";
typedef struct __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUIVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("AddContact may be altered or unavailable for releases after Windows 8.1. Instead, use AddContact without the ID.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* AddContact)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI* This,
        HSTRING id,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContact* contact,
        enum __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CAddContactResult* result);
    HRESULT (STDMETHODCALLTYPE* RemoveContact)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI* This,
        HSTRING id);
    HRESULT (STDMETHODCALLTYPE* ContainsContact)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI* This,
        HSTRING id,
        boolean* isContained);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DesiredFields may be altered or unavailable for releases after Windows 8.1. Instead, use DesiredFieldsWithContactFieldType.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_DesiredFields)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI* This,
        __FIVectorView_1_HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* get_SelectionMode)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI* This,
        enum __x_ABI_CWindows_CApplicationModel_CContacts_CContactSelectionMode* value);
    HRESULT (STDMETHODCALLTYPE* add_ContactRemoved)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CContacts__CProvider__CContactPickerUI_Windows__CApplicationModel__CContacts__CProvider__CContactRemovedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ContactRemoved)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUIVtbl;

interface __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUIVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("AddContact may be altered or unavailable for releases after Windows 8.1. Instead, use AddContact without the ID.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_AddContact(This, id, contact, result) \
    ((This)->lpVtbl->AddContact(This, id, contact, result))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_RemoveContact(This, id) \
    ((This)->lpVtbl->RemoveContact(This, id))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_ContainsContact(This, id, isContained) \
    ((This)->lpVtbl->ContainsContact(This, id, isContained))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("DesiredFields may be altered or unavailable for releases after Windows 8.1. Instead, use DesiredFieldsWithContactFieldType.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_get_DesiredFields(This, value) \
    ((This)->lpVtbl->get_DesiredFields(This, value))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_get_SelectionMode(This, value) \
    ((This)->lpVtbl->get_SelectionMode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_add_ContactRemoved(This, handler, token) \
    ((This)->lpVtbl->add_ContactRemoved(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_remove_ContactRemoved(This, token) \
    ((This)->lpVtbl->remove_ContactRemoved(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.Provider.IContactPickerUI2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.Provider.ContactPickerUI
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_Provider_IContactPickerUI2[] = L"Windows.ApplicationModel.Contacts.Provider.IContactPickerUI2";
typedef struct __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddContact)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2* This,
        __x_ABI_CWindows_CApplicationModel_CContacts_CIContact* contact,
        enum __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CAddContactResult* result);
    HRESULT (STDMETHODCALLTYPE* get_DesiredFieldsWithContactFieldType)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2* This,
        __FIVector_1_Windows__CApplicationModel__CContacts__CContactFieldType** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_AddContact(This, contact, result) \
    ((This)->lpVtbl->AddContact(This, contact, result))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_get_DesiredFieldsWithContactFieldType(This, value) \
    ((This)->lpVtbl->get_DesiredFieldsWithContactFieldType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactPickerUI2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Contacts.Provider.IContactRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Contacts_Provider_IContactRemovedEventArgs[] = L"Windows.ApplicationModel.Contacts.Provider.IContactRemovedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CContacts_CProvider_CIContactRemovedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Contacts.Provider.ContactPickerUI
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.Provider.IContactPickerUI ** Default Interface **
 *    Windows.ApplicationModel.Contacts.Provider.IContactPickerUI2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_Provider_ContactPickerUI_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_Provider_ContactPickerUI_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_Provider_ContactPickerUI[] = L"Windows.ApplicationModel.Contacts.Provider.ContactPickerUI";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Contacts.Provider.IContactRemovedEventArgs ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Contacts_Provider_ContactRemovedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Contacts_Provider_ContactRemovedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Contacts_Provider_ContactRemovedEventArgs[] = L"Windows.ApplicationModel.Contacts.Provider.ContactRemovedEventArgs";
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
#endif // __windows2Eapplicationmodel2Econtacts2Eprovider_p_h__

#endif // __windows2Eapplicationmodel2Econtacts2Eprovider_h__
