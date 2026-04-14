
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
#ifndef __windows2Eapplicationmodel2Euserdataaccounts2Eprovider_h__
#define __windows2Eapplicationmodel2Euserdataaccounts2Eprovider_h__
#ifndef __windows2Eapplicationmodel2Euserdataaccounts2Eprovider_p_h__
#define __windows2Eapplicationmodel2Euserdataaccounts2Eprovider_p_h__


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
#include "Windows.ApplicationModel.UserDataAccounts.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace Provider {
                    interface IUserDataAccountPartnerAccountInfo;
                } /* Provider */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo ABI::Windows::ApplicationModel::UserDataAccounts::Provider::IUserDataAccountPartnerAccountInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace Provider {
                    interface IUserDataAccountProviderAddAccountOperation;
                } /* Provider */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation ABI::Windows::ApplicationModel::UserDataAccounts::Provider::IUserDataAccountProviderAddAccountOperation

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace Provider {
                    interface IUserDataAccountProviderOperation;
                } /* Provider */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation ABI::Windows::ApplicationModel::UserDataAccounts::Provider::IUserDataAccountProviderOperation

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace Provider {
                    interface IUserDataAccountProviderResolveErrorsOperation;
                } /* Provider */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation ABI::Windows::ApplicationModel::UserDataAccounts::Provider::IUserDataAccountProviderResolveErrorsOperation

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace Provider {
                    interface IUserDataAccountProviderSettingsOperation;
                } /* Provider */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation ABI::Windows::ApplicationModel::UserDataAccounts::Provider::IUserDataAccountProviderSettingsOperation

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace Provider {
                    class UserDataAccountPartnerAccountInfo;
                } /* Provider */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ac401b26-3ebf-5cbf-9643-c96a40ab40a2"))
