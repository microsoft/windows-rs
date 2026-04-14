
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
#ifndef __windows2Eapplicationmodel2Ewallet2Esystem_h__
#define __windows2Eapplicationmodel2Ewallet2Esystem_h__
#ifndef __windows2Eapplicationmodel2Ewallet2Esystem_p_h__
#define __windows2Eapplicationmodel2Ewallet2Esystem_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_APPLICATIONMODEL_WALLET_WALLETCONTRACT_VERSION)

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
#include "Windows.ApplicationModel.Wallet.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                namespace System {
                    interface IWalletItemSystemStore;
                } /* System */
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore ABI::Windows::ApplicationModel::Wallet::System::IWalletItemSystemStore

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                namespace System {
                    interface IWalletItemSystemStore2;
                } /* System */
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2 ABI::Windows::ApplicationModel::Wallet::System::IWalletItemSystemStore2

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                namespace System {
                    interface IWalletManagerSystemStatics;
                } /* System */
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics ABI::Windows::ApplicationModel::Wallet::System::IWalletManagerSystemStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_FWD_DEFINED__

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


namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                namespace System {
                    class WalletItemSystemStore;
                } /* System */
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("587c5f4f-7c55-5f74-b26a-f80e3bc6d4f2"))
IAsyncOperation<ABI::Windows::ApplicationModel::Wallet::System::WalletItemSystemStore*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::System::WalletItemSystemStore*, ABI::Windows::ApplicationModel::Wallet::System::IWalletItemSystemStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Wallet.System.WalletItemSystemStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Wallet::System::WalletItemSystemStore*> __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fe76bd86-3570-5d56-932e-a6fb8093a557"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Wallet::System::WalletItemSystemStore*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::System::WalletItemSystemStore*, ABI::Windows::ApplicationModel::Wallet::System::IWalletItemSystemStore*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Wallet.System.WalletItemSystemStore>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Wallet::System::WalletItemSystemStore*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                class WalletItem;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                interface IWalletItem;
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem ABI::Windows::ApplicationModel::Wallet::IWalletItem

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8e64ddb0-ea5c-5593-a1f3-0b8209df3905"))
IAsyncOperation<ABI::Windows::ApplicationModel::Wallet::WalletItem*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletItem*, ABI::Windows::ApplicationModel::Wallet::IWalletItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.Wallet.WalletItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::Wallet::WalletItem*> __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("88b0349f-503d-5786-a267-55bb37a8a1b1"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Wallet::WalletItem*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletItem*, ABI::Windows::ApplicationModel::Wallet::IWalletItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.Wallet.WalletItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::Wallet::WalletItem*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e3ceb002-c2dd-5e63-913c-d7d577561e73"))
IIterator<ABI::Windows::ApplicationModel::Wallet::WalletItem*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletItem*, ABI::Windows::ApplicationModel::Wallet::IWalletItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Wallet.WalletItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Wallet::WalletItem*> __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_t;
#define __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ac174c8c-0fdd-5cff-a29f-4e8ce1c8bc81"))
IIterable<ABI::Windows::ApplicationModel::Wallet::WalletItem*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletItem*, ABI::Windows::ApplicationModel::Wallet::IWalletItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Wallet.WalletItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Wallet::WalletItem*> __FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem_t;
#define __FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2dc89188-2b5b-591a-bb3d-d7d57ff7312c"))
IVectorView<ABI::Windows::ApplicationModel::Wallet::WalletItem*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::WalletItem*, ABI::Windows::ApplicationModel::Wallet::IWalletItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Wallet.WalletItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Wallet::WalletItem*> __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_t;
#define __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("23540ddd-166f-5b93-8669-e340b5e1820d"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Wallet.WalletItem>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9302d49e-dda3-5971-b48a-dfdf02c572af"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Wallet.WalletItem>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3c619943-a672-57ee-ad32-f6d97a6f4217"))
ITypedEventHandler<ABI::Windows::ApplicationModel::Wallet::System::WalletItemSystemStore*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Wallet::System::WalletItemSystemStore*, ABI::Windows::ApplicationModel::Wallet::System::IWalletItemSystemStore*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.Wallet.System.WalletItemSystemStore, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::Wallet::System::WalletItemSystemStore*, IInspectable*> __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable_USE */

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
            namespace Wallet {
                namespace System {
                    typedef enum WalletItemAppAssociation : int WalletItemAppAssociation;
                } /* System */
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.Wallet.System.WalletItemAppAssociation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                namespace System {
                    enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("WalletItemAppAssociation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    WalletItemAppAssociation : int
                    {
                        WalletItemAppAssociation_None = 0,
                        WalletItemAppAssociation_AppInstalled = 1,
                        WalletItemAppAssociation_AppNotInstalled = 2,
                    };
                } /* System */
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.System.IWalletItemSystemStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.System.WalletItemSystemStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_System_IWalletItemSystemStore[] = L"Windows.ApplicationModel.Wallet.System.IWalletItemSystemStore";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                namespace System {
                    MIDL_INTERFACE("522e2bff-96a2-4a17-8d19-fe1d9f837561")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    IWalletItemSystemStore : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                        DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                        virtual HRESULT STDMETHODCALLTYPE GetItemsAsync(
                            __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem** operation
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                        DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                        virtual HRESULT STDMETHODCALLTYPE DeleteAsync(
                            ABI::Windows::ApplicationModel::Wallet::IWalletItem* item,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                        DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                        virtual HRESULT STDMETHODCALLTYPE ImportItemAsync(
                            ABI::Windows::Storage::Streams::IRandomAccessStreamReference* stream,
                            __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem** operation
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                        DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                        virtual HRESULT STDMETHODCALLTYPE GetAppStatusForItem(
                            ABI::Windows::ApplicationModel::Wallet::IWalletItem* item,
                            ABI::Windows::ApplicationModel::Wallet::System::WalletItemAppAssociation* result
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                        DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                        virtual HRESULT STDMETHODCALLTYPE LaunchAppForItemAsync(
                            ABI::Windows::ApplicationModel::Wallet::IWalletItem* item,
                            __FIAsyncOperation_1_boolean** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWalletItemSystemStore = __uuidof(IWalletItemSystemStore);
                } /* System */
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.System.IWalletItemSystemStore2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.System.WalletItemSystemStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_System_IWalletItemSystemStore2[] = L"Windows.ApplicationModel.Wallet.System.IWalletItemSystemStore2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                namespace System {
                    MIDL_INTERFACE("f98d3a4e-be00-4fdd-9734-6c113c1ac1cb")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletItemSystemStore2 is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    IWalletItemSystemStore2 : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                        DEPRECATED("IWalletItemSystemStore2 is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                        virtual HRESULT STDMETHODCALLTYPE add_ItemsChanged(
                            __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable* handler,
                            EventRegistrationToken* cookie
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                        DEPRECATED("IWalletItemSystemStore2 is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                        virtual HRESULT STDMETHODCALLTYPE remove_ItemsChanged(
                            EventRegistrationToken cookie
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWalletItemSystemStore2 = __uuidof(IWalletItemSystemStore2);
                } /* System */
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.System.IWalletManagerSystemStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.System.WalletManagerSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_System_IWalletManagerSystemStatics[] = L"Windows.ApplicationModel.Wallet.System.IWalletManagerSystemStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Wallet {
                namespace System {
                    MIDL_INTERFACE("bee8eb89-2634-4b9a-8b23-ee8903c91fe0")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    DEPRECATED("IWalletManagerSystemStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                    IWalletManagerSystemStatics : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                        DEPRECATED("IWalletManagerSystemStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
                        virtual HRESULT STDMETHODCALLTYPE RequestStoreAsync(
                            __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IWalletManagerSystemStatics = __uuidof(IWalletManagerSystemStatics);
                } /* System */
            } /* Wallet */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.System.WalletItemSystemStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Wallet.System.IWalletItemSystemStore ** Default Interface **
 *    Windows.ApplicationModel.Wallet.System.IWalletItemSystemStore2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_System_WalletItemSystemStore_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_System_WalletItemSystemStore_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_System_WalletItemSystemStore[] = L"Windows.ApplicationModel.Wallet.System.WalletItemSystemStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.System.WalletManagerSystem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Wallet.System.IWalletManagerSystemStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_System_WalletManagerSystem_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_System_WalletManagerSystem_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletManagerSystem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_System_WalletManagerSystem[] = L"Windows.ApplicationModel.Wallet.System.WalletManagerSystem";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2 __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStoreVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStoreVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem;

#endif // ____x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem_FWD_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem;

typedef struct __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem __FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem;

typedef struct __FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        __FIIterator_1_Windows__CApplicationModel__CWallet__CWalletItem** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        __FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CWalletItemAppAssociation __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CWalletItemAppAssociation;

/*
 *
 * Struct Windows.ApplicationModel.Wallet.System.WalletItemAppAssociation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletItemAppAssociation is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CWalletItemAppAssociation
{
    WalletItemAppAssociation_None = 0,
    WalletItemAppAssociation_AppInstalled = 1,
    WalletItemAppAssociation_AppNotInstalled = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.System.IWalletItemSystemStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.System.WalletItemSystemStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_System_IWalletItemSystemStore[] = L"Windows.ApplicationModel.Wallet.System.IWalletItemSystemStore";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStoreVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* GetItemsAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CWallet__CWalletItem** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* DeleteAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* item,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* ImportItemAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* stream,
        __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CWalletItem** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* GetAppStatusForItem)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* item,
        enum __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CWalletItemAppAssociation* result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* LaunchAppForItemAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore* This,
        __x_ABI_CWindows_CApplicationModel_CWallet_CIWalletItem* item,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStoreVtbl;

interface __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStoreVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_GetItemsAsync(This, operation) \
    ((This)->lpVtbl->GetItemsAsync(This, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_DeleteAsync(This, item, operation) \
    ((This)->lpVtbl->DeleteAsync(This, item, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_ImportItemAsync(This, stream, operation) \
    ((This)->lpVtbl->ImportItemAsync(This, stream, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_GetAppStatusForItem(This, item, result) \
    ((This)->lpVtbl->GetAppStatusForItem(This, item, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_LaunchAppForItemAsync(This, item, operation) \
    ((This)->lpVtbl->LaunchAppForItemAsync(This, item, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.System.IWalletItemSystemStore2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.System.WalletItemSystemStore
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_System_IWalletItemSystemStore2[] = L"Windows.ApplicationModel.Wallet.System.IWalletItemSystemStore2";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("IWalletItemSystemStore2 is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemSystemStore2 is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* add_ItemsChanged)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore_IInspectable* handler,
        EventRegistrationToken* cookie);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemSystemStore2 is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* remove_ItemsChanged)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2* This,
        EventRegistrationToken cookie);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemSystemStore2 is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_add_ItemsChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_ItemsChanged(This, handler, cookie))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletItemSystemStore2 is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_remove_ItemsChanged(This, cookie) \
    ((This)->lpVtbl->remove_ItemsChanged(This, cookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletItemSystemStore2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.Wallet.System.IWalletManagerSystemStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Wallet.System.WalletManagerSystem
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_Wallet_System_IWalletManagerSystemStatics[] = L"Windows.ApplicationModel.Wallet.System.IWalletManagerSystemStatics";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("IWalletManagerSystemStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletManagerSystemStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    HRESULT (STDMETHODCALLTYPE* RequestStoreAsync)(__x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CWallet__CSystem__CWalletItemSystemStore** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
    DEPRECATED("IWalletManagerSystemStatics is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
#define __x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_RequestStoreAsync(This, operation) \
    ((This)->lpVtbl->RequestStoreAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CWallet_CSystem_CIWalletManagerSystemStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.System.WalletItemSystemStore
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.Wallet.System.IWalletItemSystemStore ** Default Interface **
 *    Windows.ApplicationModel.Wallet.System.IWalletItemSystemStore2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_System_WalletItemSystemStore_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_System_WalletItemSystemStore_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletItemSystemStore is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_System_WalletItemSystemStore[] = L"Windows.ApplicationModel.Wallet.System.WalletItemSystemStore";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.Wallet.System.WalletManagerSystem
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.Wallet.System.IWalletManagerSystemStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Wallet_System_WalletManagerSystem_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Wallet_System_WalletManagerSystem_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
DEPRECATED("WalletManagerSystem is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xe0000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Wallet_System_WalletManagerSystem[] = L"Windows.ApplicationModel.Wallet.System.WalletManagerSystem";
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
#endif // __windows2Eapplicationmodel2Ewallet2Esystem_p_h__

#endif // __windows2Eapplicationmodel2Ewallet2Esystem_h__