IIterator<ABI::Windows::ApplicationModel::UserDataAccounts::Provider::UserDataAccountPartnerAccountInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataAccounts::Provider::UserDataAccountPartnerAccountInfo*, ABI::Windows::ApplicationModel::UserDataAccounts::Provider::IUserDataAccountPartnerAccountInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::UserDataAccounts::Provider::UserDataAccountPartnerAccountInfo*> __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_t;
#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5cdb425e-da5a-55fa-b349-5467996cab32"))
IIterable<ABI::Windows::ApplicationModel::UserDataAccounts::Provider::UserDataAccountPartnerAccountInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataAccounts::Provider::UserDataAccountPartnerAccountInfo*, ABI::Windows::ApplicationModel::UserDataAccounts::Provider::IUserDataAccountPartnerAccountInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::UserDataAccounts::Provider::UserDataAccountPartnerAccountInfo*> __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_t;
#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b730f093-e2fb-5b20-9d9e-4f9defe647b0"))
IVectorView<ABI::Windows::ApplicationModel::UserDataAccounts::Provider::UserDataAccountPartnerAccountInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::UserDataAccounts::Provider::UserDataAccountPartnerAccountInfo*, ABI::Windows::ApplicationModel::UserDataAccounts::Provider::IUserDataAccountPartnerAccountInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::UserDataAccounts::Provider::UserDataAccountPartnerAccountInfo*> __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_t;
#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                typedef enum UserDataAccountContentKinds : unsigned int UserDataAccountContentKinds;
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace Provider {
                    typedef enum UserDataAccountProviderOperationKind : int UserDataAccountProviderOperationKind;
                } /* Provider */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace Provider {
                    typedef enum UserDataAccountProviderPartnerAccountKind : int UserDataAccountProviderPartnerAccountKind;
                } /* Provider */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderOperationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace Provider {
                    enum UserDataAccountProviderOperationKind : int
                    {
                        UserDataAccountProviderOperationKind_AddAccount = 0,
                        UserDataAccountProviderOperationKind_Settings = 1,
                        UserDataAccountProviderOperationKind_ResolveErrors = 2,
                    };
                } /* Provider */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderPartnerAccountKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace Provider {
                    enum UserDataAccountProviderPartnerAccountKind : int
                    {
                        UserDataAccountProviderPartnerAccountKind_Exchange = 0,
                        UserDataAccountProviderPartnerAccountKind_PopOrImap = 1,
                    };
                } /* Provider */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountPartnerAccountInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_Provider_IUserDataAccountPartnerAccountInfo[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountPartnerAccountInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace Provider {
                    MIDL_INTERFACE("5f200037-f6ef-4ec3-8630-012c59c1149f")
                    IUserDataAccountPartnerAccountInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Priority(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AccountKind(
                            ABI::Windows::ApplicationModel::UserDataAccounts::Provider::UserDataAccountProviderPartnerAccountKind* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataAccountPartnerAccountInfo = __uuidof(IUserDataAccountPartnerAccountInfo);
                } /* Provider */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderAddAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_Provider_IUserDataAccountProviderAddAccountOperation[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderAddAccountOperation";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace Provider {
                    MIDL_INTERFACE("b9c72530-3f84-4b5d-8eaa-45e97aa842ed")
                    IUserDataAccountProviderAddAccountOperation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ContentKinds(
                            ABI::Windows::ApplicationModel::UserDataAccounts::UserDataAccountContentKinds* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PartnerAccountInfos(
                            __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompleted(
                            HSTRING userDataAccountId
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataAccountProviderAddAccountOperation = __uuidof(IUserDataAccountProviderAddAccountOperation);
                } /* Provider */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_Provider_IUserDataAccountProviderOperation[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace Provider {
                    MIDL_INTERFACE("a20aad63-888c-4a62-a3dd-34d07a802b2b")
                    IUserDataAccountProviderOperation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Kind(
                            ABI::Windows::ApplicationModel::UserDataAccounts::Provider::UserDataAccountProviderOperationKind* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataAccountProviderOperation = __uuidof(IUserDataAccountProviderOperation);
                } /* Provider */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderResolveErrorsOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_Provider_IUserDataAccountProviderResolveErrorsOperation[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderResolveErrorsOperation";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace Provider {
                    MIDL_INTERFACE("6235dc15-bfcb-41e1-9957-9759a28846cc")
                    IUserDataAccountProviderResolveErrorsOperation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_UserDataAccountId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompleted(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataAccountProviderResolveErrorsOperation = __uuidof(IUserDataAccountProviderResolveErrorsOperation);
                } /* Provider */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderSettingsOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_Provider_IUserDataAccountProviderSettingsOperation[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderSettingsOperation";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace UserDataAccounts {
                namespace Provider {
                    MIDL_INTERFACE("92034db7-8648-4f30-acfa-3002658ca80d")
                    IUserDataAccountProviderSettingsOperation : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_UserDataAccountId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompleted(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IUserDataAccountProviderSettingsOperation = __uuidof(IUserDataAccountProviderSettingsOperation);
                } /* Provider */
            } /* UserDataAccounts */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountPartnerAccountInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountPartnerAccountInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountPartnerAccountInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountPartnerAccountInfo[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderAddAccountOperation ** Default Interface **
 *    Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderAddAccountOperation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderAddAccountOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderAddAccountOperation[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderResolveErrorsOperation ** Default Interface **
 *    Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderResolveErrorsOperation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderResolveErrorsOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderResolveErrorsOperation[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderSettingsOperation ** Default Interface **
 *    Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderSettingsOperation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderSettingsOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderSettingsOperation[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation;

#endif // ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo;

typedef struct __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfoVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo;

typedef struct __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        __FIIterator_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfoVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfoVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountContentKinds __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountContentKinds;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CUserDataAccountProviderOperationKind __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CUserDataAccountProviderOperationKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CUserDataAccountProviderPartnerAccountKind __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CUserDataAccountProviderPartnerAccountKind;

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderOperationKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CUserDataAccountProviderOperationKind
{
    UserDataAccountProviderOperationKind_AddAccount = 0,
    UserDataAccountProviderOperationKind_Settings = 1,
    UserDataAccountProviderOperationKind_ResolveErrors = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderPartnerAccountKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CUserDataAccountProviderPartnerAccountKind
{
    UserDataAccountProviderPartnerAccountKind_Exchange = 0,
    UserDataAccountProviderPartnerAccountKind_PopOrImap = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountPartnerAccountInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_Provider_IUserDataAccountPartnerAccountInfo[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountPartnerAccountInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Priority)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_AccountKind)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CUserDataAccountProviderPartnerAccountKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_get_Priority(This, value) \
    ((This)->lpVtbl->get_Priority(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_get_AccountKind(This, value) \
    ((This)->lpVtbl->get_AccountKind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountPartnerAccountInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderAddAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_Provider_IUserDataAccountProviderAddAccountOperation[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderAddAccountOperation";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContentKinds)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CUserDataAccountContentKinds* value);
    HRESULT (STDMETHODCALLTYPE* get_PartnerAccountInfos)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation* This,
        __FIVectorView_1_Windows__CApplicationModel__CUserDataAccounts__CProvider__CUserDataAccountPartnerAccountInfo** value);
    HRESULT (STDMETHODCALLTYPE* ReportCompleted)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation* This,
        HSTRING userDataAccountId);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_get_ContentKinds(This, value) \
    ((This)->lpVtbl->get_ContentKinds(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_get_PartnerAccountInfos(This, value) \
    ((This)->lpVtbl->get_PartnerAccountInfos(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_ReportCompleted(This, userDataAccountId) \
    ((This)->lpVtbl->ReportCompleted(This, userDataAccountId))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderAddAccountOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_Provider_IUserDataAccountProviderOperation[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation* This,
        enum __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CUserDataAccountProviderOperationKind* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderResolveErrorsOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_Provider_IUserDataAccountProviderResolveErrorsOperation[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderResolveErrorsOperation";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UserDataAccountId)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompleted)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_get_UserDataAccountId(This, value) \
    ((This)->lpVtbl->get_UserDataAccountId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_ReportCompleted(This) \
    ((This)->lpVtbl->ReportCompleted(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderResolveErrorsOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderSettingsOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_UserDataAccounts_Provider_IUserDataAccountProviderSettingsOperation[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderSettingsOperation";
typedef struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UserDataAccountId)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompleted)(__x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_get_UserDataAccountId(This, value) \
    ((This)->lpVtbl->get_UserDataAccountId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_ReportCompleted(This) \
    ((This)->lpVtbl->ReportCompleted(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CUserDataAccounts_CProvider_CIUserDataAccountProviderSettingsOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountPartnerAccountInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountPartnerAccountInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountPartnerAccountInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountPartnerAccountInfo[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountPartnerAccountInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderAddAccountOperation ** Default Interface **
 *    Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderAddAccountOperation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderAddAccountOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderAddAccountOperation[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderAddAccountOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderResolveErrorsOperation ** Default Interface **
 *    Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderResolveErrorsOperation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderResolveErrorsOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderResolveErrorsOperation[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderResolveErrorsOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderSettingsOperation ** Default Interface **
 *    Windows.ApplicationModel.UserDataAccounts.Provider.IUserDataAccountProviderOperation
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderSettingsOperation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderSettingsOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_UserDataAccounts_Provider_UserDataAccountProviderSettingsOperation[] = L"Windows.ApplicationModel.UserDataAccounts.Provider.UserDataAccountProviderSettingsOperation";
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
#endif // __windows2Eapplicationmodel2Euserdataaccounts2Eprovider_p_h__

#endif // __windows2Eapplicationmodel2Euserdataaccounts2Eprovider_h__
